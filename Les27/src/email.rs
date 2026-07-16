use std::env;

use axum::http::StatusCode;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor, message::{MultiPart, SinglePart}, transport::smtp::authentication::Credentials};

fn give_html_body(code: String) -> String{
    return  format!(r#"<!DOCTYPE html>
    <html>
    <head>
    <meta charset="UTF-8">
    </head>
    <body style="margin:0; padding:0; background-color:#f4f4f7; font-family: Arial, Helvetica, sans-serif;">
    <table width="100%" cellpadding="0" cellspacing="0" style="background-color:#f4f4f7; padding: 40px 0;">
        <tr>
        <td align="center">
            <table width="480" cellpadding="0" cellspacing="0" style="background-color:#ffffff; border-radius:12px; overflow:hidden; box-shadow: 0 2px 8px rgba(0,0,0,0.08);">
            <tr>
                <td style="background-color:#4f46e5; padding:24px; text-align:center;">
                <h1 style="color:#ffffff; margin:0; font-size:20px;">Verification Code</h1>
                </td>
            </tr>
            <tr>
                <td style="padding:32px 32px 16px 32px; text-align:center;">
                <p style="color:#333333; font-size:15px; margin:0 0 24px 0;">
                    Use the code below to verify your account. This code will expire in 5 minutes.
                </p>
                <div style="background-color:#f4f4f7; border-radius:8px; padding:20px; margin-bottom:24px;">
                    <span style="font-size:32px; font-weight:bold; letter-spacing:8px; color:#4f46e5;">
                    {code}
                    </span>
                </div>
                <p style="color:#999999; font-size:13px; margin:0;">
                    If you didn't request this code, you can safely ignore this email.
                </p>
                </td>
            </tr>
            <tr>
                <td style="padding:16px 32px 32px 32px; text-align:center;">
                <p style="color:#cccccc; font-size:12px; margin:0;">
                    &copy; {year} MorTask. All rights reserved.
                </p>
                </td>
            </tr>
            </table>
        </td>
        </tr>
    </table>
    </body>
    </html>"#,
    code = code,
    year = chrono::Utc::now().format("%Y"));
}

pub async fn send_email(email: String, code: String)
-> Result<(), (StatusCode, String)> {
    let smtp_user = env::var("SMTP_USERNAME")
    .map_err(|_e| {
        tracing::error!("SMTP_USERNAME error in .env file.");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "We were unable to send the code to your email address".to_string()
        )
    })?;
    let smtp_pass = env::var("SMTP_PASSWORD")
    .map_err(|_e| {
        tracing::error!("SMTP_PASSWORD error in .env file.");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "We were unable to send the code to your email address".to_string()
        )
    })?;

    let email = Message::builder()
    .from(smtp_user.parse().map_err(|_| {
        (StatusCode::INTERNAL_SERVER_ERROR, "Invalid sender email".to_string())
    })?)
    .to(email.parse().map_err(|_e|{
        (StatusCode::BAD_REQUEST, "Your email address is invalid.".to_string())
    })?)
    .subject("Your verification code")
    .multipart(
        MultiPart::alternative()
        .singlepart(
            SinglePart::plain(format!("Your verification code is: {code}"))
        )
        .singlepart(
            SinglePart::html(give_html_body(code))
        )
    )
    .map_err(|_e|{
        (StatusCode::INTERNAL_SERVER_ERROR, "Failed to build email".to_string())
    })?;

    let creds = Credentials::new(smtp_user, smtp_pass);

    let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.gmail.com")
        .map_err(|_e|{
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to connect to SMTP server".to_string())
        })?
        .credentials(creds)
        .build::<Tokio1Executor>();

    mailer.send(email)
    .await
    .map_err(|e|{
        tracing::error!("Email send error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR,
        "We were unable to send the code to your email address.".to_string())
    })?;

    Ok(())
}
