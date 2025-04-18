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
    #[serde(default)]
    pub id: i32,
    // algorithm
    pub ag: String,
    // block mode
    pub bm: Option<String>,
    // padding mode
    pub pm: Option<String>,
    // output format
    pub of: OutputFormat,
    // key
    #[serde_as(as = "Base64")]
    pub ky: Vec<u8>,
    // iv
    #[serde_as(as = "Option<Base64>")]
    // #[serde_as(as = "Option<SerializeFoo>")]
    // This works as normal:
    // #[serde(skip_serializing_if = "Option::is_none")]
    pub iv: Option<Vec<u8>>,
    // nonce
    pub no: Option<Vec<u8>>,
    // additional authentication data
    pub ad: Option<Vec<u8>>,
}

#[allow(unused)]
impl CryptoCipherSpec {
    pub fn new(id: i32, algorithm: Option<String>, key: &[u8], iv: Option<&[u8]>, output_format: OutputFormat) -> Self {
        let mut _ag = String::from("AES");
        let mut _bm = Some(String::from("CBC"));
        let mut _pm = Some(String::from("PKCS7"));
        let mut _iv = iv.and_then(|iv| hex::decode(iv).ok());
        if id == 400 {
            let alg_str = match algorithm {
                Some(alg) => match alg.as_str() {
                    "SHA-256" | "SHA256" => "SHA-256",
                    "SHA-384" | "SHA384" => "SHA-384",
                    "SHA-512" | "SHA512" => "SHA-512",
                    "SHA-512_256" | "SHA512_256" => "SHA-512_256",
                    _ => "SHA-512",
                },
                None => "SHA-512",
            };
            _ag = String::from(alg_str);
            _bm = None;
            _pm = None;
            _iv = None;
        }
        Self {
            id,
            ag: _ag,
            bm: _bm,
            pm: _pm,
            of: output_format,
            ky: hex::decode(key).unwrap_or_else(|_| vec![]),
            iv: _iv,
            no: None,
            ad: None,
        }
    }
}

#[allow(unused, non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum OutputFormat {
    #[default]
    b64,
    h16,
}

#[allow(unused)]
impl OutputFormat {
    pub fn encoder(&self) -> fn(&[u8]) -> String {
        match self {
            OutputFormat::b64 => crypto_util::encode_base64,
            OutputFormat::h16 => |data: &[u8]| hex::encode(data)
        }
    }

    pub fn decoder(&self) -> fn(&str) -> Result<Vec<u8>, CryptoError> {
        match self {
            OutputFormat::b64 => |data: &str| BASE64_STANDARD.decode(data).map_err(|e| CryptoError::CipherSpectBase64Error(e.to_string())),
            OutputFormat::h16 => |data: &str| hex::decode(data).map_err(|e| CryptoError::CipherSpectHexError(e.to_string())),
        }
    }
}