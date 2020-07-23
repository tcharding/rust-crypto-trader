// This file is not built into the project, this is a scratch pad for shit I
// wrote that might be useful later.

pub struct DecimalPlaces {
    pub currency: String,
    pub code: String,
    pub volume: usize, // Volume decimal places.
    pub fiat: usize,   // Fiat offer/bid decimal places.
}

pub fn bitcoin_decimal_places() -> DecimalPlaces {
    DecimalPlaces {
        currency: "bitcoin".to_string(),
        code: "xbt".to_string(),
        volume: 8,
        fiat: 2,
    }
}


    debug!("{}", url.to_string());

    let body = client.get(url).send().await?.text().await?;
    println!("{:?}", body);
     unimplemented!();



pub async fn get_valid_primary_currency_codes(client: Client) -> Result<Vec<String>> {
    let url = build_url("GetValidPrimaryCurrencyCodes")?;
    let body = client.get(url).send().await?.text().await?;
    let v: Vec<String> = serde_json::from_str(&body)?;

    Ok(v)
}

pub async fn get_valid_secondary_currency_codes(client: Client) -> Result<Vec<String>> {
    let url = build_url("GetValidSecondaryCurrencyCodes")?;
    let body = client.get(url).send().await?.text().await?;
    let v: Vec<String> = serde_json::from_str(&body)?;

    Ok(v)
}
