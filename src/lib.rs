use crate::api::locations::get_locations;
use crate::api::login::{login, LoggedIn, LoginError};
use crate::api::zones::set_zone_temperature;
use crate::api::USER_AGENT;
use reqwest::Client;

mod api;

pub use api::locations::{Location, Zone};

/// The honeywell TCC api
pub struct TccApi {
    client: Client,
    login: LoggedIn,
}

impl TccApi {
    /// Create a new API client and log in with Honeywell
    ///
    /// # Errors
    ///
    /// If the request fails
    pub async fn new_with_login<S1: AsRef<str>, S2: AsRef<str>>(
        email: S1,
        password: S2,
    ) -> Result<Self, LoginError> {
        let client = Client::builder()
            .user_agent(USER_AGENT)
            .cookie_store(true)
            .build()?;

        let login = login(&client, email, password).await?;

        Ok(Self { client, login })
    }

    /// Get the display name of the logged in user
    pub fn display_name(&self) -> &str {
        &self.login.display_name
    }

    /// Get the ID of the logged in user
    pub fn user_id(&self) -> &str {
        &self.login.user_id
    }

    /// Get locations
    ///
    /// # Errors
    ///
    /// If the request fails
    pub async fn get_locations(&self) -> Result<Vec<Location>, reqwest::Error> {
        get_locations(&self.client).await
    }

    /// Set the temperature of a zone.
    /// The temperature may only be a whole integer or half of an integer. E.g. `13.5` and `13.0` are fine, `13.7` is not.
    ///
    /// # Errors
    ///
    /// If the request fails
    pub async fn set_zone_temperature(
        &self,
        zone_id: impl Into<String>,
        temperature: f32,
    ) -> Result<(), reqwest::Error> {
        set_zone_temperature(&self.client, zone_id, temperature).await
    }
}
