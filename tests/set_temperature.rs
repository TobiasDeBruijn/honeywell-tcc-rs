use color_eyre::eyre::Error;
use honeywell_tcc::TccApi;
use std::env::var;

#[tokio::test]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let api = TccApi::new_with_login(var("TCC_EMAIL")?, var("TCC_PASSWORD")?).await?;

    let locations = api.get_locations().await?;
    let zone = locations
        .first()
        .ok_or(Error::msg("No locations available"))?
        .zones
        .first()
        .ok_or(Error::msg("No zone available"))?;

    api.set_zone_temperature(zone.id.clone(), 13.5).await?;

    Ok(())
}
