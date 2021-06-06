use crate::{config, shelly};
use opentelemetry::global;
use std::{error::Error, time::Duration};
use tokio::{task, time};

pub async fn run(config: config::Config) -> Result<(), Box<dyn Error + Send + Sync>> {
    let meter = global::meter("shellymetry");
    let uptime = meter
        .u64_value_recorder("shelly_device_uptime")
        .with_description("The device's uptime in seconds.")
        .init();

    task::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(config.refresh_interval));

        loop {
            interval.tick().await;

            for device in config.devices.iter() {
                println!("Loading data for {}", device.name);
                let data = shelly::load(device.url()).await.unwrap();
                uptime.record(data.uptime, &device.kv_labels());
            }
        }
    })
    .await?
}
