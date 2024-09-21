use std::{future::Future, pin::Pin};

use bod_models::schemas::{config::user_config::user_config_errors::UserConfigError, mst::user::models::user_with_id::UserWithId};
use bson::doc;
use common::utils::ntex_private::repository::public_repository::AbstractRepository;
use futures::StreamExt;

use crate::{modules::authentication::models::login_result::{LoginResult, LoginResultRestaurantEnum}, utils::repositories::restaurant_repository::RestaurantRepository};

type Callback<'a> = Box<dyn Fn(&'a UserWithId, &'a RestaurantRepository, &'a mut LoginResult) -> Pin<Box<dyn Future<Output = Result<(), UserConfigError>> + Send + 'a>> + Send + Sync + 'a>;

async fn handle_restaurant(
    user: &UserWithId,
    restaurant_repository: &RestaurantRepository,
    login_result: &mut LoginResult,
) -> Result<(), UserConfigError> {
    if user.type_provider == "C" {
        let mut cursor = restaurant_repository
            .find(doc! {"companyId": user.id})
            .await
            .map_err(|_| UserConfigError::LoginUserError("Error al buscar el restaurante"))?;
        
        while let Some(restaurant) = cursor.next().await {
            match restaurant {
                Ok(rest) => login_result.add_restaurant(LoginResultRestaurantEnum::Restaurant(rest).into()),
                Err(_) => {
                    return Err(UserConfigError::LoginUserError(
                        "Error al iterar sobre los restaurantes",
                    ))
                }
            }
        }
    } else if user.type_provider == "ATM" {
        let restaurant = restaurant_repository
            .find_one(doc! {"companyId": user.employed_by})
            .await
            .map_err(|_| UserConfigError::LoginUserError("Error al buscar el restaurante"))?
            .ok_or(UserConfigError::LoginUserError("No se encontró el restaurante"))?;
        login_result.add_restaurant(LoginResultRestaurantEnum::Restaurant(restaurant).into());
    } else {
        let restaurant = restaurant_repository
            .find_one(doc! {"companyId": user.employed_by})
            .await
            .map_err(|_| UserConfigError::LoginUserError("Error al buscar el restaurante"))?
            .ok_or(UserConfigError::LoginUserError("No se encontró el restaurante"))?;
        login_result.add_restaurant(LoginResultRestaurantEnum::ShortRestaurant(restaurant).into());
    }
    Ok(())
}


pub struct RestaurantCallback;

impl RestaurantCallback {
    pub fn create_callback<'a>() -> Callback<'a> {
        create_callback()
    }
}
fn create_callback<'a>() -> Callback<'a> {
    Box::new(move |user, restaurant_repository, login_result| {
        Box::pin(handle_restaurant(user, restaurant_repository, login_result))
    })
}