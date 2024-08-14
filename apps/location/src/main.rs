use bod_models::{schemas::location::{
    country::country::CountrySchema, region::region::RegionSchema,
}, shared::schema::Schema};
use common::utils::ntex_private::{
        collection::collection::{Collection, CollectionFunctions},
        repository::public_repository::PublicRepository,
    };
use modules::{loc_country::loc_country_scope::loc_country_scope, loc_region::loc_region_scope};
use ntex::web::{self, scope};

pub mod modules;
pub mod public;
pub mod utils;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    //crear rpatron repository
    let public_repository = PublicRepository::connect()
        .await
        .map_err(|_| panic!())
        .unwrap();
    //*creamos los indices y schemas necesarios al ejecutar la app se cargaran otra vez
    let client = public_repository.get_client().unwrap();
    let country: Box<dyn Schema + Sync + Send> = Box::new(CountrySchema);
    let region: Box<dyn Schema + Sync + Send> = Box::new(RegionSchema);
    let collections = vec![country, region];
    let collections = Collection::new(client, collections);
    collections.run_indexes().await;

    //aqui nos traemos los repositorios

    web::HttpServer::new(move || {
        web::App::new()
            .state(public_repository.clone())
            .service(scope("/country").configure(loc_country_scope))
            .service(scope("/region").configure(loc_region_scope::loc_region_scope))
    })
    .bind(("127.0.0.1", 8080))? //TODO poner puerto en envieronment
    .run()
    .await
}
