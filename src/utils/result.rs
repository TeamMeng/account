use crate::STATUS_CODE;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use serde_json::Value::Null;

#[derive(Serialize)]
struct ResponseData<T: Serialize> {
    code: usize,
    message: String,
    data: T,
}

impl<T: Serialize> ResponseData<T> {
    fn new(code: usize, data: T) -> Self {
        let message = STATUS_CODE.get().unwrap().get(&code).unwrap();
        Self {
            code,
            message: message.to_string(),
            data,
        }
    }
}

fn response_json<T: Serialize>(code: usize, data: T) -> Response {
    let rd = ResponseData::new(code, data);
    let mut data = String::from("");

    if let Ok(msg) = serde_json::to_string(&rd) {
        data = msg;
    }

    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", " application/json;charset:utf-8;")
        .body(data)
        .unwrap()
        .into_response()
}

pub fn success<T: Serialize>(data: T) -> Response {
    response_json(1, data)
}

pub fn success_null() -> Response {
    response_json(1, Null)
}

pub fn fail<T: Serialize>(code: usize, data: T) -> Response {
    response_json(code, data)
}

pub fn fail_null(code: usize) -> Response {
    response_json(code, Null)
}
