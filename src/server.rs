use hyper::{
    header::CONTENT_TYPE,
    service::{make_service_fn, service_fn},
    Body, Method, Request, Response,
};
use opentelemetry_prometheus::PrometheusExporter;
use prometheus::{Encoder, TextEncoder};
use std::{convert::Infallible, error::Error, net::SocketAddr, sync::Arc};

pub struct Server {
    addr: SocketAddr,
    exporter: PrometheusExporter,
}

struct AppState {
    exporter: PrometheusExporter,
}

pub fn build(addr: SocketAddr, exporter: PrometheusExporter) -> Server {
    Server {
        addr: addr,
        exporter: exporter,
    }
}

impl Server {
    pub async fn start(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        let state = Arc::new(AppState {
            exporter: self.exporter.clone(),
        });
        let make_svc = make_service_fn(move |_conn| {
            let state = state.clone();
            async move { Ok::<_, Infallible>(service_fn(move |req| serve_req(req, state.clone()))) }
        });

        let server = hyper::Server::bind(&self.addr).serve(make_svc);
        println!("server running on {}", self.addr);
        server.await?;
        Ok(())
    }
}

async fn serve_req(
    req: Request<Body>,
    state: Arc<AppState>,
) -> Result<Response<Body>, hyper::Error> {
    println!("Receiving request at path {}", req.uri());

    let response = match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Response::builder()
            .status(200)
            .body(Body::from("Hello World"))
            .unwrap(),
        (&Method::GET, "/metrics") => {
            let mut buffer = vec![];
            let encoder = TextEncoder::new();
            let metric_families = state.exporter.registry().gather();
            encoder.encode(&metric_families, &mut buffer).unwrap();

            Response::builder()
                .status(200)
                .header(CONTENT_TYPE, encoder.format_type())
                .body(Body::from(buffer))
                .unwrap()
        }
        _ => Response::builder()
            .status(404)
            .body(Body::from("Missing Page"))
            .unwrap(),
    };
    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_build() {
        let exporter = opentelemetry_prometheus::exporter().init();
        let _server = build("127.0.0.1:1345".parse().unwrap(), exporter);
    }
}
