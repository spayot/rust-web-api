use chrono::offset::Utc;
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Post {
    title: String,
    author: String,
    body: String,
    datetime: DateTime<Utc>,
    uuid: Uuid,
}

impl Post {
    pub fn new(title: &str, author: &str, body: &str, datetime: DateTime<Utc>, uuid: Uuid) -> Post {
        Post {
            title: title.to_string(),
            author: author.to_string(),
            body: body.to_string(),
            datetime,
            uuid,
        }
    }

    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }
}
