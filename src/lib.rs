use anyhow::Result;
use reqwest::Client;

const URL: &str = "https://api.independentreserve.com";

pub async fn get_valid_primary_currency_codes(client: Client) -> Result<Vec<String>> {
    let url = public().with_path("GetValidPrimaryCurrencyCodes");
    let body = client.get(&url).send().await?.text().await?;
    let v: Vec<String> = serde_json::from_str(&body)?;

    Ok(v)
}

pub fn public() -> String {
    format!("{}/Public", URL)
}

pub trait WithPath {
    fn with_path(&self, path: &str) -> Self;
}

impl WithPath for String {
    fn with_path(&self, path: &str) -> Self {
        format!("{}/{}", self, path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use spectral::prelude::*;

    #[test]
    fn path_construction_works() {
        let want = format!("{}/Public/foo", URL);
        let got = public().with_path("foo");
        assert_that(&got).is_equal_to(&want);
    }
}
