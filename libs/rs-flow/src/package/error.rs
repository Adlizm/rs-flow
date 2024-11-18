use thiserror::Error;

use super::serde::PackageDeserializerError;
use super::serde::PackageSerializerError;

#[derive(Debug, Error)]
pub enum PackageError {
    #[error("Not a empty package")]
    NotEmpty,

    #[error("Package not contain a number")]
    NotNumber,

    #[error("Package not contain a bool")]
    NotBoolean,

    #[error("Package not contain a string")]
    NotString,

    #[error("Package not contain bytes")]
    NotBytes,

    #[error("Package not contain a array")]
    NotArray,

    #[error("Package not contain a object")]
    NotObject,

    #[error("{0}")]
    SerializeFail(PackageSerializerError),

    #[error("{0}")]
    DeserializeFail(PackageDeserializerError),
}
