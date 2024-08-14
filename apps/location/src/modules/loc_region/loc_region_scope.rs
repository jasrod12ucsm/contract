use ntex::web::{scope, ServiceConfig};

use super::loc_region_controller::{create_region, get_all_regions, get_region_by_country_code};

pub fn loc_region_scope(configure: &mut ServiceConfig) {
    configure.service(
        scope("/")
            .service(create_region)
            .service(get_all_regions)
            .service(get_region_by_country_code),
    );
}
