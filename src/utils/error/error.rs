use sqlx::Error;
use std::fmt;

#[derive(Debug)]
pub enum AppErrorType {
    DbError,
    NotFoundError
}

#[derive(Debug)]
pub struct AppError {
    pub code : i32,
    pub message :  Option<String>,
    pub cause: Option<String>,
    pub error_type: AppErrorType,
}

impl From<Error> for AppError {
    fn from(error: Error) -> AppError {
        AppError {
            code: 0,
            message: None, 
            cause: Some(error.to_string()),
            error_type: AppErrorType::DbError
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod test {

}