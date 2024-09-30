use ntex::web::{scope, ServiceConfig};

use super::authentication_controller::{
    authenticate,  login_client, renew, resend_email, singup_client,
};

pub fn authentication_route(configure: &mut ServiceConfig) {
    configure.service(
        scope("/")
            .service(singup_client)
            .service(authenticate)
            .service(login_client)
            .service(renew)
            .service(resend_email),
    );
}
