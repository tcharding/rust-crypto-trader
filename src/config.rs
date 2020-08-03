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

#[derive(Debug, Deserialize)]
pub struct Config {
    pub keys: Keys,
}

#[derive(Debug, Deserialize)]
pub struct Keys {
    pub read: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_works() {
        let config: Config = toml::from_str(
            r#"
        [keys]
        read = "b2111111-4b1c-4880-b4c4-036d81f3de59"
    "#,
        )
        .unwrap();

        assert_eq!(&config.keys.read, "b2111111-4b1c-4880-b4c4-036d81f3de59");
    }
}
