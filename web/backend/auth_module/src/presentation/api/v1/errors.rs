use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use rust_kratos::domain::errors::{AuthError, DomainError};
use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub struct ErrorBody {
    pub error: String,
    pub message: String,
}

#[derive(Debug)]
pub enum ApiError {
    Validation(String),
    Unauthorized(String),
    Forbidden(String),
    NotFound(String),
    Conflict(String),
    TooManyRequests(String),
    Internal(String),
}

impl ApiError {
    pub fn validation(msg: impl Into<String>) -> Self {
        Self::Validation(msg.into())
    }

    pub fn from_domain(err: DomainError) -> Self {
        match err {
            DomainError::Auth(AuthError::NotAuthenticated) => {
                Self::Unauthorized("Not authenticated".into())
            }
            DomainError::Auth(AuthError::SessionExpired) => {
                Self::Unauthorized("Session expired".into())
            }
            DomainError::Auth(AuthError::AlreadyLoggedIn) => {
                Self::Conflict("Already logged in".into())
            }
            DomainError::Auth(AuthError::InvalidCredentials) => {
                Self::Unauthorized("Invalid credentials".into())
            }
            DomainError::Auth(AuthError::PrivilegedSessionRequired) => {
                Self::Forbidden("Privileged session required".into())
            }
            DomainError::Auth(AuthError::Forbidden) => Self::Forbidden("Forbidden".into()),
            DomainError::Auth(AuthError::AccountDisabled) => {
                Self::Forbidden("Account disabled".into())
            }
            DomainError::Auth(AuthError::TooManyAttempts) => {
                Self::TooManyRequests("Too many attempts".into())
            }
            DomainError::Conflict(msg) => Self::Conflict(msg),
            DomainError::NotFound(msg) => Self::NotFound(msg),
            DomainError::InvalidData(msg) => Self::Validation(msg),
            DomainError::ServiceUnavailable(msg) => Self::Internal(msg),
            DomainError::Internal(msg) => Self::Internal(msg),
        }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Validation(msg) => write!(f, "{}", msg),
            Self::Unauthorized(msg) => write!(f, "{}", msg),
            Self::Forbidden(msg) => write!(f, "{}", msg),
            Self::NotFound(msg) => write!(f, "{}", msg),
            Self::Conflict(msg) => write!(f, "{}", msg),
            Self::TooManyRequests(msg) => write!(f, "{}", msg),
            Self::Internal(msg) => write!(f, "{}", msg),
        }
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Validation(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Self::Forbidden(_) => StatusCode::FORBIDDEN,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Conflict(_) => StatusCode::CONFLICT,
            Self::TooManyRequests(_) => StatusCode::TOO_MANY_REQUESTS,
            Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let code = self.status_code();
        let body = ErrorBody {
            error: code.canonical_reason().unwrap_or("error").to_string(),
            message: self.to_string(),
        };
        HttpResponse::build(code).json(body)
    }
}
