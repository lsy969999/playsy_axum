use anyhow::Result;
use lettre::{message::header::ContentType, transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};
use tracing::info;
use crate::utils;

pub fn send_mail(to: &str, subject: &str, body: &str) -> Result<()> {
    let info = utils::settings::get_settings_smtp_info();
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

    let mailer: SmtpTransport = SmtpTransport::relay("smtp.gmail.com")?
        .credentials(creds)
        .build();

    let response = mailer.send(&email)?;
    info!("to: {:?}, response: {:?}", to, response);
    Ok(())
}