use reqwest::{Error, StatusCode, Response};

pub struct HttpResponseAdapter { 
    pub url: String,
    pub content: String,
    pub status_code: StatusCode,
    pub headers: reqwest::header::HeaderMap,
} 

pub async fn handle_http_request(url: &str) -> Result<HttpResponseAdapter, Error> {
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;

    if response.status().is_success() {
        let body = response.text().await?;
        let status_code = response.status();
        let headers = response.headers().clone();

        Ok(HttpResponseAdapter {
            url: url.to_string(),
            content: body,
            status_code,
            headers,
        }) 
    } else {
        Err(Error::from(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Request failed with status: {}", response.status())
        )))
    }
}