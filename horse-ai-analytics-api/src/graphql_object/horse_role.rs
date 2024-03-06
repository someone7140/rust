use async_graphql::*;

use crate::struct_def::common_struct;

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
        if Some(&self.role) == Some(&Role::Anonymous) {
            Ok(())
        } else if let Some(_) = ctx.data_opt::<common_struct::AuthContext>() {
            Ok(())
        } else {
            Err("Forbidden".into())
        }
    }
}
