use eyre::{bail, Result};
use reqwest::{get, Client};
use serde::{Deserialize, Serialize};

const BASE_API_URL: &str = "http://localhost:3000";

#[tokio::test]
async fn server_sets_header() -> Result<()> {
    let url = format!("{BASE_API_URL}/set_header");
    let response = get(url).await?;

    assert_eq!(response.status(), 200);

    let headers = response.headers();
    let Some(content_type ) =  headers.get("content-type") else { bail!("content type header not set")};

    assert_eq!(content_type.to_str()?, "application/json");
    assert_eq!(response.text().await?, "hello world");

    Ok(())
}

#[tokio::test]
async fn server_sets_custom_header() -> Result<()> {
    let url = format!("{BASE_API_URL}/set_custom_header");
    let response = get(url).await?;

    assert_eq!(response.status(), 200);

    let headers = response.headers();
    let Some(token_header) =  headers.get("token") else { bail!("Custom header 'token' not set")};

    assert_eq!(token_header.to_str()?, "Bearer 1234567890");

    Ok(())
}

#[tokio::test]
async fn server_sets_custom_and_typed_header() -> Result<()> {
    let url = format!("{BASE_API_URL}/set_custom_and_typed_header");
    let response = get(url).await?;

    assert_eq!(response.status(), 200);

    let headers = response.headers();
    let Some(content_type ) =  headers.get("content-type") else { bail!("content type header not set")};
    let Some(token_header) =  headers.get("token") else { bail!("Custom header 'token' not set")};

    assert_eq!(token_header.to_str()?, "Bearer 1234567890");
    assert_eq!(content_type.to_str()?, "application/json");

    assert_eq!(response.text().await?, "hello world");

    Ok(())
}

#[tokio::test]
async fn server_gets_custom_and_typed_headers() -> Result<()> {
    let url = format!("{BASE_API_URL}/get_custom_and_typed_headers");
    let client = Client::new();
    let response = client
        .get(url)
        .header("content-type", "application/json")
        .header("token", "Bearer 1234567890")
        .send()
        .await?;

    assert_eq!(response.status(), 200);

    let response_headers = response.json::<ResponseHeaders>().await?;

    assert_eq!(
        response_headers,
        ResponseHeaders {
            content_type: "application/json".to_owned(),
            token: "Bearer 1234567890".to_owned()
        }
    );

    Ok(())
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct ResponseHeaders {
    content_type: String,
    token: String,
}
