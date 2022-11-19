use iced::widget::text_input;
use iced::window;
use iced::subscription;
use iced::alignment;
use iced::{
    Application, Element, Settings, Subscription,
    Command, Length, Color
};
use iced::widget::{scrollable, container, text, column};
use iced::theme::Theme;
use serde::{Deserialize, Serialize};

mod tweet;
mod user;

use crate::tweet::{Tweet, TweetMessage};
use crate::user::User;


pub fn main() -> iced::Result {
    Pwitter::run(Settings {
        window: window::Settings {
            size: (500, 800),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

#[derive(Debug)]
enum Pwitter {
    Loading,
    Loaded(State),
}

#[derive(Debug, Default)]
struct State {
    me: Option<User>,
    input_value: String,
    tweets: Vec<Tweet>
}

#[derive(Debug, Clone)]
enum Message {
    Loaded(Result<SavedState, LoadError>),
    InputChanged(String),
    PostTweet,
    TweetMessage(TweetMessage),
}

impl Application for Pwitter {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Pwitter, Command<Message>) {
        (
            Pwitter::Loading,
            Command::perform(SavedState::load(), Message::Loaded)
        )
    }

    fn title(&self) -> String {
        "Pwitter".to_string()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match self {
            Pwitter::Loading => {
                match message {
                    Message::Loaded(Ok(state)) => {
                        *self = Pwitter::Loaded(State {
                            tweets: state.tweets,
                            me: Some(state.me),
                            ..State::default()
                        });
                    }
                    Message::Loaded(Err(_)) => {
                        *self = Pwitter::Loaded(State::default());
                    }
                    _ => {}
                };
                Command::none()
            }
            Pwitter::Loaded(state) => {
                if let Some(me) = &state.me {
                    match message {
                        Message::InputChanged(value) => {
                            state.input_value = value;
                        }
                        Message::PostTweet => {
                            if !state.input_value.is_empty() {
                                state
                                    .tweets
                                    .push(Tweet::new(
                                        me.addr.to_string(),
                                        state.input_value.clone()
                                    ));
                                state.input_value.clear();
                            }
                        }
                        _ => {}
                    }
                }
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        match self {
            Pwitter::Loading => loading_message(),
            Pwitter::Loaded(state) if state.me.is_none() => nouser_message(),
            Pwitter::Loaded(State { input_value, tweets, me }) => {
                let title = text("Home")
                    .width(Length::Fill)
                    .size(24)
                    .style(Color::from([0.5, 0.5, 0.5]));
                let input = text_input(
                    "What's happening?",
                    input_value,
                    Message::InputChanged)
                    .on_submit(Message::PostTweet);
                let tweets: Element<_> = if tweets.len() > 0 {
                    column(
                        tweets
                            .iter()
                            .map(|tweet| tweet.view().map(move |message| {
                                Message::TweetMessage(message)
                            }))
                            .collect(),
                    )
                    .spacing(10)
                    .into()
                } else {
                    empty_message("There's no tweets yet.")
                };
                let content = column![title, input, tweets].spacing(20).max_width(800);
                scrollable(
                    container(content)
                        .width(Length::Fill)
                        .padding(40)
                        .center_x()
                ).into()
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        subscription::events_with(|event, status| match (event, status) {
            _ => None
        })
    }

}

fn loading_message<'a>() -> Element<'a, Message> {
    container(
        text("Loading...")
            .horizontal_alignment(alignment::Horizontal::Center)
            .size(50),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_y()
    .into()
}

fn nouser_message<'a>() -> Element<'a, Message> {
    container(
        text("No User Info")
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x()
    .center_y()
    .into()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SavedState {
    me: User,
    tweets: Vec<Tweet>
}

#[derive(Debug, Clone)]
enum LoadError {
    TweetsFileNotFound,
    MeNotFound,
    Format(String)
}

impl SavedState {
    fn path_to_tweets() -> std::path::PathBuf {
        let mut path = std::env::current_dir().unwrap_or_default();
        path.push("tweets.json");
        path
    }

    fn path_to_me() -> std::path::PathBuf {
        let mut path = std::env::current_dir().unwrap_or_default();
        path.push("me.json");
        path
    }

    async fn load_me() -> Result<User, LoadError> {
        use async_std::io::ReadExt;
        let mut buffer = String::new();
        let mut file = async_std::fs::File::open(Self::path_to_me())
            .await
            .map_err(|_| LoadError::MeNotFound)?;
        file.read_to_string(&mut buffer)
            .await
            .map_err(|_| LoadError::MeNotFound)?;
        serde_json::from_str(&buffer).map_err(|_| LoadError::Format("me".to_string()))
    }

    async fn load_tweets() -> Result<Vec<Tweet>, LoadError> {
        use async_std::io::ReadExt;
        let mut buffer = String::new();
        let mut file = async_std::fs::File::open(Self::path_to_tweets())
            .await
            .map_err(|_| LoadError::TweetsFileNotFound)?;
        file.read_to_string(&mut buffer)
            .await
            .map_err(|_| LoadError::TweetsFileNotFound)?;
        serde_json::from_str(&buffer).map_err(|_| LoadError::Format("tweets".to_string()))
    }

    async fn load() -> Result<SavedState, LoadError> {
        let me = Self::load_me().await?;
        let tweets = Self::load_tweets().await?;
        Ok (SavedState {me, tweets})
    }
}

fn empty_message(message: &str) -> Element<'_, Message> {
    container(
        text(message)
            .width(Length::Fill)
            .size(25)
            .horizontal_alignment(alignment::Horizontal::Center)
            .style(Color::from([0.7, 0.7, 0.7]))
    )
    .width(Length::Fill)
    .height(Length::Units(200))
    .center_y()
    .into()
}