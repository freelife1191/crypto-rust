use thiserror::Error as ThisError;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

pub type CryptoResult<T> = Result<T, CryptoError>;

/// https://blog.cro.sh/posts/error-handling-story-in-rust-part-2/#thiserror
/// https://docs.rs/thiserror/latest/thiserror/
///
#[allow(unused)]
#[serde_as]
#[derive(ThisError, Debug, Deserialize, Serialize)]
pub enum CryptoError {
    #[error("error code: 1001, message: {0}")]
    Generic(String),
    #[error("error code: 2001, message: {0}")]
    SessionError(String),
    #[error("error code: 2101, message: {0}")]
    ConfigPathError(String),
    #[error("error code: 2102, message: {0}")]
    ConfigParsingError(String),
    #[error("error code: 2103, message: {0}")]
    ConfigFileError(String),
    #[error("error code: 2104, message: config {0}")]
    ConfigDataError(String),
    #[error("error code: 2105, message: Invalid OutputFormat Data: Try Hex")]
    CipherSpectHexError(String),
    #[error("error code: 2106, message: Invalid OutputFormat Data: Try Base64")]
    CipherSpectBase64Error(String),
    #[error("error code: 2110, message: {0}")]
    AwsKmsError(String),
    #[error("error code: 2205, message: AWS KMS initialization Failed")]
    AwsKmsInitError(String),
    #[error("error code: 2211, message: AWS KMS Encrypt Failed")]
    AwsKmsEncryptError(String),
    #[error("error code: 2212, message: AWS KMS Decrypt Failed")]
    AwsKmsDecryptError(String),
    #[error("error code: 2301, message: Encrypt Failed")]
    EncryptError(String),
    #[error("error code: 2302, message: Decrypt Failed")]
    DecryptError(String),
    #[error("error code: 2401, message: {0}")]
    UtilError(String),
}

// impl fmt::Display for Error {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             Error::Generic(s) => s.fmt(f),
//         }
//     }
// }


// impl From<CryptoError> for String {
//     fn from(err: CryptoError) -> String {
//         err.to_string()
//     }
// }
//
// impl error::Error for CryptoError {
//     fn source(&self) -> Option<&(dyn error::Error + 'static)> {
//         None
//     }
// }
//
// impl From<String> for CryptoError {
//     fn from(err: String) -> CryptoError {
//         CryptoError::Generic(err)
//     }
// }
//
// impl From<std::ffi::NulError> for CryptoError {
//     fn from(err: std::ffi::NulError) -> CryptoError {
//         CryptoError::Generic(err.to_string())
//     }
// }