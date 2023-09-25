use reqwest::Client;
use reqwest::header::ACCEPT;
use serde::Deserialize;
use crate::api::url;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct GetLocationsResponse {
    content: GetLocationsContent,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct GetLocationsContent {
    locations: Vec<Location>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Location {
    pub name: String,
    pub id: String,
    pub city: String,
    pub country: String,
    pub zones: Vec<Zone>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Zone {
    pub id: String,
    pub name: String,
    pub temperature: f32,
    pub min_heat_setpoint: f32,
    pub max_heat_setpoint: f32,
    pub target_heat_temperature: f32,
}

pub async fn get_locations(client: &Client) -> Result<Vec<Location>, reqwest::Error> {
    let payload: GetLocationsResponse = client.get(url("/locationsapi/getLocations"))
        .header(ACCEPT, "application/json")
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(payload.content.locations)
}
