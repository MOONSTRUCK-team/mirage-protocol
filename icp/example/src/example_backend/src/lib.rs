use std::cell::RefCell;

thread_local! {
    static NAME: RefCell<String> = RefCell::new(String::from("John Doe"));
}

#[ic_cdk::query]
fn get_name() -> String {
    NAME.with(|name| (*name.borrow()).clone())
}

#[ic_cdk::update]
fn set_name(new_name: String) {
    NAME.with(|name| *name.borrow_mut() = new_name);
}
