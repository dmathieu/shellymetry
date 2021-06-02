use hyper::{
    service::{make_service_fn, service_fn},
    Body, Method, Request, Response,
};
use std::{convert::Infallible, error::Error, net::SocketAddr};

pub struct Server {
    addr: SocketAddr,
}

pub fn build(addr: SocketAddr) -> Server {
    Server { addr: addr }
}

impl Server {
    pub async fn start(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        let make_svc = make_service_fn(move |_conn| async move {
            Ok::<_, Infallible>(service_fn(move |req| serve_req(req)))
        });

        let server = hyper::Server::bind(&self.addr).serve(make_svc);
        println!("server running on {}", self.addr);
        server.await?;
        Ok(())
    }
}

async fn serve_req(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    println!("Receiving request at path {}", req.uri());

    let response = match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Response::builder()
            .status(200)
            .body(Body::from("Hello World"))
            .unwrap(),
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
        let _server = build("127.0.0.1:1345".parse().unwrap());
    }
}
