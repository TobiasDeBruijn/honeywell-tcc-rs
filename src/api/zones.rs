use reqwest::Client;
use serde::Serialize;
use crate::api::url;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SetZoneTemperatureRequest {
    zone_id: String,
    heat_temperature: String,
    hot_water_state_is_on: bool,
    is_permanent: bool,
    set_until_hours: String,
    set_until_minutes: String,
    location_time_offset_minutes: i32,
    is_following_schedule: bool,
}

impl SetZoneTemperatureRequest {
    fn new(zone_id: String, temperature: f32) -> Self {
        Self {
            zone_id,
            heat_temperature: format!("{temperature:.1}"),
            hot_water_state_is_on: false,
            is_permanent: true,
            set_until_hours: "00".to_string(),
            set_until_minutes: "00".to_string(),
            location_time_offset_minutes: 120,
            is_following_schedule: false,
        }
    }
}

pub async fn set_zone_temperature(client: &Client, zone_id: impl Into<String>, temperature: f32) -> Result<(), reqwest::Error> {
    client
        .post(url("/ZonesApi/SetZoneTemperature"))
        .json(&SetZoneTemperatureRequest::new(zone_id.into(), temperature))
        .send()
        .await?
        .error_for_status()?;

    Ok(())
}