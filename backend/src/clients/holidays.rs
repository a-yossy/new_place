use reqwest::{Client, Result};
use std::collections::HashMap;

type FetchHolidaysResponse = HashMap<String, String>;

pub async fn fetch_holidays() -> Result<FetchHolidaysResponse> {
    let client = Client::new();
    let response = client
        .get("https://holidays-jp.github.io/api/v1/date.json")
        .send()
        .await?
        .json::<FetchHolidaysResponse>()
        .await?;

    Ok(response)
}
