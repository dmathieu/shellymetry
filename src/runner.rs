use crate::{config, shelly};
use opentelemetry::{global, trace::Tracer};
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
        let tracer = global::tracer("shellymetry");

        loop {
            interval.tick().await;
            tracer
                .in_span("runner.tick", |_cx| async {
                    for device in config.devices.iter() {
                        tracer
                            .in_span("runner.tick.update", |_cx| async {
                                let data = shelly::load(device.url()).await.unwrap();
                                uptime.record(data.uptime, &device.kv_labels());
                            })
                            .await;
                    }
                })
                .await;
        }
    })
    .await?
}
