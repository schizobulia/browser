use reqwest;

/**
 * Get html by url
 */
#[tokio::main]
pub async fn get_html_by_url(url: String) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?.text().await?;
    Ok(response)
}

#[tokio::main]
pub async fn get_img_by_url(url: String) -> Result<Vec<u8>, reqwest::Error> {
    let response = reqwest::get(url).await?.bytes().await?;
    Ok(response.to_vec())
}
