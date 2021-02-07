use hyper::{Body, Client, Response};

pub async fn get(url: String) -> serve_static::Result<Response<Body>> {
    let client = Client::new();
    let url = url.parse()?;
    let resp = client.get(url).await?;
    log::debug!("{:?}", resp);
    Ok(resp)
}
