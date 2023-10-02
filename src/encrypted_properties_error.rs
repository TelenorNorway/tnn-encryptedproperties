use thiserror::Error;

#[derive(Error, Debug)]
pub enum EncryptedPropertiesError {
    #[error("Master password cannot be empty")]
    EmptyPassword,
    #[error("Failed to decode base64")]
    Base64DecodeError,
    #[error("Invalid key length")]
    InvalidKeyLength,
    #[error("OpenSSL error: {0}")]
    OpenSSLError(#[from] openssl::error::ErrorStack),
    #[error("UTF-8 conversion error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
    #[error("Base64 decode error: {0}")]
    Base64Error(#[from] base64::DecodeError),
}