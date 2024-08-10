use async_trait::async_trait;

use crate::user::Email;
use color_eyre::eyre::Result;

pub struct SendEmail<'a> {
    pub recipient: &'a Email,
    pub subject: &'a str,
    pub content: &'a str,
}

#[async_trait]
pub trait EmailClient {
   async fn send_email(&self, recipient: &Email, subject: &str, content: &str) -> Result<()>;
}