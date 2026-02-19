use crate::ollama::Response;
use anyhow::bail;
use ratatui::widgets::{ListItem, ListState};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub enum Attitude {
    #[serde(rename = "negative")]
    Negative,
    #[serde(rename = "neutral")]
    Neutral,
    #[serde(rename = "positive")]
    Positive,
}

impl fmt::Display for Attitude {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Attitude::Negative => write!(f, "Negative"),
            Attitude::Neutral => write!(f, "Neutral"),
            Attitude::Positive => write!(f, "Positive"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Dialog {
    pub line: String,
    pub attitude: Attitude,
    #[serde(rename = "affectsRelationship")]
    pub affects_relationship: bool,
    pub options: Option<Vec<DialogOption>>,
}

#[derive(Debug)]
pub struct DialogOptionList {
    pub items: Vec<DialogOption>,
    pub state: ListState,
}

impl From<&DialogOption> for ListItem<'_> {
    fn from(value: &DialogOption) -> Self {
        ListItem::new(value.line.clone())
    }
}

impl TryInto<Dialog> for Response {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Dialog, Self::Error> {
        let parse: Result<Dialog, serde_json::Error> = serde_json::from_str(&self.response);
        match parse {
            Ok(dialog) => return Ok(dialog),
            Err(err) => bail!("{}", err),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DialogOption {
    pub line: String,
    pub tone: Attitude,
}
