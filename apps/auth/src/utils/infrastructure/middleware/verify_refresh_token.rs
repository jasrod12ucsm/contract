
use bod_models::shared::{errors::{BadRequestError, ErrorGenerate}, jwt::claims::RenewClaims};
use common::helpers::env::env::ENV;
use jsonwebtoken::{DecodingKey, Validation};
use ntex::{
    http::header,
    web::{self, WebResponse},
    Middleware, Service,
};

pub struct VerifyRefreshToken;

impl<S> Middleware<S> for VerifyRefreshToken {
    type Service = VerifyRefreshTokenMiddleware<S>;

    fn create(&self, service: S) -> Self::Service {
        VerifyRefreshTokenMiddleware { service }
    }
}

pub struct VerifyRefreshTokenMiddleware<S> {
    service: S,
}

impl<S, Err> Service<web::WebRequest<Err>> for VerifyRefreshTokenMiddleware<S>
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
            return Ok(BadRequestError::render_web_response(
                req,
                "authentication error".to_string(),
                "That header is missing".to_string(),
            ));
        }
        let header = header.unwrap();
        let token = header.split(" ").collect::<Vec<&str>>()[1];
        let secret = ENV.get_string("SECRET_KEY_REFRESH").unwrap().to_string();
        let decoded_token = match jsonwebtoken::decode::<RenewClaims>(
            &token,
            &DecodingKey::from_secret(secret.to_string().as_ref()),
            &Validation::default(),
        ) {
            Ok(token) => token,
            Err(err) => {
                let error_type = err.to_string();
                match &err.into_kind() {
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature
                    | jsonwebtoken::errors::ErrorKind::ImmatureSignature => {
                        req.extensions_mut().insert::<bool>(false);
                    }
                    _ => todo!(),
                }
                return Ok(BadRequestError::render_web_response(
                    req,
                    "Authentication error".to_string(),
                    error_type,
                ));
            }
        };

        // Clonar el valor de claims antes de que la referencia se pierda
        let os = decoded_token.claims.os();
        req.extensions_mut().insert::<String>(os);
        Ok(ctx.call(&self.service, req).await?)
    }
}
