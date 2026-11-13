use std::future::{ready, Ready};

use actix_web::{dev::Payload, error::ErrorUnauthorized, Error, FromRequest, HttpRequest};

pub const USER_ID_HEADER: &str = "X-User-Id";

pub struct CurrentUser {
    pub id: i64,
}

impl FromRequest for CurrentUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let id = req
            .headers()
            .get(USER_ID_HEADER)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.parse::<i64>().ok());

        ready(
            id.map(|id| CurrentUser { id })
                .ok_or_else(|| ErrorUnauthorized("missing or invalid X-User-Id header")),
        )
    }
}
