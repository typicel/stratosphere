use atrium_api::app::bsky::{embed::images::ViewImage, feed::defs::FeedViewPost};
use dioxus::prelude::*;

#[derive(Props, PartialEq)]
pub struct PostViewProps {
    post: FeedViewPost,
}

pub fn PostView(cx: Scope<PostViewProps>) -> Element {
    use atrium_api::app::bsky::feed::defs::PostViewEmbedEnum;
    use atrium_api::records::Record;
    let post_view = cx.props.post.post.clone();

    let images: Option<Vec<ViewImage>> = match post_view.embed? {
        PostViewEmbedEnum::AppBskyEmbedImagesView(view) => Some(view.images),
        _ => None,
    };

    render! {
        match post_view.record {
            Record::AppBskyFeedPost(post_record) => {
                rsx! {
                    div {
                        display: "flex",
                        flex_direction: "column",
                        div {
                            display: "flex",
                            flex_direction: "row",
                            img {
                                src: "{post_view.author.avatar.unwrap()}",
                                width: "30px",
                                height: "30px",
                                border_radius: "50%",
                            }
                            h4 { "style": "", "{post_view.author.display_name.unwrap()}"}
                            p { "{post_record.text}" }
                        }

                        if !images.is_none() {
                            rsx! {
                                div {
                                    for img in images.unwrap() {
                                        rsx! (
                                            img {
                                                src: "{img.thumb}",
                                                alt: "{img.alt}",
                                                max_width: 1,
                                            }
                                        )
                                    }
                                }
                            }
                        }

                        rsx! {
                            button {
                                onclick: move |_| {


                                }
                            }
                        }
                    }
                }
            }

            _ => {
                rsx! {
                    li { "Unknown post type" }
                }
            }
        }
    }
}
