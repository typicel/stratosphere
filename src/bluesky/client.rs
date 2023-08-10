use super::session::StratosphereSession;
use anyhow::Result;
use async_trait::async_trait;
use atrium_api::com::atproto::server::create_session::Input as CreateSessionInput;
use atrium_api::{
    app::bsky::actor::get_profile::Parameters as GetProfileParams, client::AtpServiceClient,
};
use atrium_xrpc::client::reqwest::ReqwestClient;
use core::fmt;
use std::fmt::Formatter;
use std::sync::{Arc, Mutex};

struct StratosphereXrpc {
    inner: Arc<ReqwestClient>,
    session: Mutex<StratosphereSession>,
}

pub struct StratosphereApp {
    client: Arc<AtpServiceClient<StratosphereXrpc>>,
    xrpc: Arc<StratosphereXrpc>,
}

impl fmt::Debug for StratosphereApp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StratosphereApp").finish()
    }
}

impl Clone for StratosphereApp {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            xrpc: self.xrpc.clone(),
        }
    }
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
