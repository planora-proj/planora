use actix_web::http::header::HeaderName;

pub const X_USER_ID_HEADER_KEY: &'static str = "x-user-id";
pub const X_USER_ID_HEADER: HeaderName = HeaderName::from_static(X_USER_ID_HEADER_KEY);

pub const X_ORG_ID_HEADER_KEY: &'static str = "x-organization-id";
pub const X_ORG_ID_HEADER: HeaderName = HeaderName::from_static(X_ORG_ID_HEADER_KEY);
