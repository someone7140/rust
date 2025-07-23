use async_graphql::{Error, ErrorExtensions};

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Auth Error: {0}")]
    AuthorizationError(String),
    #[error("Forbidden Error: {0}")]
    ForbiddenError(String),
    #[error("NotFound Error: {0}")]
    NotFoundError(String),
    #[error("System error: {0}")]
    SystemError(String),
}

impl ErrorExtensions for AppError {
    fn extend(&self) -> Error {
        Error::new(format!("{}", self)).extend_with(|_err, e| match self {
            AppError::AuthorizationError(_) => e.set("code", 401),
            AppError::ForbiddenError(_) => e.set("code", 403),
            AppError::NotFoundError(_) => e.set("code", 404),
            AppError::SystemError(_) => e.set("code", 500),
        })
    }
}
