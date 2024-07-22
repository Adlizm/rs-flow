use super::serializer::PackageSerializerError;


#[derive(Debug)]
pub enum PackageError {
    NotNumber,
    NotBoolean,
    NotString,
    NotBytes,
    NotArray,
    NotObject,

    SerializeFail(PackageSerializerError)
}

impl std::fmt::Display for PackageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::error::Error for PackageError {
    
}