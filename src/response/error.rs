use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum Code {
    // Auth related errors (1000-1999)
    InvalidCredentials = 1001,
    TokenExpired = 1002,
    InvalidToken = 1003,
    UserNotFound = 1004,
    UserAlreadyExists = 1005,

    // Validation errors (2000-2999)
    ValidationError = 2001,
    InvalidInput = 2002,

    // Database errors (3000-3999)
    DatabaseError = 3001,
    ConnectionError = 3002,

    // Server errors (4000-4999)
    InternalServerError = 4001,
    ServiceUnavailable = 4002,

    // Business logic errors (5000-5999)
    InvalidOperation = 5001,
    ResourceNotFound = 5002,
    ResourceAlreadyExists = 5003,
}
