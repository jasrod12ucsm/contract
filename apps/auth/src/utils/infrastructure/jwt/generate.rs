use std::time::{SystemTime, UNIX_EPOCH};


use bod_models::shared::jwt::claims::{DefaultClaims, RenewClaims};
use bson::oid::ObjectId;
use common::helpers::env::env::ENV;
use jsonwebtoken::{
    encode,
    errors::{Error, ErrorKind},
    Algorithm, EncodingKey, Header,
};


pub  fn generate_jwt(uid: ObjectId) -> Result<String, Error> {
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

pub fn generate_refresh_jwt(os:&str,id:ObjectId) -> Result<String, Error> {
    //hora actual
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .map_err(|_| Error::from(ErrorKind::ImmatureSignature))?;
    let current_time = since_the_epoch.as_secs();
    //obtener el usuario
    //trarmos el claim
    let default_claim = RenewClaims::new((current_time + 86400) as usize,os.to_string(),id);
    //crear el header del token
    let header = Header::new(Algorithm::HS256);
    let secret_key = ENV
        .get_string("SECRET_KEY_REFRESH")
        .map_err(|_| Error::from(ErrorKind::InvalidKeyFormat))?;
    //crear el token
    let token = encode(
        &header,
        &default_claim,
        &EncodingKey::from_secret(secret_key.as_str().as_ref()),
    )?;
    Ok(token)
}
