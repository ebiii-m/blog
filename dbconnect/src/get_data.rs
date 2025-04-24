use core::sync;

use reqwest;
use serde_json::Value;
use super::get_config;
use super::api_response::ApiResponse;

pub async fn get_rec(id: String) -> ApiResponse {
    let client = reqwest::Client::new();
    let query = [(String::from("rec_id"), id)];
    let response = client.get(get_config())
            .query(&query)
            .send()
            .await?;
    
    

}