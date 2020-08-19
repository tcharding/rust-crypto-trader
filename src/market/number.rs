//! Custom Decimal wrapper type.
//!
//! We use this to catch serder errors when ser/deser numbers from an API call.

use rust_decimal::Decimal;
use serde::{
    de::Error as _, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
};

pub struct Number {
    inner: Option<Decimal>,
}

impl Serialize for Number {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for Number {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        let role = String::deserialize(deserializer)?;
        let role =
            Role::from_str(role.as_str()).map_err(<D as Deserializer<'de>>::Error::custom)?;

        Ok(Http(role))
    }
}
