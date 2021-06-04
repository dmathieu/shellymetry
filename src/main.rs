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
    let flags = flags::build();
    let config = config::build(flags.config).unwrap();

    let addr = SocketAddr::new(LOCALHOST, config.server_port);
    let exporter = opentelemetry_prometheus::exporter().init();
    server::build(addr, exporter).start().await
}
