
use rand::Rng;

use crate::helpers::env::env::ENV;

use super::encryptation_error::EncryptationError;


pub struct PasswordFunctions;
impl PasswordFunctions {
    pub fn hash_password(password: &str) -> Result<String, EncryptationError> {
        let env_salt=ENV.get_array("SALT_KEY").expect("No salt key provided");
        let mut salt:Vec<u8>=vec![];
        for value in env_salt {
            let u8_value=u8::try_from(value.into_int().expect("Data in SALT KEY is not correct")).expect("Hash lecture error");
            salt.push(u8_value);
        }
        argon2::hash_encoded(password.as_bytes(),&salt , &argon2::Config::default())
            .map_err(|_| EncryptationError::Error)
    }
    pub fn verify_password(hash: &str, password: &str) -> Result<bool, EncryptationError> {
        argon2::verify_encoded(hash, password.as_bytes())
            .map_err(|_| EncryptationError::Error)
    }
    pub fn generate_random_number()-> i32{
        //de 6 digitos el numero
        let mut rng = rand::thread_rng();
        let random_number: i32 = rng.gen_range(100000..=999999);
        return random_number
    }
}
