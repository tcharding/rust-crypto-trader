use anyhow::Result;
use serde::Deserialize;
use std::{fs::File, io::prelude::*, path::Path};

/// Attempt to load and parse the config file into our Config struct.
/// If a file cannot be found, or we cannot parse it, return an error.
pub fn parse(path: &Path) -> Result<Config> {
    let mut config_toml = String::new();
    let mut file = File::open(path)?;
    file.read_to_string(&mut config_toml)?;
    let config: Config = toml::from_str(&config_toml)?;

    Ok(config)
}

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub ir: Exchange,
    pub kraken: Exchange,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Exchange {
    /// A read-only API Key.
    pub read_only: Key,
}

/// A single key, made up of public and private parts.
#[derive(Clone, Debug, Deserialize)]
pub struct Key {
    pub api_key: String,
    pub api_secret: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use spectral::prelude::*;

    #[test]
    fn config_works() {
        let config: Config = toml::from_str(
            r#"
        [ir]

                [read-only]
                api_key = "b2111111-4b1c-4880-b4c4-036d81f3de59"
                api_secret = "11111193333335555558888888111111"
    "#,
        )
        .unwrap();

        let want_key = "b2111111-4b1c-4880-b4c4-036d81f3de59".to_string();
        let want_secret = "11111193333335555558888888111111".to_string();
        assert_that!(&config.ir.read_only.api_key).is_equal_to(&want_key);
        assert_that!(&config.ir.read_only.api_secret).is_equal_to(&want_secret)
    }
}
