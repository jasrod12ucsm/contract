use common::middlewares::verify_token::VerifyToken;
use ntex::web::{scope, ServiceConfig};

use super::contract_controller::{create_contract, renew_contract};

pub fn contract_scope(configure: &mut ServiceConfig) {
    configure.service(
        scope("/").service(create_contract).service(
            scope("/protected/")
                .wrap(VerifyToken)
                .service(renew_contract),
        ),
    );
}
