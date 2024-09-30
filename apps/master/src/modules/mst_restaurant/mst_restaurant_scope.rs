use common::middlewares::verify_token::VerifyToken;
use ntex::web::{self, scope, ServiceConfig};

use super::mst_restaurant_controller::get_all_restaurants;

pub fn mst_restaurant_scope(configure: &mut ServiceConfig) {
    configure.service(
        scope("/")
            //tambien poner middleware
            .service(
                web::scope("/protected/")
                    .wrap(VerifyToken)
                    .service(get_all_restaurants),
            ), //poner middleware
    );
}
