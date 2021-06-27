use opentelemetry::{
    global,
    trace::{Span, Tracer},
    Context, Key,
};
use reqwest::Client;
use serde::Deserialize;

const URL_KEY: Key = Key::from_static_str("url");

#[derive(Debug, Deserialize)]
pub struct Meters {
    pub power: f64,
}

#[derive(Debug, Deserialize)]
pub struct Shelly {
    pub uptime: u64,
    pub meters: Vec<Meters>,

    pub ram_total: u64,
    pub ram_free: u64,
    pub fs_size: u64,
    pub fs_free: u64,
}

pub async fn load(cx: Context, url: String) -> Result<Shelly, Box<dyn std::error::Error>> {
    let mut span = global::tracer("shellymetry").start_with_context("shelly.load", cx);
    span.set_attribute(URL_KEY.string(url.clone()));

    let resp = Client::new().get(url).send().await?;
    let shelly = resp.json::<Shelly>().await?;
    Ok(shelly)
}

#[cfg(test)]
mod tests {
    use super::*;
    use httptest::{matchers::*, responders::*, Expectation, Server};
    use std::fs;

    #[tokio::test]
    async fn test_shelly_load() {
        let content = fs::read_to_string("fixtures/plug.json").unwrap();
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path("GET", "/shelly"))
                .respond_with(status_code(200).body(content)),
        );

        let shelly = load(Context::current(), server.url("/shelly").to_string())
            .await
            .unwrap();
        assert_eq!(343771, shelly.uptime);
        assert_eq!(88.44, shelly.meters[0].power);
        assert_eq!(50616, shelly.ram_total);
        assert_eq!(38840, shelly.ram_free);
        assert_eq!(233681, shelly.fs_size);
        assert_eq!(164907, shelly.fs_free);
    }
}
