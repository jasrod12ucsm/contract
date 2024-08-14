use std::time::{SystemTime, UNIX_EPOCH};

use bod_models::{schemas::mst::user::models::user_with_id::UserWithId, shared::jwt::claims::DefaultClaims};

use common::helpers::env::env::ENV;
use jsonwebtoken::{
    encode,
    errors::{Error, ErrorKind},
    Algorithm, EncodingKey, Header,
};


pub  fn generate_jwt(uid: UserWithId) -> Result<String, Error> {
    //hora actual
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .map_err(|_| Error::from(ErrorKind::ImmatureSignature))?;
    let current_time = since_the_epoch.as_secs();
    //obtener el usuario
    //trarmos el claim
    let default_claim = DefaultClaims::new((current_time + 86400) as usize,uid);
    //crear el header del token
    let header = Header::new(Algorithm::HS256);
    let secret_key = ENV
        .get_string("SECRET_KEY")
        .map_err(|_| Error::from(ErrorKind::InvalidKeyFormat))?;
    //crear el token
    let token = encode(
        &header,
        &default_claim,
        &EncodingKey::from_secret(secret_key.as_str().as_ref()),
    )?;
    Ok(token)
}
