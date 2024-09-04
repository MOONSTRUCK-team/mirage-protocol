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
    pub token_metadata: String,
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
    // Should be replaced with the actual URL of the external service
    let url = "https://our.external.service/notify";
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
        max_response_bytes: Some(1024 * 1024), //  max response size
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

// Function to fetch the message from the external bridge service
pub async fn get_message_from_bridge() -> Result<Message, ExecuteError> {
    // URL of the external service from where to fetch the message
    let url = "https://our.external.service/get_message"; // Replace with actual URL

    // Prepare HTTP headers (if necessary)
    let headers = vec![HttpHeader {
        name: "Content-Type".to_string(),
        value: "application/json".to_string(),
    }];

    // Prepare the HTTP GET request
    let request = CanisterHttpRequestArgument {
        url: url.to_string(),
        method: HttpMethod::GET,
        headers,
        max_response_bytes: Some(1024 * 1024), // Max response size
        ..Default::default()
    };

    // Send the HTTP request
    match http_request(request, 0).await {
        Ok((response,)) => {
            // Extract the tuple response
            // Parse the body into a Message struct
            match serde_json::from_slice::<Message>(&response.body) {
                Ok(msg) => Ok(msg),
                Err(e) => Err(ExecuteError::HttpError {
                    rejection_code: RejectionCode::Unknown,
                    message: format!("Failed to parse message: {:?}", e),
                }),
            }
        }
        Err((rejection_code, error_message)) => Err(ExecuteError::HttpError {
            rejection_code,
            message: error_message.to_string(),
        }),
    }
}