use opentelemetry::global;
use std::{
    error::Error,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};

mod config;
mod flags;
mod server;
mod shelly;

const LOCALHOST: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let exporter = opentelemetry_prometheus::exporter().init();

    let flags = flags::build();
    let config = config::build(flags.config).unwrap();

    update_devices(&config).await;
    let addr = SocketAddr::new(LOCALHOST, config.server_port);
    server::build(addr, exporter).start().await
}

async fn update_devices(config: &config::Config) {
    let meter = global::meter("shellymetry");
    let uptime = meter
        .u64_value_recorder("shelly_device_uptime")
        .with_description("The device's uptime in seconds.")
        .init();

    for device in config.devices.iter() {
        println!("Loading data for {}", device.name);
        let data = shelly::load(device.url()).await.unwrap();
        uptime.record(data.uptime, &[]);
    }
}
