use opentelemetry::{
    global::shutdown_tracer_provider,
    sdk::{export::trace::stdout, trace::Tracer},
};
use opentelemetry_prometheus::PrometheusExporter;
use std::{
    collections::HashMap,
    error::Error,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};
use tokio::try_join;

mod config;
mod flags;
mod runner;
mod server;
mod shelly;

const LOCALHOST: IpAddr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let flags = flags::build();
    let config = config::build(flags.config).unwrap();
    let (exporter, _tracer) = init_telemetry(&config).unwrap();

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

fn init_telemetry(config: &config::Config) -> Result<(PrometheusExporter, Tracer), Box<dyn Error>> {
    let exporter = opentelemetry_prometheus::exporter().init();

    let headers: HashMap<String, String> = match &config.otlp_headers {
        Some(h) => h.clone(),
        None => HashMap::new(),
    };
    let tracer: Tracer = match &config.otlp_endpoint {
        Some(endpoint) => opentelemetry_otlp::new_pipeline()
            .with_endpoint(endpoint.to_string())
            .with_grpcio()
            .with_tls(true)
            .with_headers(headers)
            .install_batch(opentelemetry::runtime::Tokio)?,
        None => stdout::new_pipeline()
            .with_pretty_print(true)
            .install_simple(),
    };

    Ok((exporter, tracer))
}
