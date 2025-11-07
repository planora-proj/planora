mod error;
mod jwt;
mod service;

type AuthResult<T> = Result<T, error::AuthError>;

pub use error::AuthError;
pub use service::AuthService;
pub mod constants;
