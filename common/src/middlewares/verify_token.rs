
use bod_models::shared::jwt::claims::DefaultClaims;
use bson::oid::ObjectId;
use jsonwebtoken::{DecodingKey, Validation};
use ntex::{
    http::{header, Response},
    web::{self, WebResponse},
    Middleware, Service,
};

use crate::helpers::env::env::ENV;

pub struct VerifyToken;

impl<S> Middleware<S> for VerifyToken {
    type Service = VerifyTokenMiddleware<S>;

    fn create(&self, service: S) -> Self::Service {
        VerifyTokenMiddleware { service }
    }
}

pub struct VerifyTokenMiddleware<S> {
    service: S,
}

impl<S, Err> Service<web::WebRequest<Err>> for VerifyTokenMiddleware<S>
where
    S: Service<web::WebRequest<Err>, Response = web::WebResponse, Error = web::Error>,
    Err: web::ErrorRenderer,
{
    type Response = WebResponse;
    type Error = web::Error;

    ntex::forward_ready!(service);

    async fn call(
        &self,
        req: web::WebRequest<Err>,
        ctx: ntex::ServiceCtx<'_, Self>,
    ) -> Result<Self::Response, Self::Error> {
        let header = req
            .headers()
            .get(header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok());
        if header.is_none() {
            return Ok(req.into_response(Response::InternalServerError().body("token is missing")));
        }
        let header = header.unwrap();
        let token = header.split(" ").collect::<Vec<&str>>()[1];
        let secret = ENV.get_string("SECRET_KEY").unwrap().to_string();
        let decoded_token = match jsonwebtoken::decode::<DefaultClaims>(
            &token,
            &DecodingKey::from_secret(secret.to_string().as_ref()),
            &Validation::default(),
        ) {
            Ok(token) => {
               token 
            }
            Err(err) => {
                return Ok(req.into_response(
                    Response::InternalServerError().body(format!("invalid token: {}", err)),
                ));
            }
        };
        let uid=decoded_token.claims.user();

        req.extensions_mut().insert::<ObjectId>(uid.to_owned());
        let res = ctx.call(&self.service, req).await?;
        Ok(res)
    }
}
