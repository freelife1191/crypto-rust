use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use serde::{Deserialize, Serialize};
use super::super::util::crypto_util;
use serde_with::base64::Base64;
use serde_with::serde_as;
use crate::error::crypto_error::CryptoError;

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CryptoCipherSpec {
    // algorithm
    pub ag: String,
    // block mode
    pub bm: String,
    // padding mode
    pub pm: String,
    // output format
    pub of: OutputFormat,
    // key
    #[serde_as(as = "Base64")]
    pub ky: Vec<u8>,
    // iv
    #[serde_as(as = "Base64")]
    pub iv: Vec<u8>,
    // nonce
    pub no: Option<Vec<u8>>,
    // additional authentication data
    pub ad: Option<Vec<u8>>,
}

#[allow(unused)]
impl CryptoCipherSpec {
    pub fn new(key: &[u8], iv: &[u8], output_format: OutputFormat) -> Self {
        Self {
            ag: String::from("AES"),
            bm: String::from("CBC"),
            pm: String::from("PKCS7"),
            of: output_format,
            ky: hex::decode(key).unwrap(),
            iv: hex::decode(iv).unwrap(),
            no: None,
            ad: None,
        }
    }
}

#[allow(unused, non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum OutputFormat {
    #[default]
    h16,
    b64,
}

#[allow(unused)]
impl OutputFormat {
    pub fn encoder(&self) -> fn(&[u8]) -> String {
        match self {
            OutputFormat::h16 => |data: &[u8]| hex::encode(data),
            OutputFormat::b64 => crypto_util::encode_base64
        }
    }

    pub fn decoder(&self) -> fn(&str) -> Result<Vec<u8>, CryptoError> {
        match self {
            OutputFormat::h16 => |data: &str| hex::decode(data).map_err(|e| CryptoError::CipherSpectHexError(e.to_string())),
            OutputFormat::b64 => |data: &str| BASE64_STANDARD.decode(data).map_err(|e| CryptoError::CipherSpectBase64Error(e.to_string())),
        }
    }
}