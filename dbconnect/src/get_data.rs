use reqwest;
use serde_json::Value;
use super::api_response::ApiResponse;
use super::get_config;

pub async fn get_rec(id: String) -> Result<ApiResponse, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let query = [("rec_id", id)];
    
    let response = client
        .get(get_config())
        .header("Accept", "application/json")
        .query(&query)
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
        _ => {
            let error: ApiResponse = response.json().await?;
            eprintln!("Error {}: {:?}", error.status, error);
            Ok(error)
        }
    }
}