use anyhow::Result;
use lettre::{message::header::ContentType, transport::smtp::authentication::Credentials, AsyncSmtpTransport, AsyncTransport, Message, SmtpTransport, Tokio1Executor, Transport};
use tracing::info;
use crate::utils;

pub async fn send_mail(to: &str, subject: &str, body: &str) -> Result<()> {
    let info = utils::config::get_config_smtp();
    let smtp_from  = &info.smtp_from;
    let from: lettre::message::Mailbox = smtp_from.parse()?;
    let email = Message::builder()
        .from(from)
        .to(to.parse()?)
        .subject(subject)
        .header(ContentType::TEXT_HTML)
        .body(String::from(body))?;

    let smtp_username = (&info.smtp_user_name).clone();
    let smtp_password = (&info.smtp_password).clone();
    let creds = Credentials::new(smtp_username, smtp_password);

    let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.gmail.com")?
        .credentials(creds)
        .build();
    

    let response = mailer.send(email).await?;
    info!("to: {:?}, response: {:?}", to, response);
    Ok(())
}