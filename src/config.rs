use config;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Device {
    pub kind: String,
    pub name: String,
    pub labels: HashMap<String, String>,
}

impl Device {
    pub fn url(&self) -> String {
        format!(
            "http://shelly{}-{}.local/status",
            self.kind,
            self.name.to_uppercase()
        )
        .to_string()
    }
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server_port: u16,
    pub devices: Vec<Device>,
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

        let mut labels = HashMap::new();
        labels.insert("room".to_string(), "bedroom".to_string());
        assert_eq!(
            vec![Device {
                kind: "plug".to_string(),
                name: "ABCDEF".to_string(),
                labels: labels,
            }],
            config.devices
        );
    }

    #[test]
    fn test_device() {
        let device = Device {
            kind: "plug".to_string(),
            name: "foobar".to_string(),
            labels: HashMap::new(),
        };

        assert_eq!(
            "http://shellyplug-FOOBAR.local/status".to_string(),
            device.url()
        )
    }
}
