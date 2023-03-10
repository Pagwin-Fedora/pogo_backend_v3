use lazy_static::lazy_static;
use tokio::runtime::{Handle, Runtime};

lazy_static! {
    static ref RUNTIME: Runtime = Runtime::new().expect("Async runtime creation failed");
}

pub fn get_handle() -> Handle {
    RUNTIME.handle().clone()
}
