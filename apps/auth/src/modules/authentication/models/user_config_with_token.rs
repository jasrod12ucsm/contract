use bod_models::schemas::config::user_config::models::user_config_without_password::UserConfigWithoutPassword;
use serde::{Deserialize, Serialize};
#[derive(Deserialize,Serialize,Debug)]
pub struct UserConfigWithToken{
    pub user: UserConfigWithoutPassword,
}