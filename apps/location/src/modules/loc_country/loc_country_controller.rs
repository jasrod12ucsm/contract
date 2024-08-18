use bod_models::schemas::location::country::{
    country::Country, country_attributes::CountryAttributes, country_errors::CountryError,
    models::country_with_id::CountryWithId,
};
use bson::{doc, oid::ObjectId};
use common::{
    helpers::env::env::ENV,
    public::models::path::IdPath,
    utils::ntex_private::{
        extractors::json::JsonAdvanced,
        repository::public_repository::{AbstractRepository, PublicRepository},
    },
};
use futures::StreamExt;
use ntex::web::{
    self,
    types::{Path, State},
};

use crate::{
    public::models::{rest_country::RestCountries, secret_path::SecretPath},
    utils::{
        country_repository::CountryRepository, domain::rest_countries_datasource_trait::RestCountriesDataSourceTrait, infraestructure::{
            datasource::rest_countries_data_source::RestCountriesDataSource,
            repositories::rest_countries_repository::RestCountriesRepository,
        }, region_repository::RegionRepository, user_repository::UserRepository
    },
};

//*este solo lo uso para crear un country */
//*posdt::tambien actualiza con informacion actual */
#[web::get("create/{name}/{secret}")]
pub async fn create_country(
    name: Path<SecretPath>,
    repo: State<PublicRepository>,
) -> Result<JsonAdvanced<CountryWithId>, CountryError> {
    let SecretPath { name, secret } = name.into_inner();
    //traer secret password de las variables de entorno
    let secret_password_env = ENV
        .get_string("SECRET_ADMIN_PASSWORD")
        .map_err(|_| CountryError::CreateCountryError("admin not exist"))?;
    //deja pasar si el secret password es correcto
    if secret != secret_password_env {
        return Err(CountryError::CreateCountryError(
            "not coincide admin credentials",
        ));
    }
    //traemos el country de restcountries
    //creamos repositorio con data source
    let rest_countries_repository = RestCountriesRepository::new(RestCountriesDataSource::new());
    let rest_countries: RestCountries = rest_countries_repository
        .get_country_by_name(name)
        .await
        .map_err(|err| {
        println!("{}", err);
        CountryError::CreateCountryError("cannot get that country for restcountries")
    })?;
    //creamos repositorio de countries
    let country_repository: CountryRepository = repo
        .get_repository::<CountryRepository>()
        .await
        .map_err(|_| CountryError::CreateCountryError("a terrible error is passing"))?;
    let country = rest_countries
        .get(0)
        .ok_or_else(|| CountryError::CreateCountryError("cannot get country"))?;
    println!("{:?}", country);
    let country: Country = country.into();
    let country: CountryAttributes = country.into();
    //actualiza si lo encuentras, si no upsert
    let document_to_insert_country = doc! {
        "$set": bson::to_bson(&country).unwrap(),
    };
    let country_inserted = country_repository
        .find_one_and_update_with_upsert(
            doc! {"code":country.code},
            document_to_insert_country,
            None,
        )
        .await
        .map_err(|_| CountryError::CreateCountryError("error creating country"))?
        .ok_or_else(|| CountryError::CreateCountryError("result of coutnry is none"))?;
    Ok(JsonAdvanced(country_inserted))
}
//the last method is used only for create countries, and it is only for administrative peronal that have a token that y provide
//and a secret password for this moment, that y pass in path

//ahora si funciones normales como el getCountry by user y otras
#[web::get("user/{id}")]
pub async fn get_country_by_user_id(
    path: Path<IdPath>,
    repo: State<PublicRepository>,
) -> Result<JsonAdvanced<CountryWithId>, CountryError> {
    let country_repository: CountryRepository = repo
        .get_repository::<CountryRepository>()
        .await
        .map_err(|_| {
            CountryError::GetCountryError("internal error, comunicate with programmers")
        })?;
    //user repository
    let user_repsoitory: UserRepository =
        repo.get_repository::<UserRepository>().await.map_err(|_| {
            CountryError::GetCountryError("internal error, comunicate with programmers")
        })?;
    //busqueda de usuario con el path id
    let user_id = ObjectId::parse_str(path.id())
        .map_err(|_| CountryError::GetCountryError("canot parse important data"))?;
    println!("{:?}", user_id);
    let user = user_repsoitory
        .find_one(doc! {"_id":user_id}, None)
        .await
        .map_err(|err| {
            println!("{}", err);
            CountryError::GetCountryError("internal data failure")
        })?
        .ok_or_else(|| CountryError::GetCountryError("not exist user"))?;
    println!("{:?}", user.country.code);
    let country = country_repository
        .find_one(doc! {"code":user.country.code}, None)
        .await
        .map_err(|_| CountryError::GetCountryError("internal data failure"))?
        .ok_or_else(|| CountryError::GetCountryError("not exist country"))?;
    Ok(JsonAdvanced(country))
}

