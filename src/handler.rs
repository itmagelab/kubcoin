use serde::Deserialize;

const CONTENT: &str = include_str!("../static/content.yaml");

#[derive(Debug, Deserialize)]
pub(crate) struct Content {
    pub(crate) chats: Vec<ChatItem>,
    pub(crate) qa: Vec<QAItem>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ChatItem {
    pub(crate) title: String,
    pub(crate) subtitle: String,
    pub(crate) dialogs: Vec<Dialog>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Dialog {
    pub(crate) req: String,
    pub(crate) res: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct QAItem {
    pub(crate) question: String,
    pub(crate) answer: String,
}

pub(crate) fn get_content() -> Content {
    serde_yaml::from_str(CONTENT).unwrap()
}
