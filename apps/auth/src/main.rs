use bod_models::{
    schemas::{
        config::{
            reset_token::reset_token::ResetTokenSchema, user_config::user_config::UserConfigSchema,
        },
        location::country::country::CountrySchema,
        mst::user::user::UserSchema,
    },
    shared::schema::Schema,
};
use common::utils::ntex_private::{
    collection::collection::{Collection, CollectionFunctions},
    repository::public_repository::PublicRepository,
};
use modules::authentication::authentication_scope::authentication_route;
use ntex::{
    http,
    web::{self, scope},
};
use ntex_cors::Cors;
pub mod modules;
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
    let user_config: Box<dyn Schema + Sync + Send> = Box::new(UserConfigSchema);
    let reset_token: Box<dyn Schema + Sync + Send> = Box::new(ResetTokenSchema);
    let country: Box<dyn Schema + Sync + Send> = Box::new(CountrySchema);
    let user: Box<dyn Schema + Sync + Send> = Box::new(UserSchema);
    let collections = vec![user_config, reset_token, country, user];
    let collections = Collection::new(client, collections);
    collections.run_indexes().await;

    //aqui nos traemos los repositorios

    web::HttpServer::new(move || {
        web::App::new()
            .wrap(
                Cors::new()
                    .send_wildcard()
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "PATCH"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .max_age(300)
                    .finish(),
            )
            .state(public_repository.clone())
            .service(scope("/auth").configure(authentication_route))
    })
    .bind(("127.0.0.1", 8082))? //TODO poner puerto en envieronment
    .run()
    .await
}