#[web::get("{id}")]
pub async fn get_country_by_id(
    path: Path<IdPath>,
    repo: State<PublicRepository>,
) -> Result<JsonAdvanced<CountryWithId>, CountryError> {
    let country_repository: CountryRepository = repo
        .get_repository::<CountryRepository>()
        .await
        .map_err(|_| {
            CountryError::GetCountryError("internal error, comunicate with programmers")
        })?;
    println!("{}", path.id());
    let country_id = ObjectId::parse_str(path.id())
        .map_err(|_| CountryError::GetCountryError("canot parse important data"))?;
    let country = country_repository
        .find_one(doc! {"_id":country_id}, None)
        .await
        .map_err(|_| CountryError::GetCountryError("internal data failure"))?
        .ok_or_else(|| CountryError::GetCountryError("not exist country"))?;
    Ok(JsonAdvanced(country))
}

//get coutnry by code
#[web::get("code/{id}")]
pub async fn get_country_by_code(
    path: Path<IdPath>,
    repo: State<PublicRepository>,
) -> Result<JsonAdvanced<CountryWithId>, CountryError> {
    let country_repository: CountryRepository = repo
        .get_repository::<CountryRepository>()
        .await
        .map_err(|_| {
            CountryError::GetCountryError("internal error, comunicate with programmers")
        })?;
    let country = country_repository
        .find_one(doc! {"code":path.id()}, None)
        .await
        .map_err(|_| CountryError::GetCountryError("internal data failure"))?
        .ok_or_else(|| CountryError::GetCountryError("not exist country"))?;
    Ok(JsonAdvanced(country))
}

#[web::get("get_all")]
pub async fn get_all_countries(
    repo: State<PublicRepository>,
) -> Result<JsonAdvanced<Vec<CountryWithId>>, CountryError> {
    println!("no data");
    let country_repository: CountryRepository = repo
        .get_repository::<CountryRepository>()
        .await
        .map_err(|_| {
            CountryError::GetCountryError("internal error, comunicate with programmers")
        })?;
    let mut countries = country_repository
        .find(doc! {})
        .await
        .map_err(|_| CountryError::GetCountryError("internal data failure"))?;
    let mut countries_vector = vec![];
    while let Some(country) = countries.next().await {
        if country.is_err() {
            return Err(CountryError::GetCountryError(
                "code error, contact with tecnical team",
            ));
        }
        let country = country.unwrap();
        countries_vector.push(country);
    }
    Ok(JsonAdvanced(countries_vector))
}

//genera el getCountryForRegion
#[web::get("region/{id}")]
pub async fn get_country_by_region_id(
    path: Path<IdPath>,
    repo: State<PublicRepository>,
) -> Result<JsonAdvanced<Vec<CountryWithId>>, CountryError> {
    let region_id = path.id();
    let country_repository: CountryRepository = repo
        .get_repository::<CountryRepository>()
        .await
        .map_err(|_| {
            CountryError::GetCountryError("internal error, comunicate with programmers")
        })?;
   //trae region repository
    let region_repository: RegionRepository = repo
        .get_repository::<RegionRepository>()
        .await
        .map_err(|_| {
            CountryError::GetCountryError("internal error, comunicate with programmers")
        })?;
    //busca region por id
    let region = region_repository
        .find_one(doc! {"code":region_id}, None)
        .await
        .map_err(|_| CountryError::GetCountryError("internal data failure"))?
        .ok_or_else(|| CountryError::GetCountryError("not exist region"))?;
    //busca paises por region
    let mut countries = country_repository
        .find(doc! {"regionId":region.country_id})
        .await
        .map_err(|_| CountryError::GetCountryError("internal data failure"))?;
    let mut countries_vector = vec![];
    while let Some(country) = countries.next().await {
        if country.is_err() {
            return Err(CountryError::GetCountryError(
                "code error, contact with tecnical team",
            ));
        }
        let country = country.unwrap();
        countries_vector.push(country);
    }
    Ok(JsonAdvanced(countries_vector))

}
