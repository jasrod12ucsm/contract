use ntex::web::{scope, ServiceConfig};

use crate::utils::infrastructure::middleware::verify_refresh_token::VerifyRefreshToken;

use super::authentication_controller::{
    authenticate, login_by_token, login_client, renew, resend_email, singup_client,
};

pub fn authentication_route(configure: &mut ServiceConfig) {
    configure.service(
        scope("/")
            .service(singup_client)
            .service(authenticate)
            .service(login_client)
            .service(login_by_token)
            .service(resend_email)
            .service(scope("/protected/").wrap(VerifyRefreshToken).service(renew)),
    );
}
