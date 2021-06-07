use opentelemetry::{
    global::shutdown_tracer_provider,
    sdk::{export::trace::stdout, trace::Tracer},
};
use opentelemetry_prometheus::PrometheusExporter;
use std::{
    error::Error,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};
use tokio::try_join;

mod config;
mod flags;
mod runner;
mod server;
mod shelly;

const LOCALHOST: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (exporter, _tracer) = init_telemetry().unwrap();

    let flags = flags::build();
    let config = config::build(flags.config).unwrap();

    let addr = SocketAddr::new(LOCALHOST, config.server_port);
    let s = server::build(addr, exporter);

    match try_join!(runner::run(config), s.start()) {
        Ok((_, _)) => {
            // Everything went all right
        }
        Err(err) => {
            println!("failure: {}", err);
        }
    }

    shutdown_tracer_provider();
    Ok(())
}

fn init_telemetry() -> Result<(PrometheusExporter, Tracer), Box<dyn Error>> {
    let exporter = opentelemetry_prometheus::exporter().init();
    let tracer = stdout::new_pipeline()
        .with_pretty_print(true)
        .install_simple();

    Ok((exporter, tracer))
}
