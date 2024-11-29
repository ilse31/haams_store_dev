use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ResponseAPI<T> {
    pub success: bool,
    pub message: String,
    pub status_code: u16,
    pub data: Option<T>,
}

#[derive(Debug, Serialize)]
pub struct TableResponseAPI<T> {
    pub success: bool,
    pub message: String,
    pub status_code: u16,
    pub data: Vec<T>,
    pub total: u64,
}

impl<T> TableResponseAPI<T> {
    pub fn success(data: Vec<T>, total: u64, message: &str) -> Self {
        Self {
            success: true,
            message: message.to_string(),
            status_code: 200,
            data,
            total,
        }
    }
}

impl<T> ResponseAPI<T> {
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
