use iced::{Element, Length};
use iced::widget::{container, text, row, image};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tweet {
    ip_posted_by: String,
    body: String,
    replies: Box<Vec<Tweet>>
}

#[derive(Debug, Clone)]
pub enum TweetMessage {
    ClickReply
}

impl Tweet {
    pub fn new(ip_posted_by: String, body: String) -> Self {
        Tweet {
            ip_posted_by, body, replies: Box::new(vec![])
        }
    }

    pub fn view(&self) -> Element<TweetMessage> {
        let img = image("resources/wolf5.png");
        let icon: Element<_> = container(img)
            .width(Length::Units(48))
            .height(Length::Units(48))
            .into();
        let user_name: Element<_> = text(&self.ip_posted_by).width(Length::Fill).into();
        let tweet_body: Element<_> = text(&self.body).width(Length::Fill).into();
        row![icon, user_name, tweet_body].into()
    }
}