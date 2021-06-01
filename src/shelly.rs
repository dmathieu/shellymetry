use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Shelly {
    pub uptime: u64,
}

pub async fn load(url: String) -> Result<Shelly, Box<dyn std::error::Error>> {
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

        let shelly = load(server.url("/shelly").to_string()).await.unwrap();
        assert_eq!(343771, shelly.uptime);
    }
}
