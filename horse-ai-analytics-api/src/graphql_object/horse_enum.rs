use async_graphql::*;

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum ErrorType {
    SystemError,
    AuthError,
}
