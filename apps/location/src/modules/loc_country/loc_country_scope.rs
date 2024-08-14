use ntex::web::{scope, ServiceConfig};

use super::loc_country_controller::{
    create_country, get_all_countries, get_country_by_code, get_country_by_id, get_country_by_user_id
};

pub fn loc_country_scope(configure: &mut ServiceConfig) {
    configure.service(
        scope("/")
            //tambien poner middleware
            .service(create_country)
            //poner middleware
            .service(get_all_countries)
            .service(get_country_by_user_id)
            .service(get_country_by_id)
            .service(get_country_by_code)
            
    );
}
