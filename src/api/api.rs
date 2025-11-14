

use reqwest::{Client, Error}; 

use serde_json::json; 
use serde_json::Value;

/* 

Aggiornare url per il localhost:3000 con uno custom

*/

async fn send_data(uri: String, params: Value) -> Result<(), Error> {

    let client = Client::new();

    let url = format!("http://localhost:3000/api/{}", uri); 

    let resp = client.post(&url)
        .json(&params)
        .send()
        .await?;

    if resp.status().is_success() {
        Ok(())
    } else {
        Err(reqwest::Error::new(
            reqwest::StatusCode::BAD_RAQUEST,
            "Request failed"
        ))
    }
    

}


