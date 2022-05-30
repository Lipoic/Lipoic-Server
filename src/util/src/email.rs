use lettre::transport::smtp::authentication::Credentials;
use lettre::{
    message::{header, MultiPart, SinglePart},
    Message, SmtpTransport, Transport,
};
use maud::html;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyEmailClaims {
    pub exp: usize,
    pub email: String,
}

pub fn send_verify_email(
    username: &String,
    password: &String,
    issuer: &String,
    auth_path: String,
    code: String,
    to_email: &String,
) {
    let url = format!("{}{}?code={}", issuer, auth_path, code);
    let email_html = html! {
        head {
            title { "Lipoic Email Verify" }
        }
        div {
            a href=(url) { "Verified Email" }
        }
    };

    let email = Message::builder()
        .from(username.parse().unwrap())
        .to(to_email.parse().unwrap())
        .subject("Lipoic Email Verify")
        .multipart(
            MultiPart::alternative() // This is composed of two parts.
                .singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::TEXT_HTML)
                        .body(email_html.into_string()),
                ),
        )
        .unwrap();

    let creds = Credentials::new(username.clone(), password.clone());
    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    mailer.send(&email).unwrap();
}
