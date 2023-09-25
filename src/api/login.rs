use crate::api::url;
use reqwest::header::ACCEPT;
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize)]
struct LoginRequest<'a> {
    #[serde(rename = "EmailAddress")]
    email_address: &'a str,
    #[serde(rename = "Password")]
    password: &'a str,
    #[serde(rename = "IsServiceStatusReturned")]
    is_service_status_returned: bool,
    #[serde(rename = "ApiActive")]
    api_active: bool,
    #[serde(rename = "ApiDown")]
    api_down: bool,
    #[serde(rename = "RedirectUrl")]
    redirect_url: &'static str,
    #[serde(rename = "events")]
    events: Vec<()>,
    #[serde(rename = "formErrors")]
    form_errors: Vec<()>,
}

impl<'a> LoginRequest<'a> {
    fn from_email_password(email: &'a str, password: &'a str) -> Self {
        Self {
            email_address: email,
            password,
            is_service_status_returned: true,
            api_active: true,
            api_down: false,
            redirect_url: "",
            events: vec![],
            form_errors: vec![],
        }
    }
}

#[derive(Deserialize)]
struct LoginResponse {
    #[serde(rename = "Content")]
    content: Content,
}

#[derive(Deserialize)]
struct Content {
    #[serde(rename = "UserId")]
    user_id: String,
    #[serde(rename = "DisplayName")]
    display_name: String,
}

pub struct LoggedIn {
    pub user_id: String,
    pub display_name: String,
    pub session_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Error)]
pub enum LoginError {
    #[error("{0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Cookies was missing")]
    NoCookie,
}

pub async fn login<S1: AsRef<str>, S2: AsRef<str>>(
    client: &Client,
    email: S1,
    password: S2,
) -> Result<LoggedIn, LoginError> {
    let response = client
        .post(url("/accountApi/login"))
        .header(ACCEPT, "application/json")
        .json(&LoginRequest::from_email_password(
            email.as_ref(),
            password.as_ref(),
        ))
        .send()
        .await?
        .error_for_status()?;

    let session_token = find_cookie(&response, "SessionCookie")?;
    let refresh_token = find_cookie(&response, "RefreshCookie")?;
    let payload: LoginResponse = response.json().await?;

    Ok(LoggedIn {
        user_id: payload.content.user_id,
        display_name: payload.content.display_name,
        session_token,
        refresh_token,
    })
}

fn find_cookie(response: &Response, name: &str) -> Result<String, LoginError> {
    Ok(response
        .cookies()
        .find(|cookie| cookie.name().eq(name))
        .ok_or(LoginError::NoCookie)?
        .value()
        .to_string())
}
