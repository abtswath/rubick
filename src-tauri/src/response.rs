use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Response<T> {
    pub code: i32,
    pub message: String,
    pub data: T,
}

impl<T> Response<T> {
    pub fn new(code: i32, message: &str, data: T) -> Response<T> {
        Response {
            code,
            message: message.to_string(),
            data,
        }
    }

    pub fn ok(message: &str, data: T) -> Self {
        Self::new(0, message, data)
    }

    pub fn fail(message: &str, data: T) -> Self {
        Self::new(99999, message, data)
    }
}
