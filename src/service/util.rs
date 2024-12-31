use axum::response::{IntoResponse, Response};
use serde::Serialize;

#[derive(Serialize)]
pub struct MyResponse<T> {
    pub error_code: i32,
    pub err_msg: String,
    pub data: T,
}

impl<T: Serialize> IntoResponse for MyResponse<T> {
    fn into_response(self) -> Response {
        serde_json::to_string(&self).unwrap().into_response()
    }
}

pub fn resp_success<T: Serialize>(data: T) -> MyResponse<T> {
    MyResponse {
        error_code: 0,
        err_msg: "success".to_string(),
        data,
    }
}

pub fn resp_error<T>(error_code: i32, err_msg: String) -> MyResponse<Option<T>> {
    MyResponse {
        error_code,
        err_msg,
        data: None,
    }
}
