use reqwest::Error;

pub struct HttpResponseAdapter {
    pub url: String,
    pub content: String,
}


pub async fn handle_http_request(url: &str) -> Result<HttpResponseAdapter, Error> {

    let client = reqwest::Client::new();

    let response = client.get(url).send().await?;

    if response.status().is_success() {
        let body = response.text().await?;

        Ok(HttpResponseAdapter {
            url: url.to_string(),
            content: body,
        }) 
    } else {
        Err(reqwest::Error::new(
            reqwest::StatusCode::BAD_REQUEST,
            "Request Failed",
        ))
    }

}