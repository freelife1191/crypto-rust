use crate::domain::crypto_cipher_spec::{CryptoCipherSpec, OutputFormat};
use crate::domain::crypto_config::{CryptoConfig, CryptoType, JsonConfig};
use crate::error::crypto_error::CryptoError;
use crate::kms::aws_kms_service::{AwsConfig, AwsKmsService};
use crate::util::crypto_util;
use crate::util::crypto_util::{decode_aes_256_gcm, encode_base64};
use hex::encode;
use std::path::Path;
use std::string::String;
use aws_sdk_kms::config::retry::ShouldAttempt::No;

#[allow(unused)]
#[derive(Debug, Clone, Default)]
pub struct CryptoSession {
    ag: String,
    bm: String,
    pm: String,
    key: Vec<u8>,
    iv: Vec<u8>,
    of: OutputFormat
}

#[allow(unused)]
impl CryptoSession {

    fn read_config() -> Result<AwsConfig, Box<dyn std::error::Error>> {
        let config_content = include_str!("../resources/local/config.json");
        // let config_content = fs::read_to_string(path)?;
        let config = serde_json::from_str(&config_content)?;
        Ok(config)
    }

    pub fn encrypt(&self, plaintext: String) -> Result<String, String> {
        // println!("Encrypt algorithm={}, block_mode={}, padding_mode={}", self.ag, self.bm, self.pm);
        let encrypted = crypto_util::encrypt_algorithm(plaintext.as_bytes(), self.key.as_slice(), self.iv.as_slice())
            .map_err(|e| e.to_string())?;
        let encrypt_encoded = self.of.encoder()(encrypted.as_slice());
        // println!("Encrypt Encoded: {:?}", encrypt_encoded);
        Ok(encrypt_encoded)
    }

    pub fn decrypt(&self, encrypted: String) -> Result<String, String> {
        // println!("Decrypt algorithm={}, block_mode={}, padding_mode={}", self.ag, self.bm, self.pm);
        let decrypted =
            crypto_util::decrypt_algorithm(
                self.of.decoder()(&encrypted)
                    .map_err(|e| e.to_string())?
                    .as_slice(), self.key.as_slice(), self.iv.as_slice())
                .map_err(|e| e.to_string())?;
        let data = String::from_utf8(decrypted)
            .map_err(|e| CryptoError::SessionError(e.to_string()).to_string())?;
        Ok(data)
        // let decrypt_decoded = self.of.decoder()();
        // println!("Encrypt Encoded: {:?}", encrypt_encoded);
        // encrypt_encoded
    }

    // 경로지정 + json 파일로 config 를 받을 수도 있게
    pub fn create(path: &Path) -> Result<CryptoSession, String> {
        // Path::new(path.as_str())
        let config: JsonConfig = crypto_util::from_json_path(path)
            .map_err(|e| e.to_string())?;
        Self::set_crypto_session(&config).map_err(|e| e.to_string())
    }

    pub fn of_local(key: String, iv: String, seed: String, credential: String) -> Result<CryptoSession, String> {
        let config: JsonConfig = JsonConfig {
            aws_kms_key_arn: None,
            aws_access_key_id: None,
            aws_secret_access_key: None,
            key: Some(key),
            iv: Some(iv),
            seed: Some(seed),
            credential: Some(credential)
        };
        Self::set_crypto_session(&config).map_err(|e| e.to_string())
    }

    pub fn of(aws_kms_key_arn: String, aws_access_key_id: String, aws_secret_access_key: String, seed: String, credential: String) -> Result<CryptoSession, String> {
        let aws_kms_key_arn = if aws_kms_key_arn.is_empty() {
            None
        } else { Some(aws_kms_key_arn) };
        let aws_access_key_id = if aws_access_key_id.is_empty() {
            None
        } else { Some(aws_access_key_id) };
        let aws_secret_access_key = if aws_secret_access_key.is_empty() {
            None
        } else { Some(aws_secret_access_key) };
        let config: JsonConfig = JsonConfig {
            aws_kms_key_arn,
            aws_access_key_id,
            aws_secret_access_key,
            key: None,
            iv: None,
            seed: Some(seed),
            credential: Some(credential)
        };
        Self::set_crypto_session(&config).map_err(|e| e.to_string())
    }

    pub fn of_byte(bytes: &[u8]) -> Result<CryptoSession, String> {
        let config: JsonConfig = crypto_util::from_json_byte(bytes.to_vec())
            .map_err(|e| e.to_string())?;
        Self::set_crypto_session(&config).map_err(|e| e.to_string())
    }

    pub fn of_config(config: JsonConfig) -> Result<CryptoSession, String> {
        Self::set_crypto_session(&config).map_err(|e| e.to_string())
    }

    fn set_crypto_session(config: &JsonConfig) -> Result<CryptoSession, String> {
        // println!("JsonConfig: {:#?}", &config);
        let crypto_config = CryptoConfig::from_json_config(config.clone())
            .map_err(|e| e.to_string())?;

        // println!("CryptoConfig: {:#?}", &crypto_config);
        let cipher_spec = Self::get_cipher_spec(crypto_config.clone())
            .map_err(|e| e.to_string())?;
        // println!("cipher_spec: {:#?}", &cipher_spec);
        // println!("key: {:?}, iv: {:?}", &cipher_spec.ky.len(), &cipher_spec.iv.len());
        Ok(Self::builder()
            .algorithm(cipher_spec.ag)
            .block_mode(cipher_spec.bm)
            .padding_mode(cipher_spec.pm)
            .key(cipher_spec.ky)
            .iv(cipher_spec.iv)
            .of(cipher_spec.of)
            .build())
    }

