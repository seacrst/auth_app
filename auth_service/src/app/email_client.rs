use async_trait::async_trait;

use crate::user::Email;

pub struct SendEmail<'a> {
    pub recipient: &'a Email,
    pub subject: &'a str,
    pub content: &'a str,
}

#[async_trait]
pub trait EmailClient {
   async fn send_email<'a>(&self, email_detes: SendEmail<'a>) -> Result<(), String>;
}