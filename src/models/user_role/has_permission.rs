use crate::guards::user_token::UserTokenInfo;
use crate::models::user_role::UserRole;

pub fn has_permission_to_add_roles(user: &UserTokenInfo, roles: &Vec<UserRole>) -> bool {
    for role in roles {
        let can_add = match role {
            UserRole::TaskManager => {
                user.roles.contains(&UserRole::Director) || user.roles.contains(&UserRole::HumanResources)
            }
            UserRole::HumanResources => {
                user.roles.contains(&UserRole::Director)
            }
            UserRole::Director => {
                false
            }
        };
        if !can_add {
            return false;
        }
    }
    return true;
}
