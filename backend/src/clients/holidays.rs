use reqwest::{Client, Result};
use std::collections::HashMap;

use crate::utils::url::get_base_url;

type FetchHolidaysResponse = HashMap<String, String>;

pub async fn fetch_holidays() -> Result<FetchHolidaysResponse> {
    let base_url = get_base_url("https://holidays-jp.github.io");
    let client = Client::new();
    let response = client
        .get(format!("{}/api/v1/date.json", base_url))
        .send()
        .await?
        .json::<FetchHolidaysResponse>()
        .await?;

    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::mocks::server::MockServer;
    use serde_json::json;

    #[tokio::test]
    async fn fetch_holidays_ok() {
        let server = MockServer::new_async().await;
        let path = "/api/v1/date.json";
        let json_body = json!({ "2025-01-01": "休み" });
        {
            let mut srv = server.0.lock().await;
            srv.mock("GET", path)
                .with_status(200)
                .with_header("content-type", "application/json")
                .with_body(json_body.to_string())
                .create_async()
                .await;
            let response = fetch_holidays().await;

            assert!(response.is_ok());
            assert_eq!(response.unwrap().get("2025-01-01").unwrap(), "休み");

            srv.reset();
        }
    }

    #[tokio::test]
    async fn fetch_holidays_err() {
        let server = MockServer::new_async().await;
        let path = "/api/v1/date.json";
        let json_body = json!("invalid");
        {
            let mut srv = server.0.lock().await;
            srv.mock("GET", path)
                .with_status(200)
                .with_header("content-type", "application/json")
                .with_body(json_body.to_string())
                .create_async()
                .await;

            let response = fetch_holidays().await;

            assert!(response.is_err());
            srv.reset();
        }
    }
}
