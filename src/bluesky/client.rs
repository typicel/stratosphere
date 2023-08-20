use super::command::*;
use super::record::*;
use super::session::StratosphereSession;
use anyhow::Result;
use async_trait::async_trait;
use atrium_api::app::bsky::feed::defs::FeedViewPost;
use atrium_api::com::atproto::admin::get_record::Parameters;
use atrium_api::com::atproto::server::create_session::Input as CreateSessionInput;
use atrium_api::records::Record;
use atrium_api::{
    app::bsky::actor::get_profile::Parameters as GetProfileParams, client::AtpServiceClient,
};
use atrium_xrpc::client::reqwest::ReqwestClient;
use chrono::Utc;
use reqwest::Client;
use std::sync::{Arc, Mutex};

struct StratosphereXrpc {
    inner: Arc<ReqwestClient>,
    session: Mutex<StratosphereSession>,
}

pub struct StratosphereApp {
    client: Arc<AtpServiceClient<StratosphereXrpc>>,
    xrpc: Arc<StratosphereXrpc>,
}

pub enum ClientResponse {
    Success,
    Timeline(atrium_api::app::bsky::feed::get_timeline::Output),
    Panic,
}

impl StratosphereApp {
    pub async fn login(username: String, password: String) -> Result<Self> {
        let bootstrap =
            AtpServiceClient::new(Arc::new(ReqwestClient::new("https://bsky.social".into())));

        let session = bootstrap
            .com
            .atproto
            .server
            .create_session(CreateSessionInput {
                identifier: username,
                password,
            })
            .await?;

        let xrpc = Arc::new(StratosphereXrpc {
            inner: Arc::new(ReqwestClient::new("https://bsky.social".into())),
            session: Mutex::new(session.try_into()?),
        });

        Ok(Self {
            client: Arc::new(AtpServiceClient::new(xrpc.clone())),
            xrpc,
        })
    }

    pub async fn get_profile(&self, handle: String) -> Result<()> {
        let profile = self
            .client
            .app
            .bsky
            .actor
            .get_profile(GetProfileParams {
                actor: handle.clone(),
            })
            .await?;

        println!("profile: {:?}", profile);

        Ok(())
    }

    pub async fn handle_command(&self, command: Command) -> Result<ClientResponse> {
        match command {
            Command::CreateRecord(record) => {
                use atrium_api::com::atproto::repo::create_record::Input;

                let input = match record {
                    CreateRecordCommand::Post(args) => {
                        use atrium_api::app::bsky::feed::post::{
                            Record as PostRecord, RecordEmbedEnum, ReplyRef,
                        };

                        let text = args.text;

                        Input {
                            collection: String::from("app.bsky.feed.post"),
                            record: Record::AppBskyFeedPost(Box::new(PostRecord {
                                created_at: Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
                                embed: None,
                                entities: None,
                                facets: None,
                                reply: None,
                                text: text,
                                langs: None,
                            })),
                            // get session did from
                            repo: self.xrpc.session.lock().unwrap().did.clone(),
                            rkey: None,
                            swap_commit: None,
                            validate: None,
                        }
                    }
                    CreateRecordCommand::Like(args) => {
                        use atrium_api::app::bsky::feed::like::Record as LikeRecord;
                        // let ru = RecordUri::try_from(args.uri.as_str())?;

                        // call the get_record method from AtpServiceClient
                        let record = self.client.com.atproto.admin.get_record(Parameters {
                            cid: None,
                            uri: args.uri.clone(),
                        }).await;

                        Input {
                            collection: String::from("app.bsky.feed.like"),
                            record: Record::AppBskyFeedLike(Box::new(LikeRecord {
                                created_at: Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
                                subject: atrium_api::com::atproto::repo::strong_ref::Main {
                                    cid: record.unwrap().cid,
                                    uri: args.uri,
                                },
                            })),
                            repo: self.xrpc.session.lock().unwrap().did.clone(),
                            rkey: None,
                            swap_commit: None,
                            validate: None,
                        }
                    }
                };

                let record = self.client.com.atproto.repo.create_record(input).await?;
                println!("Record: {:?}", record);

                Ok(ClientResponse::Success)
            }

            Command::GetTimeline => {
                use atrium_api::app::bsky::feed::get_timeline::Parameters as GetTimelineParams;

                let timeline = self
                    .client
                    .app
                    .bsky
                    .feed
                    .get_timeline(GetTimelineParams {
                        algorithm: None,
                        cursor: None,
                        limit: Some(10),
                    })
                    .await;

                Ok(ClientResponse::Timeline(timeline.unwrap()))
            }

            _ => {
                println!("Command not implemented");
                return Err(anyhow::Error::msg("Command not implemented"));
            }
        }
    }
}

#[async_trait]
impl atrium_xrpc::HttpClient for StratosphereXrpc {
    async fn send(
        &self,
        req: http::Request<Vec<u8>>,
    ) -> Result<http::Response<Vec<u8>>, Box<dyn std::error::Error + Send + Sync>> {
        self.inner.send(req).await
    }
}

#[async_trait]
impl atrium_xrpc::XrpcClient for StratosphereXrpc {
    fn host(&self) -> &str {
        "https://bsky.social"
    }

    fn auth(&self, is_refresh: bool) -> Option<String> {
        self.session.lock().ok().map(|session| {
            if is_refresh {
                session.refresh_jwt.clone()
            } else {
                session.access_jwt.clone()
            }
        })
    }
}
