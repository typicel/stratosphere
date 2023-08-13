use super::PostView::PostView;
use atrium_api::app::bsky::feed::defs::FeedViewPost;
use dioxus::prelude::*;

#[derive(Props, PartialEq)]
pub struct TimelineViewProps {
    timeline: Vec<FeedViewPost>,
}

pub fn TimelineView(cx: Scope<TimelineViewProps>) -> Element {
    render! {
        div {
            display: "flex",
            justify_content: "center",
            flex_direction: "column",
            border_left: "2px solid black",
            border_right: "2px solid black",
            width: "70%",
            h1 { "Timeline" }
            for post in cx.props.timeline.clone() {
                render! {
                    PostView {
                        post: post
                    }
                }
            }
        }
    }
}
