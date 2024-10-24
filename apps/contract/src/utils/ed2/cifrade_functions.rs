use common::helpers::env::env::ENV;
use hex::decode;

pub fn descifre_env_variable() -> [u8; 32] {
    let cifrade = ENV.get_string("CIF_KEY").expect("No hay clave de cifrado");
    let cifrade_bytes = decode(cifrade).expect("No se pudo decodificar la clave de cifrado");
    let mut cifrade_array = [0u8; 32];
    cifrade_array.copy_from_slice(&cifrade_bytes[..32]);

    cifrade_array
    
}
