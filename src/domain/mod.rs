use crate::errors::AppError;

pub mod collection;
pub mod file;
pub mod token;
pub mod user;

pub type AppResponse<T> = Result<Option<T>, AppError>;
