use candid::{CandidType, Nat};
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse, TransformArgs,
    TransformContext,
};
use ic_cdk_macros::{query, update};
use serde::{Deserialize, Serialize};

// Derive CandidType to ensure compatibility with canister calls
#[derive(Serialize, Deserialize, CandidType)]
struct ExternalNotification {
    action: String,
    account: String,
    token_id: Nat,
}

#[update]
async fn notify_external_service(notification: ExternalNotification) -> String {
    let url = "https://external-service.com/api/notify";

    let request_headers = vec![
        HttpHeader {
            name: "Content-Type".to_string(),
            value: "application/json".to_string(),
        },
        HttpHeader {
            name: "User-Agent".to_string(),
            value: "bridge_common_layer".to_string(),
        },
    ];

    let json_string = serde_json::to_string(&notification).unwrap();
    let request_body: Option<Vec<u8>> = Some(json_string.into_bytes());

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
            let str_body = String::from_utf8(response.body).expect("Response not UTF-8.");
            str_body
        }
        Err((r, m)) => format!("Request failed. RejectionCode: {r:?}, Error: {m}"),
    }
}

// Function to calculate required cycles
fn http_request_required_cycles(arg: &CanisterHttpRequestArgument) -> u128 {
    let max_response_bytes = arg.max_response_bytes.unwrap_or(2 * 1024 * 1024u64) as u128;
    let arg_raw = candid::utils::encode_args((arg,)).expect("Failed to encode arguments.");
    (3_000_000u128
        + 60_000u128 * 13
        + (arg_raw.len() as u128 + "http_request".len() as u128) * 400
        + max_response_bytes * 800)
        * 13
}
