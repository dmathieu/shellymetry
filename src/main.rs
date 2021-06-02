use std::{
    error::Error,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};

mod server;
mod shelly;

const LOCALHOST: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let addr = SocketAddr::new(LOCALHOST, 1304);
    server::build(addr).start().await
}
