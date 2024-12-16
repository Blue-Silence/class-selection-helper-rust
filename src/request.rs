use reqwest::Client;
use serde_json::json;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

pub async fn send_request(ciphertext: String, checkcode: String, x_token: String) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let url = "https://1.tongji.edu.cn/api/electionservice/student/elect";
    let body = json!({
        "ciphertext": ciphertext,
        "checkCode": checkcode
    });

    let mut headers = HeaderMap::new();
    headers.insert(HeaderName::from_static("accept"), HeaderValue::from_static("application/json, text/plain, */*"));
    headers.insert(HeaderName::from_static("accept-language"), HeaderValue::from_static("zh-CN,zh;q=0.9"));
    headers.insert(HeaderName::from_static("content-type"), HeaderValue::from_static("application/json;charset=UTF-8"));
    headers.insert(HeaderName::from_static("priority"), HeaderValue::from_static("u=1, i"));
    headers.insert(HeaderName::from_static("sec-ch-ua"), HeaderValue::from_static("\"Google Chrome\";v=\"131\", \"Chromium\";v=\"131\", \"Not_A Brand\";v=\"24\""));
    headers.insert(HeaderName::from_static("sec-ch-ua-mobile"), HeaderValue::from_static("?0"));
    headers.insert(HeaderName::from_static("sec-ch-ua-platform"), HeaderValue::from_static("\"Windows\""));
    headers.insert(HeaderName::from_static("sec-fetch-dest"), HeaderValue::from_static("empty"));
    headers.insert(HeaderName::from_static("sec-fetch-mode"), HeaderValue::from_static("cors"));
    headers.insert(HeaderName::from_static("sec-fetch-site"), HeaderValue::from_static("same-origin"));
    headers.insert(HeaderName::from_static("x-kl-ajax-request"), HeaderValue::from_static("Ajax_Request"));
    headers.insert(HeaderName::from_static("x-token"), HeaderValue::from_str(&x_token)?);

    let response = client
        .post(url)
        .headers(headers.clone())
        .body(body.to_string())
        .send()
        .await?;

    println!("Response: {:?}", response.text().await?);

    Ok(())
} 