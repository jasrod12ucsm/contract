use bod_models::{schemas::config::card_plan::card_plan::CardPlanSchema, shared::schema::Schema};
use common::{
    helpers::{env::env::ENV, ip::ip_functions::IpFunctions},
    utils::ntex_private::{
        collection::collection::{Collection, CollectionFunctions},
        repository::public_repository::PublicRepository,
    },
};
use modules::card_plan::card_plan_scope::card_plan_scope;
use ntex::web::{self, scope};
use ntex_cors::Cors;
use utils::{domain::datasources::culqi_datasource_trait::CulqiDataSourceTrait, infraestructure::datasources::culqi_datasource::CulqiDatasource};
pub mod modules;
pub mod utils;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    let port = ENV.get_int("CONFIGURATION_PORT").expect("not port sended") as u16;

    {
        let _ = CulqiDatasource::new();
    }

    //crear rpatron repository
    let public_repository = PublicRepository::connect()
        .await
        .map_err(|_| panic!())
        .unwrap();
    //*creamos los indices y schemas necesarios al ejecutar la app se cargaran otra vez
    let client = public_repository.get_client().unwrap();

    let card_plan: Box<dyn Schema + Sync + Send> = Box::new(CardPlanSchema);

    let collections = vec![card_plan];
    let collections = Collection::new(client, collections);
    collections.run_indexes().await;
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
            .service(scope("/card-plan").configure(card_plan_scope))
    })
    .bind((ipv4, port))? //TODO poner puerto en envieronment
    .run()
    .await
}
