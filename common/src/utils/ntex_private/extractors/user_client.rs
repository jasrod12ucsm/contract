use core::fmt;
use std::{fmt::Debug, ops};

use bod_models::schemas::mst::user::{models::user_with_id::UserWithId, user_errors::UserError};
use ntex::web::{ErrorRenderer, FromRequest};



pub struct UserClient(pub UserWithId);

impl UserClient {
    pub fn into_inner(self) -> UserWithId {
        self.0
    }
}

impl ops::Deref for UserClient {
    type Target = UserWithId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for UserClient {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Debug for UserClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Json").field(&self.0).finish()
    }
}

impl fmt::Display for UserClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl<Err: ErrorRenderer> FromRequest<Err> for UserClient {
    type Error = UserError;

    async fn from_request(
        req: &ntex::web::HttpRequest,
        _payload: &mut ntex::http::Payload,
    ) -> Result<Self, Self::Error> {
        //traer los extension mut
        let ext = req.extensions();
        let id = ext.get::<UserWithId>();
        //programming funcional
        match id {
            Some(user) => {
                return Ok(UserClient(user.clone()));
            }
            None => {
                return Err(UserError::GetUserError("not token provided"));
            }
        }
    }
}
