use crate::{config, shelly};
use opentelemetry::{
    global,
    trace::{Span, Tracer},
};
use std::{error::Error, time::Duration};
use tokio::{task, time};

pub async fn run(config: config::Config) -> Result<(), Box<dyn Error + Send + Sync>> {
    let meter = global::meter("shellymetry");
    let uptime = meter
        .u64_value_recorder("shelly_device_uptime")
        .with_description("The device's uptime in seconds.")
        .init();
    let power = meter
        .f64_value_recorder("shelly_device_power")
        .with_description("The device's current voltage.")
        .init();

    let ram_total = meter
        .u64_value_recorder("shelly_device_ram_total")
        .with_description("The device's total ram.")
        .init();
    let ram_free = meter
        .u64_value_recorder("shelly_device_ram_free")
        .with_description("The device's free ram.")
        .init();
    let fs_size = meter
        .u64_value_recorder("shelly_device_fs_size")
        .with_description("The device's total fs.")
        .init();
    let fs_free = meter
        .u64_value_recorder("shelly_device_fs_free")
        .with_description("The device's free fs.")
        .init();

    task::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(config.refresh_interval));
        let tracer = global::tracer("shellymetry");

        loop {
            interval.tick().await;
            tracer
                .in_span("runner.tick", |_cx| async {
                    for device in config.devices.iter() {
                        let mut span = tracer.start("runner.tick.update");

                        match shelly::load(device.url()).await {
                            Ok(data) => {
                                uptime.record(data.uptime, &device.kv_labels());
                                power.record(data.meters[0].power, &device.kv_labels());

                                ram_total.record(data.ram_total, &device.kv_labels());
                                ram_free.record(data.ram_free, &device.kv_labels());
                                fs_size.record(data.fs_size, &device.kv_labels());
                                fs_free.record(data.fs_free, &device.kv_labels());
                            }
                            Err(err) => span.record_exception(&*err),
                        };
                    }
                })
                .await;
        }
    })
    .await?
}
