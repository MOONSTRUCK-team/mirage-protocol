use candid::CandidType;
use ic_cdk::api::call::RejectionCode;
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, CandidType, Clone)]
pub struct Message {
    pub id: String,
    pub nonce: u64,
    pub op_type: u8,
    pub src_chain_id: u64,
    pub dest_chain_id: u64,
    pub dest_address: String,
    pub contract_address: String,
    pub token_id: u64,
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub enum ExecuteError {
    HttpError {
        rejection_code: RejectionCode,
        message: String,
    },
}

// Function to notify external service about an event
pub async fn notify_external_service(msg: Message) -> Result<(), ExecuteError> {
    // Replace with the actual URL of the external service
    let url = "https://your.external.service/notify";
    let body = json!(msg).to_string();

    // Prepare HTTP headers
    let headers = vec![HttpHeader {
        name: "Content-Type".to_string(),
        value: "application/json".to_string(),
    }];

    // Prepare the HTTP request
    let request = CanisterHttpRequestArgument {
        url: url.to_string(),
        method: HttpMethod::POST,
        body: Some(body.into_bytes()),
        headers,
        max_response_bytes: Some(1024 * 1024), // Example max response size
        ..Default::default()
    };

    // Send the HTTP request and handle the response
    match http_request(request, 0).await {
        Ok(_) => {
            // Log success (optional)
            ic_cdk::println!("Successfully notified external service");
            Ok(())
        }
        Err((rejection_code, error_message)) => {
            // Log the error (optional)
            ic_cdk::println!("Failed to notify external service: {:?}", error_message);
            Err(ExecuteError::HttpError {
                rejection_code,
                message: error_message.to_string(),
            })
        }
    }
}
