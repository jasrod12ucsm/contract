use common::{
    helpers::{env::env::ENV, ip::ip_functions::IpFunctions},
    utils::ntex_private::repository::public_repository::PublicRepository,
};
use ntex::web::{self};
use ntex_cors::Cors;
pub mod modules;
pub mod utils;
pub mod public;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    let port = ENV.get_int("PAYMENT_PORT").expect("not port sended") as u16;

    //crear rpatron repository
    let public_repository = PublicRepository::connect()
        .await
        .map_err(|_| panic!())
        .unwrap();
    //*creamos los indices y schemas necesarios al ejecutar la app se cargaran otra vez
    let _client = public_repository.get_client().unwrap();

    let ipv4 = IpFunctions::get_local_ipv4().expect("no ip").to_string();
    //aqui nos traemos los repositorios
    //imprime el puerto y la ip
    println!("http://{}:{}", ipv4, port);

    web::HttpServer::new(move || {
        web::App::new()
            .wrap(
                Cors::new()
                    .send_wildcard()
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "PATCH"])
                    .max_age(300)
                    .finish(),
            )
            .state(public_repository.clone())
    })
    .bind((ipv4, port))? //TODO poner puerto en envieronment
    .run()
    .await
}
