use std::{fmt, ops};

use futures::StreamExt;
use serde::{de::DeserializeOwned, Serialize};

use ntex::http::{Payload, Response, StatusCode};
use ntex::web::error::{ErrorRenderer, WebResponseError};
use ntex::web::{FromRequest, HttpRequest, Responder};
use validator::Validate;

use super::errors::{JsonError, ValidationFieldsErrorStruct};
pub struct JsonAdvanced<T>(pub T);

impl<T> JsonAdvanced<T> {
    /// Deconstruct to an inner value
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> ops::Deref for JsonAdvanced<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> ops::DerefMut for JsonAdvanced<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> fmt::Debug for JsonAdvanced<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Json").field(&self.0).finish()
    }
}

impl<T> fmt::Display for JsonAdvanced<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl<T: Serialize, Err: ErrorRenderer> Responder<Err> for JsonAdvanced<T>
where
    Err::Container: From<JsonError>,
{
    async fn respond_to(self, req: &HttpRequest) -> Response {
        let body = match serde_json::to_string(&self.0) {
            Ok(body) => body,
            Err(e) => return e.error_response(req),
        };

        Response::build(StatusCode::OK)
            .content_type("application/json")
            .body(body)
    }
}

/// Json extractor. Allow to extract typed information from request's
/// payload.
///
/// To extract typed information from request's body, the type `T` must
/// implement the `Deserialize` trait from *serde*.
///
/// [**JsonConfig**](struct.JsonConfig.html) allows to configure extraction
/// process.
///
/// ## Example
///
/// ```rust
/// use ntex::web;
///
/// #[derive(serde::Deserialize)]
/// struct Info {
///     username: String,
/// }
///
/// /// deserialize `Info` from request's body
/// async fn index(info: web::types::Json<Info>) -> String {
///     format!("Welcome {}!", info.username)
/// }
///
/// fn main() {
///     let app = web::App::new().service(
///         web::resource("/index.html").route(
///            web::post().to(index))
///     );
/// }
/// ```
///

#[derive(Clone)]
pub struct JsonConfigAdvanced {
    limit: usize,
}

impl JsonConfigAdvanced {
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }
}

impl Default for JsonConfigAdvanced {
    fn default() -> Self {
        JsonConfigAdvanced { limit: 32768 }
    }
}

impl<T, Err: ErrorRenderer> FromRequest<Err> for JsonAdvanced<T>
where
    T: DeserializeOwned + Validate + 'static,
{
    type Error = JsonError;

    async fn from_request(req: &HttpRequest, payload: &mut Payload) -> Result<Self, Self::Error> {
        let content_type = req.headers().get("content-type");

        let limit = req
            .app_state::<JsonConfigAdvanced>()
            .map(|c| c.limit)
            .unwrap_or(32768);

        if content_type.is_none()
            || !content_type
                .unwrap()
                .to_str()
                .unwrap_or("")
                .contains("json")
        {
            return Err(JsonError::JsonBasicTransformError);
        }

        let mut body = Vec::with_capacity(8192);
        let mut length = 0;

        while let Some(chunk) = payload.next().await {
            let chunk = match chunk {
                Ok(chunk) => chunk,
                Err(_) => return Err(JsonError::JsonBasicTransformError),
            };

            length += chunk.len();
            if length > limit {
                return Err(JsonError::JsonBasicTransformError);
            }
            body.extend_from_slice(&chunk);
        }

        let body_slice = &body[..];
        if let Ok(data) = serde_json::from_slice::<T>(body_slice) {
            if let Err(err) = data.validate() {
                return Err(JsonError::ValidationFieldsError(
                    ValidationFieldsErrorStruct::new(err),
                ));
            }
            Ok(JsonAdvanced(data))
        } else {
            Err(JsonError::JsonSerializeError)
        }
    }
}
