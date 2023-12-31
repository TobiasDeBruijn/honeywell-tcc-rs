use honeywell_tcc::TccApi;
use std::env::var;

#[tokio::test]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let api = TccApi::new_with_login(var("TCC_EMAIL")?, var("TCC_PASSWORD")?).await?;

    let locations = api.get_locations().await?;

    println!("{locations:?}");

    Ok(())
}
