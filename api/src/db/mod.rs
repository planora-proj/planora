mod error;
pub(crate) mod helpers;
pub mod models;
pub mod repos;

pub use error::DatabaseError;

pub type DBResult<T> = Result<T, DatabaseError>;
