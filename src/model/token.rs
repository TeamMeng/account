use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct RespToken {
    token: String,
}

impl RespToken {
    pub fn new(token: String) -> Self {
        Self { token }
    }
}