    pub fn get_cipher_spec(config: CryptoConfig) -> Result<CryptoCipherSpec, String> {
        let key_iteration = if let Some(key_iteration) = config.key_iteration {
            key_iteration
        } else { 10 };
        let iv_iteration = if let Some(iv_iteration) = config.iv_iteration {
            key_iteration
        } else { 10 };
        // println!("key_iteration: {}, iv_iteration: {}", key_iteration, iv_iteration);
        let seed_vec = Self::get_seed_vec(config.clone())
            .map_err(|e| e.to_string())?;
        // println!("seed_vec: {:?}", encode(seed_vec.as_slice()));
        let cred_key_vec = crypto_util::digest("SHA-256", &seed_vec, key_iteration)
            .map_err(|e| e.to_string())?;
        let cred_iv_vec = crypto_util::digest("MD5", &seed_vec, iv_iteration)
            .map_err(|e| e.to_string())?;
        // println!("seed_vec: {:?}", seed_vec);
        // println!("cred_key_vec: {:?}", cred_key_vec);
        // println!("cred_iv_vec: {:?}", cred_iv_vec);
        let credential_vec = crypto_util::decrypt_algorithm(config.credential.unwrap().as_slice(), &cred_key_vec, &cred_iv_vec)
            .map_err(|e| e.to_string())?;
        // println!("credential_vec: {:?}", String::from_utf8(credential_vec.clone()).unwrap());
        let result = serde_json::from_slice(credential_vec.as_slice())
            .map_err(|e| e.to_string())?;
        // println!("result: {:?}", result);
        Ok(result)
    }

    fn get_seed_vec(config: CryptoConfig) -> Result<Vec<u8>, String> {
        if config.clone().crypto_type == Some(CryptoType::AWS.to_string()) {
            let kms_service = AwsKmsService::get_kms_service(config.clone());
            let seed_vec = kms_service.decrypt(config.clone().seed.unwrap().as_slice())
                .map_err(|e| e.to_string())?;
            Ok(seed_vec)
        } else {
            let key_str = config.key.map_or_else(|| "".to_string(), |v| v);
            let iv_str  = config.iv.map_or_else(|| "".to_string(), |v| v);
            // let key_str = config.clone().key.map_or_else(|| "".to_string(), |v| v);
            // let iv_str = config.clone().iv.map_or_else(|| "".to_string(), |v| v);
            // println!("key_str: {}, iv_str: {}", key_str, iv_str);
            // println!("seed: {:?}", config.clone().seed.unwrap());
            let key = hex::decode(key_str).map_err(|e| format!("{:?}", e))?;
            let iv = hex::decode(iv_str).map_err(|e| format!("{:?}", e))?;

            // let seed = config.clone().seed.unwrap();
            let seed_byte  = config.seed.unwrap();
            // println!("암호화된 seed_byte: {:?}", encode_base64(seed_byte.as_slice()));
            let token = decode_aes_256_gcm(seed_byte.as_slice(), key.as_slice(), iv.as_slice())
                .map_err(|e| format!("{:?}", e))?;
            // println!("복호화된 데이터: {}", encode(token.clone()));
            // println!("원본 데이터: {}", "7e948d72e1f08f14be079839c07260261c9837992abe2c12493f4073e4484ef6ee9e0e976409c8ffb9b8da1a89c0cb5a936e69d7d85f14d4fdc5fcf106edcb8aee4451aaab24eedf7a6f1878".to_string());
            Ok(token)
        }
    }

    fn builder() -> CryptoSessionBuilder {
        CryptoSessionBuilder::default()
    }
}

#[allow(unused)]
#[derive(Default)]
struct CryptoSessionBuilder {
    algorithm: String,
    block_mode: String,
    padding_mode: String,
    key: Vec<u8>,
    iv: Vec<u8>,
    output_format: OutputFormat,
}

#[allow(unused)]
impl CryptoSessionBuilder {
    fn algorithm(mut self, algorithm: String) -> Self {
        self.algorithm = algorithm;
        self
    }

    fn block_mode(mut self, block_mode: String) -> Self {
        self.block_mode = block_mode;
        self
    }

    fn padding_mode(mut self, padding_mode: String) -> Self {
        self.padding_mode = padding_mode;
        self
    }

    fn key(mut self, key: Vec<u8>) -> Self {
        self.key = key;
        self
    }

    fn iv(mut self, iv: Vec<u8>) -> Self {
        self.iv = iv;
        self
    }

    fn of(mut self, output_format: OutputFormat) -> Self {
        self.output_format = output_format;
        self
    }

    fn build(self) -> CryptoSession {
        CryptoSession {
            ag: self.algorithm,
            bm: self.block_mode,
            pm: self.padding_mode,
            key: self.key,
            iv: self.iv,
            of: self.output_format,
        }
    }
}