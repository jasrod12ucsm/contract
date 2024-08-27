use std::vec;

use bod_models::schemas::location::region::{
    models::region_with_id::RegionWithId, region_attributes::RegionAttributes, region_errors::RegionError
};
use bson::doc;
use common::{
    helpers::env::env::ENV,
    utils::ntex_private::{
        extractors::json::JsonAdvanced,
        repository::public_repository::{AbstractRepository, PublicRepository},
    },
};
use futures::StreamExt;
use ntex::web::{self, types::Path};

use crate::{
    public::interfaces::RegionGeneratePath,
    utils::{
        infraestructure::{
            datasource::accu_wheather_data_source::AccuWheatherDataSource,
            repositories::accu_weather_repository::AccuWeatherRepository,
        },
        region_repository::RegionRepository,
    },
};

#[web::get("create/{country}/{secret}")]
pub async fn create_region(
    path: Path<RegionGeneratePath>,
    repo: web::types::State<PublicRepository>,
    //secret: web::types::State<SecretPassword>,
) -> Result<JsonAdvanced<Vec<RegionWithId>>, RegionError> {
    let RegionGeneratePath { country, secret } = path.into_inner();
    //verifica secret
    let secret_password_env = ENV
        .get_string("SECRET_ADMIN_PASSWORD")
        .map_err(|err| RegionError::CreateRegionError(err.to_string()))?;
    //deja pasar si el secret password es correcto
    if secret != secret_password_env {
        return Err(RegionError::CreateRegionError(
            "not correct secret".to_string(),
        ));
    }
    let accu_weather_repository = AccuWeatherRepository::new(AccuWheatherDataSource::new());
    let region_repository: RegionRepository = repo
        .get_repository::<RegionRepository>()
        .await
        .map_err(|_| {
            RegionError::CreateRegionError("code error, contact with the tecnical team".to_string())
        })?;
    //
    //primero busquemos en accu weather
    let region_accu_weather = accu_weather_repository
        .get_all_regions(country)
        .await
        .map_err(|_err| RegionError::CreateRegionError("accu wheather error".to_string()))?;

    //ahora por cada accu region haremos el codigo crea un for y optimizalo
    let mut regions_with_id: Vec<RegionWithId> = vec![];
    for accu_region in region_accu_weather {
        let region: RegionAttributes = accu_region.into();
        let document_to_insert_region = doc! {
            "$set":bson::to_bson(&region).unwrap()
        };
        let region_inserted = region_repository
            .find_one_and_update(
                doc! {"code":region.code},
                document_to_insert_region,
            ).upsert(true)
            .await
            .map_err(|err| RegionError::CreateRegionError(err.to_string()))?;
        if region_inserted.is_none() {
            return Err(RegionError::CreateRegionError(
                "error creating region, contact with the tecnical team".to_string(),
            ));
        }
        regions_with_id.push(region_inserted.unwrap());
    }
    Ok(JsonAdvanced(regions_with_id))
}

//ahora vamos a crear un endpoint para obtener todas las regiones
#[web::get("get_all")]
pub async fn get_all_regions(
    repo: web::types::State<PublicRepository>,
) -> Result<JsonAdvanced<Vec<RegionWithId>>, RegionError> {
    let region_repository: RegionRepository = repo
        .get_repository::<RegionRepository>()
        .await
        .map_err(|_| {
            RegionError::GetRegionsError("code error, contact with the tecnical team")
        })?;
    let mut regions =
        region_repository.get_all().await.map_err(|_| {
            RegionError::GetRegionsError("code error, contact with the tecnical team")
        })?;
    let mut vec_regions=vec![];
    while let Some(region) = regions.next().await {
        if region.is_err() {
            return Err(RegionError::GetRegionsError(
                "code error, contact with the tecnical team",
            ));
        }
        let region = region.unwrap();
        vec_regions.push(region);
    }
    Ok(JsonAdvanced(vec_regions))
}

//ahora nos traemos region por countrycode

#[web::get("get_by_country_code/{country_code}")]
pub async fn get_region_by_country_code(
    path: Path<String>,
    repo: web::types::State<PublicRepository>,
) -> Result<JsonAdvanced<Vec<RegionWithId>>, RegionError> {
    let country_code = path.into_inner();
    let region_repository: RegionRepository = repo
        .get_repository::<RegionRepository>()
        .await
        .map_err(|_| {
            RegionError::GetRegionsError("code error, contact with the tecnical team")
        })?;
    let mut regions = region_repository.find(doc! {"countryId":country_code,"noDeleted":true})
        .await
        .map_err(|_| {
            RegionError::GetRegionsError("code error, contact with the tecnical team")
        })?;
    let mut vec_regions=vec![];
    while let Some(region) = regions.next().await {
        if region.is_err() {
            return Err(RegionError::GetRegionsError(
                "code error, contact with the tecnical team",
            ));
        }
        let region = region.unwrap();
        vec_regions.push(region);
    }
    Ok(JsonAdvanced(vec_regions))
}
