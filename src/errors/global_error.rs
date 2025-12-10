use actix_web::{HttpResponse, ResponseError, http::StatusCode, Responder};
use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub struct ErrorResponseBody {
    pub code: u16,
    pub message: String,
    pub status: u16,
    pub data: String,
}

#[derive(Debug)]
pub enum GlobalError {
    CustomInvalidRequest(u16, Option<String>),
    CustomBadRequest(u16, Option<String>),
    CustomInternalServerError(u16, Option<String>),
}

impl fmt::Display for GlobalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            GlobalError::CustomInvalidRequest(_, Some(m)) => m,
            GlobalError::CustomBadRequest(_, Some(m)) => m,
            GlobalError::CustomInternalServerError(_, Some(m)) => m,
            GlobalError::CustomInvalidRequest(_, None) => "Invalid request",
            GlobalError::CustomBadRequest(_, None) => "Bad request",
            GlobalError::CustomInternalServerError(_, None) => "Internal server error",
        };
        write!(f, "{}", msg)
    }
}

impl ResponseError for GlobalError {
    fn status_code(&self) -> StatusCode {
        match self {
            GlobalError::CustomInvalidRequest(code, _)
            | GlobalError::CustomBadRequest(code, _)
            | GlobalError::CustomInternalServerError(code, _) => {
                StatusCode::from_u16(*code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }

    fn error_response(&self) -> HttpResponse {
        let (code, message) = match self {
            GlobalError::CustomInvalidRequest(code, Some(m))
            | GlobalError::CustomBadRequest(code, Some(m))
            | GlobalError::CustomInternalServerError(code, Some(m)) => (*code, m.clone()),

            GlobalError::CustomInvalidRequest(code, None) => (*code, "Invalid request".to_string()),
            GlobalError::CustomBadRequest(code, None) => (*code, "Bad request".to_string()),
            GlobalError::CustomInternalServerError(code, None) => (*code, "Internal server error".to_string()),
        };

        let body = ErrorResponseBody {
            code,
            message: message.clone(),
            status: code,
            data: message,
        };

        HttpResponse::build(self.status_code()).json(body)
    }
}


impl GlobalError {
    pub fn invalid_request(code: Option<u16>, msg: Option<String>) -> Self {
        println!("GlobalError::invalid_request: {:?}-{:?}", code, msg);
        Self::CustomInvalidRequest(
            code.unwrap_or(422),
            Some(msg.unwrap_or_else(|| "Invalid request".to_string())),
        )
    }

    pub fn bad_request(code: Option<u16>, msg: Option<String>) -> Self {
        println!("GlobalError::bad_request: {:?}-{:?}", code, msg);
        Self::CustomBadRequest(
            code.unwrap_or(400),
            Some(msg.unwrap_or_else(|| "Bad request".to_string())),
        )
    }

    pub fn internal_error(code: Option<u16>, msg: Option<String>) -> Self {
        println!("GlobalError::internal_error: {:?}-{:?}", code, msg);
        Self::CustomInternalServerError(
            code.unwrap_or(500),
            Some(msg.unwrap_or_else(|| "Internal server error".to_string())),
        )
    }

    pub fn unauthorized_error(code: Option<u16>, msg: Option<String>) -> Self {
        println!("GlobalError::unauthorized_error: {:?}-{:?}", code, msg);
        Self::CustomInternalServerError(
            code.unwrap_or(500),
            Some(msg.unwrap_or_else(|| "Unauthorized Access".to_string())),
        )
    }

    pub fn not_found_error(code: Option<u16>, msg: Option<String>) -> Self {
        println!("GlobalError::not_found: {:?}-{:?}", code, msg);
        Self::CustomInvalidRequest(
            code.unwrap_or(404),
            Some(msg.unwrap_or_else(|| "Not Found".to_string())),
        )
    }

    pub fn conflict_request(code: Option<u16>, msg: Option<String>) -> Self {
        println!("GlobalError::conflict_request: {:?}-{:?}", code, msg);
        Self::CustomBadRequest(
            code.unwrap_or(409),
            Some(msg.unwrap_or_else(|| "Conflict request".to_string())),
        )
    }
}
