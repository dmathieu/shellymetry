use config;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server_port: u16,
}

pub fn build(config_path: String) -> Result<Config, config::ConfigError> {
    let mut c = config::Config::default();
    c.merge(config::File::new(&config_path, config::FileFormat::Json))
        .unwrap();

    c.try_into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_build() {
        let config = build("fixtures/config.json".to_string()).unwrap();
        assert_eq!(1304, config.server_port);
    }
}
