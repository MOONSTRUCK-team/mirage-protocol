use std::cell::RefCell;
use std::string::String;

thread_local! {
    static NAME: RefCell<String> = RefCell::new(String::from("John Doe"));
}

// Assuming BigNumberish is a type alias for u64
type BigNumberish = u64;

// Assuming AddressLike is a type alias for String
type AddressLike = String;

// Assuming BytesLike is a type alias for Vec<u8>
type BytesLike = Vec<u8>;

// #[derive(serde::Deserialize)]
// struct Message {
//     nonce: BigNumberish,
//     src_chain_id: BigNumberish,
//     dest_chain_id: BigNumberish,
//     dest_address: String,
//     contract_address: AddressLike,
//     token_id: BigNumberish,
// }

#[ic_cdk::query]
fn get_name() -> String {
    NAME.with(|name| (*name.borrow()).clone())
}

#[ic_cdk::update]
fn set_name(new_name: String) {
    NAME.with(|name| *name.borrow_mut() = new_name);
}

// #[ic_cdk::update]
// fn execute(id: String, _msg: Message) {
//     ic_cdk::print(format!("Message received with id: {}", id));
// }