use std::string::String;
use candid::{CandidType, Nat};
use serde::{Serialize, Deserialize};
use serde_json::{self};
use std::fmt;
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod,
};

#[derive(CandidType, Serialize, Deserialize)]
struct Message {
    id: String, // Unique identifier for the message
    nonce: u64, // Unique message nonce
    op_type: u8, // Operation type
    src_chain_id: u64, // Source chain id
    dest_chain_id: u64, // Destination chain id
    dest_address: String, // Destination address
    contract_address: String, // Contract address
    token_id: u64, // Token id
    token_metadata: String, // Token metadata
}

#[derive(CandidType, Debug)]
enum ExecuteError {
    MessageNotExecuted,
}

impl fmt::Display for ExecuteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[ic_cdk::update]
fn execute_message(msg: Message) {
    ic_cdk::print(format!("Message received with id: {}", msg.id));
}

#[ic_cdk::update]
async fn send_message() -> Result<(), ExecuteError> {
    let host = "localhost";
    let url = "https://localhost:4942/message";

    let request_headers = vec![
        HttpHeader {
            name: "Host".to_string(),
            value: format!("{host}:443"),
        },
        HttpHeader {
            name: "User-Agent".to_string(),
            value: "demo_HTTP_POST_canister".to_string(),
        },
        HttpHeader {
            name: "Content-Type".to_string(),
            value: "application/json".to_string(),
        },
    ];

    let message = Message {
        id: "1".to_string(),
        nonce: 1,
        op_type: 1,
        src_chain_id: 2,
        dest_chain_id: 1,
        dest_address: "0x0000000000000000000000000000000000000001".to_string(),
        contract_address: "0x0000000000000000000000000000000000000002".to_string(),
        token_id: 1,
        token_metadata: "".to_string()
    };

    let json_string = serde_json::to_string(&message);

    let json_utf8: Vec<u8> = json_string.unwrap().into_bytes();
    let request_body: Option<Vec<u8>> = Some(json_utf8);

    let request = CanisterHttpRequestArgument {
        url: url.to_string(),
        max_response_bytes: None,
        method: HttpMethod::POST,
        headers: request_headers,
        body: request_body,
        transform: None,
    };
    
    let cycles = http_request_required_cycles(&request);

    match http_request(request, cycles).await {

        Ok((response,)) => {
            let str_body = String::from_utf8(response.body)
                .expect("Transformed response is not UTF-8 encoded.");
            ic_cdk::api::print(format!("{:?}", str_body));

            if response.status == Nat::from(200u32) {
                return Ok(())
            } else {
                return Err(ExecuteError::MessageNotExecuted);
            }
        }
        Err((r, m)) => {
            let message =
                format!("The http_request resulted into error. RejectionCode: {r:?}, Error: {m}");
                ic_cdk::api::print(format!("{:?}", message));

            return Err(ExecuteError::MessageNotExecuted);
        }
    }

}

// Calculate required cycles for HTTP request.
fn http_request_required_cycles(arg: &CanisterHttpRequestArgument) -> u128 {
    let max_response_bytes = match arg.max_response_bytes {
        Some(ref n) => *n as u128,
        None => 2 * 1024 * 1024u128, // default 2MiB
    };
    let arg_raw = candid::utils::encode_args((arg,)).expect("Failed to encode arguments.");
    // The fee is for a 13-node subnet to demonstrate a typical usage.
    (3_000_000u128
        + 60_000u128 * 13
        + (arg_raw.len() as u128 + "http_request".len() as u128) * 400
        + max_response_bytes * 800)
        * 13
}