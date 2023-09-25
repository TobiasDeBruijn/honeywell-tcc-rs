pub mod locations;
pub mod login;
pub mod zones;

pub(crate) const USER_AGENT: &str =
    "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/117.0";
const TCC_URL: &str = "https://international.mytotalconnectcomfort.com/api/";

fn url(url: &str) -> String {
    format!("{TCC_URL}{url}")
}
