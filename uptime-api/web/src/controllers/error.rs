use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("user not found")]
    UserNotFound,
    #[error("user password not match")]
    UserPasswordNotMatch,

}