use reqwest;

#[tokio::main]
pub async fn get_html_by_url(url: String) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?.text().await?;
    Ok(response)
}
