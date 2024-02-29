use async_graphql::*;

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum ErrorType {
    AuthError,
    AlreadyExistsError,
    SystemError,
}
