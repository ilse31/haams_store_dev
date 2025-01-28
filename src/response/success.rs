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

    pub fn error(message: &str) -> Self {
        Self {
            success: false,
            message: message.to_string(),
            status_code: 500,
            data: None,
        }
    }

    pub fn unauthorized(message: &str) -> Self {
        Self {
            success: false,
            message: message.to_string(),
            status_code: 401,
            data: None,
        }
    }

    pub fn not_found(message: &str) -> Self {
        Self {
            success: false,
            message: message.to_string(),
            status_code: 404,
            data: None,
        }
    }

    pub fn bad_request(message: &str) -> Self {
        Self {
            success: false,
            message: message.to_string(),
            status_code: 400,
            data: None,
        }
    }

    pub fn forbidden(message: &str) -> Self {
        Self {
            success: false,
            message: message.to_string(),
            status_code: 403,
            data: None,
        }
    }

    pub fn conflict(message: &str) -> Self {
        Self {
            success: false,
            message: message.to_string(),
            status_code: 409,
            data: None,
        }
    }

    pub fn no_content(message: &str) -> Self {
        Self {
            success: true,
            message: message.to_string(),
            status_code: 204,
            data: None,
        }
    }

    pub fn accepted(message: &str) -> Self {
        Self {
            success: true,
            message: message.to_string(),
            status_code: 202,
            data: None,
        }
    }

    pub fn not_modified(message: &str) -> Self {
        Self {
            success: true,
            message: message.to_string(),
            status_code: 304,
            data: None,
        }
    }
}
