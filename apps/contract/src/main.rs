use std::fs;

use bod_models::{schemas::mst::user::user::UserSchema, shared::schema::Schema};
use common::{
    helpers::{env::env::ENV, ip::ip_functions::IpFunctions},
    utils::ntex_private::{
        collection::collection::{Collection, CollectionFunctions},
        repository::public_repository::PublicRepository,
    },
};
use ed25519_dalek::{SigningKey, SECRET_KEY_LENGTH};
use modules::contract::contract_scope::contract_scope;
use ntex::web::{self, scope};
use ntex_cors::Cors;
use lazy_static::lazy_static;
pub mod modules;
pub mod utils;

use rsa::{pkcs1::DecodeRsaPrivateKey, RsaPrivateKey};
use tzf_rs::DefaultFinder;

lazy_static! {
    static ref FINDER: DefaultFinder = DefaultFinder::new();
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    let port = ENV.get_int("CONTRACT_PORT").expect("not port sended") as u16;

    //crear rpatron repository
    let public_repository = PublicRepository::connect()
        .await
        .map_err(|_| panic!())
        .unwrap();
      // Generar un par de claves (privada y pública)
      println!("Generando par de claves...");
      println!("{}",SECRET_KEY_LENGTH);
       //let cifrade_key: [u8; 32] = [
    //     157, 097, 177, 157, 239, 253, 090, 096,
    //     186, 132, 074, 244, 146, 236, 044, 196,
    //     068, 073, 197, 105, 123, 050, 105, 025,
    //     112, 059, 172, 003, 028, 174, 127, 096,
    // ];
    let pem=fs::read_to_string("private_key.pem").expect("no file");
    let private_key=RsaPrivateKey::from_pkcs1_pem(&pem).expect("no key");
    //*creamos los indices y schemas necesarios al ejecutar la app se cargaran otra vez
    let client = public_repository.get_client().unwrap();
    let user_schema: Box<dyn Schema + Sync + Send> = Box::new(UserSchema);
    let collections = vec![user_schema];
    let collections = Collection::new(client, collections);
    collections.run_indexes().await;
    let ipv4 = IpFunctions::get_local_ipv4().expect("no ip").to_string();
    //aqui nos traemos los repositorios
    println!("Server initialized on port {} and ip {}", port, ipv4);
    web::HttpServer::new(move || {
        web::App::new()
            .wrap(
                Cors::new()
                    .allowed_origin("*")
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "PATCH"])
                    .max_age(300)
                    .finish(),
            )
            .state(public_repository.clone()).state(private_key.clone())
            .service(scope("/contract").configure(contract_scope))
    })
    .bind(("0.0.0.0", port))? //TODO poner puerto en envieronment
    .run()
    .await
}
