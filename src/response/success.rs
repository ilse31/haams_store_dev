use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    pub status_code: u16,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T, message: &str) -> Self {
        Self {
            success: true,
            message: message.to_string(),
            status_code: 200,
            data: Some(data),
        }
    }

    pub fn created(data: T, message: &str) -> Self {
        Self {
            success: true,
            message: message.to_string(),
            status_code: 201,
            data: Some(data),
        }
    }
}
