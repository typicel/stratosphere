mod bluesky;

use anyhow::Context;
use bluesky::StratosphereApp;

use iced::executor;
use iced::font::Font;
use iced::widget::{self, container};
use iced::{Application, Command, Element, Settings, Theme};

const ICON_FONT: Font = Font::with_name("icons");

#[tokio::main]
pub async fn main() -> iced::Result {
    StratosphereGUI::run(Settings::default())
}

#[derive(Default)]
struct StratosphereGUI {
    stratosphere_app: Option<StratosphereApp>,
}

#[derive(Debug, Clone)]
enum Message {
    LoggedIn(anyhow::Result<StratosphereApp>),
    GetProfile,
    ProfileGot(anyhow::Result<()>),
}

impl Application for StratosphereGUI {
    type Message = Message;
    type Flags = ();
    type Executor = executor::Default;
    type Theme = Theme;

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        let username = std::env::var("BLUESKY_HANDLE")
            .context("BLUESKY_HANDLE not set")
            .unwrap();
        let password = std::env::var("BLUESKY_PASSWORD")
            .context("BLUESKY_PASSWORD not set")
            .unwrap();

        (
            Self::default(),
            Command::perform(
                StratosphereApp::login(username, password),
                Message::LoggedIn,
            ),
        )
    }

    fn title(&self) -> String {
        String::from("Checkbox - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::GetProfile => {
                if let Some(app) = &self.stratosphere_app {
                    return Command::perform(
                        app.get_profile("branchpanic.me".into()),
                        Message::ProfileGot,
                    );
                } else {
                    return Command::none();
                }
            }

            _ => Command::none(),
        }
    }

    fn view(&self) -> Element<Message> {
        let content = button("View Profile").on_press(Message::GetProfile);

        container(content).into()
    }
}

fn button(text: &str) -> widget::Button<'_, Message> {
    widget::button(text).padding(10)
}
