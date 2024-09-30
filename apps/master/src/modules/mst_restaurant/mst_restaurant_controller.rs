
use bod_models::schemas::mst::restaurant::{
    models::restaurant_with_id::RestaurantWithId, restaurant_error::RestaurantError,
};
use bson::{doc, oid::ObjectId};
use common::utils::ntex_private::{
        extractors::json::JsonAdvanced,
        repository::public_repository::{AbstractRepository, PublicRepository},
    };
use futures::StreamExt;
use ntex::web::{self, types::State, HttpRequest};

use crate::{
    modules::mst_restaurant::data::coordenates_dto::CoordenatesDto,
    utils::infrastructure::{
        restaurant_repository::RestaurantRepository, user_repository::UserRepository,
    },
};

#[web::post("get")]
pub async fn get_all_restaurants(
    req: HttpRequest,
    repo: State<PublicRepository>,
    location_data: JsonAdvanced<CoordenatesDto>,
) -> Result<JsonAdvanced<Vec<RestaurantWithId>>, RestaurantError> {
    let user_id = req
        .extensions()
        .get::<ObjectId>()
        .cloned()
        .ok_or_else(|| RestaurantError::ListRestaurantsError("no user id"))?;
    let restaurant_repository = repo
        .get_repository::<RestaurantRepository>()
        .await
        .map_err(|_| RestaurantError::ListRestaurantsError("internal error"))?;
    let user_repository = repo
        .get_repository::<UserRepository>()
        .await
        .map_err(|_| RestaurantError::ListRestaurantsError("internal error"))?;
    // Verificar nivel de usuario
    let user = user_repository
        .find_one(doc! {"_id": user_id})
        .await
        .map_err(|_| RestaurantError::ListRestaurantsError("internal error"))?
        .ok_or_else(|| RestaurantError::ListRestaurantsError("user not found"))?;
    // Devuelve todos los restaurantes
    if vec!["C", "D"].contains(&(user.type_provider).as_str()) {
        let restaurants_cursor = restaurant_repository
            .find(doc! {"noActive": true})
            .await
            .map_err(|_| RestaurantError::ListRestaurantsError("internal error"))?;
        let restaurants: Vec<RestaurantWithId> = restaurants_cursor
            .filter_map(|result| async move { result.ok() }) // Usa `ok()` para convertir directamente a Option
            .collect()
            .await;
        return Ok(JsonAdvanced(restaurants));
    }
    //verifica la ubicacion actual de la persona
    let CoordenatesDto {
        latitude,
        longitude,
    } = location_data.into_inner();
    let restaurants:Vec<RestaurantWithId>= restaurant_repository
        .find(doc! {"noactive":true,"location": {"$near": {"$geometry": {"type": "Point", "coordinates": [longitude, latitude]}, "$maxDistance": 1000}}})
        .await
        .map_err(|_| RestaurantError::ListRestaurantsError("internal error"))?.filter_map(|result| async move {result.ok()}).collect().await;
    
    // Devuelve solo un restaurante
    Ok(JsonAdvanced(restaurants))
}
