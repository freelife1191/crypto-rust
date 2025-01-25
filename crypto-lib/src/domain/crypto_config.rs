use crate::error::crypto_error::CryptoError;
use crate::util::crypto_util;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_kms_key_arn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_access_key_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_secret_access_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iv: Option<String>,
    pub seed: Option<String>,
    pub credential: Option<String>,
}

#[allow(unused)]
impl JsonConfig {
    pub fn new(aws_kms_key_arn: String, aws_access_key_id: String, aws_secret_access_key: String, key: String, iv: String, seed: String, credential: String) -> Self {
        Self {
            aws_kms_key_arn: Some(aws_kms_key_arn),
            aws_access_key_id: Some(aws_access_key_id),
            aws_secret_access_key: Some(aws_secret_access_key),
            key: Some(key),
            iv: Some(iv),
            seed: Some(seed),
            credential: Some(credential)
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LocalConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iv: Option<String>,
    pub seed: Option<String>,
    pub credential: Option<String>,
}

#[allow(unused)]
impl LocalConfig {
    pub fn new(key: String, iv: String, seed: String, credential: String) -> Self {
        Self {
            key: Some(key),
            iv: Some(iv),
            seed: Some(seed),
            credential: Some(credential)
        }
    }
}

#[derive(Default)]
pub enum CryptoType {
    #[default]
    LOCAL,
    AWS
}

#[allow(unused)]
impl CryptoType {
    pub fn to_string(&self) -> String {
        match self {
            CryptoType::LOCAL => String::from("LOCAL"),
            CryptoType::AWS => String::from("AWS"),
        }
    }
    pub fn as_str(&self) -> &str {
        match self {
            CryptoType::LOCAL => "LOCAL",
            CryptoType::AWS => "AWS",
        }
    }

    pub fn from_string(value: &str) -> Self {
        match value {
            "LOCAL" => CryptoType::LOCAL,
            "AWS" => CryptoType::AWS,
            _ => CryptoType::LOCAL,
        }
    }

    pub fn from_option_string(value: Option<String>) -> Self {
        match value {
            Some(value) => CryptoType::from_string(value.as_str()),
            None => CryptoType::LOCAL,
        }
    }

    pub fn from_option_string_ref(value: Option<&str>) -> Self {
        match value {
            Some(value) => CryptoType::from_string(value),
            None => CryptoType::LOCAL,
        }
    }
}

#[allow(unused)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CryptoConfig {
    pub crypto_type: Option<String>,
    pub aws_kms_key_arn: Option<String>,
    pub aws_access_key_id: Option<String>,
    pub aws_secret_access_key: Option<String>,
    pub key : Option<String>,
    pub iv : Option<String>,
    pub key_iteration: Option<i32>,
    pub iv_iteration: Option<i32>,
    pub seed: Option<Vec<u8>>,
    pub credential: Option<Vec<u8>>,
}

#[allow(unused)]
impl CryptoConfig {
    pub fn new(path: &Path) -> Result<Self, String> {
        // Path::new(path.as_str())
        let config: JsonConfig = crypto_util::from_json_path(path)
            .map_err(|e| e.to_string())?;
        let result = Self::from_json_config(config)
            .map_err(|e| e.to_string())?;
        Ok(result)
    }

    pub fn from_json_config(json_config: JsonConfig) -> Result<Self, CryptoError> {

        let crypto_type = if json_config.aws_access_key_id.is_none() {
            CryptoType::LOCAL.to_string()
        } else {
            CryptoType::AWS.to_string()
        };

        let aws_kms_key_arn = if json_config.clone().aws_kms_key_arn.is_none() {
            json_config.clone().aws_kms_key_arn
        } else {
            Some(get_non_empty_value(json_config.clone().aws_kms_key_arn, "aws_kms_key_arn")?)
        };
        let aws_access_key_id = if json_config.clone().aws_access_key_id.is_none() {
            json_config.clone().aws_access_key_id
        } else {
            Some(get_non_empty_value(json_config.clone().aws_access_key_id, "aws_access_key_id")?)
        };
        let aws_secret_access_key = if json_config.clone().aws_secret_access_key.is_none() {
            json_config.clone().aws_secret_access_key
        } else {
            Some(get_non_empty_value(json_config.clone().aws_secret_access_key, "aws_secret_access_key")?)
        };

        let key = if crypto_type == CryptoType::LOCAL.to_string() {
            Some(json_config.clone().key.map_or_else(|| "".to_string(), |v| v))
        } else {
            None
        };
        let iv = if crypto_type == CryptoType::LOCAL.to_string() {
            Some(json_config.clone().iv.map_or_else(|| "".to_string(), |v| v))
        } else {
            None
        };

        let encrypt_seed = get_non_empty_value(json_config.clone().seed, "seed")?;
        let seed = BASE64_STANDARD.decode(encrypt_seed).map_err(|_| CryptoError::ConfigDataError("Failed to decode seed".to_string()))?;
        let encrypt_credential = json_config.clone().credential.ok_or_else(|| CryptoError::ConfigDataError("credential is required!".to_string()))?;
        let credential= BASE64_STANDARD.decode(encrypt_credential).map_err(|_| CryptoError::ConfigDataError("Failed to decode credential".to_string()))?;

        Ok(Self {
            crypto_type: Some(crypto_type),
            aws_kms_key_arn,
            aws_access_key_id,
            aws_secret_access_key,
            key,
            iv,
            key_iteration: None,
            iv_iteration: None,
            seed: Some(seed),
            credential: Some(credential),
        })
    }
}

fn get_non_empty_value(value: Option<String>, field_name: &str) -> Result<String, CryptoError> {
    let value = value.ok_or_else(|| CryptoError::ConfigDataError(format!("{} is required!", field_name)))?;
    let trimmed_value = value.trim().to_string();
    if trimmed_value.is_empty() {
        return Err(CryptoError::ConfigDataError(format!("{} cannot be blank!", field_name)));
    }
    Ok(trimmed_value)
}