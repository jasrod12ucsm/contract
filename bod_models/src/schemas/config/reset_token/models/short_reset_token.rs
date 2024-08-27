use bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

use super::reset_token_with_id::ResetTokenWithId;

#[derive(Serialize, Deserialize)]
pub struct ShortResetToken {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub token: String,
    #[serde(rename = "userId")]
    pub user_id: ObjectId,
    pub created: DateTime,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
}

impl From<ResetTokenWithId> for ShortResetToken {

    fn from(value: ResetTokenWithId) -> Self {
        Self {
            id: value.id,
            token: value.token,
            user_id: value.user_id,
            created: value.created,
            is_deleted: value.is_deleted,
        }
    }
}
