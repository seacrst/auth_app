use async_trait::async_trait;

use crate::app::email_client::{EmailClient, SendEmail};

pub mod api;
pub mod auth;
pub mod tokens;
pub mod constants;
pub mod two_fa;
pub struct MockEmailClient;

#[async_trait]
impl EmailClient for MockEmailClient {
    async fn send_email<'a>(&self, email_detes: SendEmail<'a>) -> Result<(), String> {
        let SendEmail {content, recipient, subject} = email_detes;
        println!(
            "Sending email to {} with subject: {} and content: {}",
            recipient.as_ref(), subject, content
        );
        Ok(())
    }
}