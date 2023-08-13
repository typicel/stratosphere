use atrium_api::app::bsky::feed::defs::FeedViewPost;
use dioxus::prelude::*;

#[derive(Props, PartialEq)]
pub struct PostViewProps {
    post: FeedViewPost,
}

pub fn PostView(cx: Scope<PostViewProps>) -> Element {
    use atrium_api::records::Record;
    let post_view = cx.props.post.post.clone();

    render! {
        match post_view.record {
            Record::AppBskyFeedPost(post_record) => {
                rsx! {
                    div {
                        display: "flex",
                        flex_direction: "row",
                        img {
                            src: "{post_view.author.avatar.unwrap()}",
                            width: "50px",
                            height: "51px",
                            border_radius: "50%",
                        }
                        h4 { "style": "", "{post_view.author.display_name.unwrap()}"}
                        p { "{post_record.text}" }
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
