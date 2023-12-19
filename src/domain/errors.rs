pub struct Error {
    pub message: String,
    pub status_code: StatusCode,
}

#[derive(Eq, PartialEq, Debug)]
pub enum StatusCode {
    NotFound,
    Unauthorized,
    InternalServerError,
    Conflict,
    BadRequest,
}

impl Error {
    pub fn not_found(message: String) -> Self {
        Self {
            message,
            status_code: StatusCode::NotFound,
        }
    }

    pub fn unauthorized(message: String) -> Self {
        Self {
            message,
            status_code: StatusCode::Unauthorized,
        }
    }

    pub fn internal_server_error(message: String) -> Self {
        Self {
            message,
            status_code: StatusCode::InternalServerError,
        }
    }

    pub fn conflict(message: String) -> Self {
        Self {
            message,
            status_code: StatusCode::Conflict,
        }
    }

    pub fn bad_request(message: String) -> Self {
        Self {
            message,
            status_code: StatusCode::BadRequest,
        }
    }
}

impl StatusCode {
    pub fn get_code(status_code: StatusCode) -> i32 {
        match status_code {
            StatusCode::NotFound => 404,
            StatusCode::Unauthorized => 401,
            StatusCode::InternalServerError => 500,
            StatusCode::Conflict => 409,
            StatusCode::BadRequest => 400,
        }
    }
}
