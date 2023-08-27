use crate::api::{Message, ParseMode};
use crate::BotRequest;
use crate::API;
use serde::Serialize;

#[derive(Debug, Default, Serialize, Clone, BotRequest)]
pub struct SendPhotoRequest {
    /// Unique identifier for the target chat or username of the target
    pub chat_id: i64,

    /// Photo to send. Pass a file_id as String to send a photo that exists
    /// on the Telegram servers (recommended), pass an HTTP URL as a String
    /// for Telegram to get a photo from the Internet, or upload a new photo
    /// using multipart/form-data. The photo must be at most 10 MB in size. 
    /// The photo's width and height must not exceed 10000 in total. 
    /// Width and height ratio must be at most 20.
    pub photo: String,

    /// Photo caption (may also be used when resending photos by file_id),
    /// 0-1024 characters after entities parsing
    pub caption: String,

    /// Mode for parsing entities in the message text. See formatting options for
    /// more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
}

impl SendPhotoRequest {
    pub fn new_file_id(chat_id: i64, file_id: String) -> Self {
        Self {
            chat_id: chat_id,
            photo: file_id,
            ..Default::default()
        }
    }
    pub fn new_external_url(chat_id: i64, url: String) -> Self {
        Self {
            chat_id: chat_id,
            photo: url,
            ..Default::default()
        }
    }
    pub fn with_caption(self, text: impl Into<String>) -> Self {
        Self {
            caption: text.into(),
            ..self
        }
    }
    pub fn with_parse_mode(self, parse_mode: ParseMode) -> Self {
        Self {
            parse_mode: Some(parse_mode),
            ..self
        }
    }
}

impl API {
    /// Use this method to send photos. On success, the sent Message is returned.
    pub async fn send_photo(&self, req: &SendPhotoRequest) -> anyhow::Result<Message> {
        self.client.post("sendPhoto", req).await
    }
}
