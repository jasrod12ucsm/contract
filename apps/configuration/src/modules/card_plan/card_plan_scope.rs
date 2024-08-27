use ntex::web::{scope, ServiceConfig};

use super::card_plan_controller::{
    get_all_card_plans,
    update_card_plan,
};

pub fn card_plan_scope(configure: &mut ServiceConfig) {
    configure.service(
        scope("/")
            .service(get_all_card_plans)
            .service(update_card_plan),
    );
}