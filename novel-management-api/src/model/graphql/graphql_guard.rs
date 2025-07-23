use async_graphql::*;

use crate::model::{common::context_info::AuthContext, graphql::graphql_error::AppError};

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Role {
    User,
    Anonymous,
}

pub struct RoleGuard {
    pub role: Role,
}

impl RoleGuard {
    pub fn new(role: Role) -> Self {
        Self { role }
    }
}

impl Guard for RoleGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        // Roleの使い分けは適当に・・
        if Some(&self.role) == Some(&Role::Anonymous) {
            Ok(())
        } else if let Some(_) = ctx.data_opt::<AuthContext>() {
            Ok(())
        } else {
            Err(AppError::AuthorizationError("Auth guard error".to_string()).extend())
        }
    }
}
