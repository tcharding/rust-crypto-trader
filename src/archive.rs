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
