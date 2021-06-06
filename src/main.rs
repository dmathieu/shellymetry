use std::{
    error::Error,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};

mod config;
mod flags;
mod runner;
mod server;
mod shelly;

const LOCALHOST: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let exporter = opentelemetry_prometheus::exporter().init();

    let flags = flags::build();
    let config = config::build(flags.config).unwrap();

    match runner::run(&config).await {
        Ok(()) => {
            // Everything went all right
        }
        Err(err) => {
            println!("runner failed: {}", err);
        }
    }
    let addr = SocketAddr::new(LOCALHOST, config.server_port);
    match server::build(addr, exporter).start().await {
        Ok(()) => {
            // Everything went all right
        }
        Err(err) => {
            println!("server failed: {}", err);
        }
    }

    Ok(())
}
