use crate::icrc7::MintArgs;
use crate::manager::token_mint; // Import the `token_mint` function directly
use candid::Nat;
use candid::{CandidType, Principal};
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod,
};
use ic_cdk_macros::update;

use serde::{Deserialize, Serialize};

// Define the Message struct, representing a message from the external bridge
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

// Function to fetch a message from the external bridge service and mint a token
#[update]
pub async fn process_bridge_message() -> Result<Nat, String> {
    // Define the URL of the external service that provides the message
    let url = "https://our.external.service/get_message"; // Replace with actual URL

    // Set the necessary HTTP headers, specifying that the content is JSON
    let headers = vec![HttpHeader {
        name: "Content-Type".to_string(),
        value: "application/json".to_string(),
    }];

    // Prepare the HTTP GET request to fetch the message
    let request = CanisterHttpRequestArgument {
        url: url.to_string(),                  // Set the request URL
        method: HttpMethod::GET,               // Use the GET method to retrieve data
        headers,                               // Include the content-type header
        max_response_bytes: Some(1024 * 1024), // Set maximum allowed response size to 1 MB
        ..Default::default()                   // Use default values for other fields
    };

    // Send the HTTP request and wait for the response asynchronously
    match http_request(request, 1_000_000_000).await {
        Ok((response,)) => {
            // If the request succeeds
            // Attempt to deserialize the response body into a Message struct
            match serde_json::from_slice::<Message>(&response.body) {
                Ok(msg) => {
                    // If deserialization succeeds
                    // Convert the Message into MintArgs to use with the mint function
                    let mint_args = MintArgs {
                        to: crate::icrc7::Account {
                            owner: Principal::from_text(msg.dest_address).unwrap(), // Convert dest_address to Principal
                            subaccount: None,                                       // No subaccount
                        },
                        token_id: Nat::from(msg.token_id), // Convert token_id to Nat
                        metadata: vec![],                  // No metadata provided
                    };

                    // Call the token_mint function directly
                    match token_mint(mint_args).await {
                        Ok(token_id) => Ok(token_id), // Return the token ID if successful
                        Err(err) => Err(format!("Failed to mint token: {:?}", err)), // Return error if minting fails
                    }
                }
                Err(e) => Err(format!("Failed to parse message: {:?}", e)), // Return error if deserialization fails
            }
        }
        // Handle any errors that occur during the HTTP request
        Err((rejection_code, error_message)) => Err(format!(
            "Failed to fetch message from bridge: {:?} - {:?}",
            rejection_code, error_message
        )),
    }
}
