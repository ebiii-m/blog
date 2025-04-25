use reqwest;
use serde_json::{ json , Value };
use serde::Serialize;
use super::api_response::ApiResponse;
use super::get_config;

pub async fn post<T: Serialize>(data: T) -> Result<ApiResponse, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let query = json!({
        "data": data
    });
    let response = client
        .post(get_config())
        .json(&query)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let mut api_response: ApiResponse = response.json().await?;

            if api_response.data.is_none() {
                api_response = ApiResponse {
                    status: "error".to_string(),
                    message: Some("no data detected".to_string()),
                    rec_id: Some("None".to_string()),
                    data: Some(Value::Null),
                };
            }

            Ok(api_response)
        },
        reqwest::StatusCode::BAD_REQUEST => {
            let error: ApiResponse = response.json().await?;
            eprintln!("error 400: {:?}", error);
            Ok(error)
        },
        reqwest::StatusCode::NOT_FOUND => {
            let error: ApiResponse = response.json().await?;
            eprintln!("error 404: {:?}", error);
            Ok(error)
        },
        _ => {
            let error: ApiResponse = response.json().await?;
            eprintln!("error 500: {:?}", error);
            Ok(error)
        }
    }
}