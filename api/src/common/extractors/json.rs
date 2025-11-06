use actix_web::{FromRequest, HttpRequest, dev::Payload, web};
use futures::future::LocalBoxFuture;
use serde::de::DeserializeOwned;
use std::ops::Deref;

use crate::common::ApiError;

pub struct JsonApi<T>(pub T);

impl<T> Deref for JsonApi<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> FromRequest for JsonApi<T>
where
    T: DeserializeOwned + 'static,
{
    type Error = ApiError;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        tracing::trace!("Deserialize json payload");
        let fut = web::Json::<T>::from_request(req, payload);

        Box::pin(async move {
            match fut.await {
                Ok(json) => Ok(JsonApi(json.into_inner())),
                Err(err) => {
                    tracing::warn!(?err, "Failed to parse JSON body");
                    Err(ApiError::BadRequest("invalid request body".to_string()))
                }
            }
        })
    }
}
