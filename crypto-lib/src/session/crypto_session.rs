use crate::domain::crypto_cipher_spec::CryptoCipherSpec;
use crate::domain::crypto_config::{CryptoConfig, CryptoType, JsonConfig};
use crate::error::crypto_error::CryptoError;
use crate::kms::aws_kms_service::{AwsConfig, AwsKmsService};
use crate::util::crypto_util;
use crate::util::crypto_util::decode_aes_256_gcm;
use std::path::Path;
use std::string::String;
use ring::digest::{Algorithm, SHA256, SHA384, SHA512, SHA512_256};

#[allow(unused)]
#[derive(Debug, Clone, Default)]
pub struct CryptoSession {
    session_type: CryptoSessionType,
    cipher_spec_vec: Vec<CryptoCipherSpec>,
    cipher_spec_enc: CryptoCipherSpec,
    cipher_spec_hash: CryptoCipherSpec
}


#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Default, PartialEq)]
enum CryptoSessionType {
    #[default]
    ENC_HASH,
    ONLY_ENC
}

#[allow(unused)]
impl CryptoSession {

    fn read_config() -> Result<AwsConfig, Box<dyn std::error::Error>> {
        let config_content = include_str!("../resources/default/config.json");
        // let config_content = fs::read_to_string(path)?;
        let config = serde_json::from_str(&config_content)?;
        Ok(config)
    }

    pub fn encrypt(&self, plaintext: String) -> Result<String, String> {
        // println!("Encrypt algorithm={}, block_mode={}, padding_mode={}", self.ag, self.bm, self.pm);
        let crypto_cipher_spec = self.cipher_spec_enc.clone();
        let _iv = match crypto_cipher_spec.iv.as_ref() {
            Some(iv) => iv.as_slice(),
            None => return Err("Initialization vector (IV) is missing".to_string()),
        };
        let encrypted = crypto_util::encrypt_algorithm(plaintext.as_bytes(), crypto_cipher_spec.ky.as_slice(), _iv)
            .map_err(|e| e.to_string())?;
        let encrypt_encoded = crypto_cipher_spec.clone().of.encoder()(encrypted.as_slice());
        // println!("Encrypt Encoded: {:?}", encrypt_encoded);
        Ok(encrypt_encoded)
    }

    pub fn encrypt_id(&self, plaintext: String, id: i32) -> Result<String, String> {
        // println!("Encrypt id={}, plaintext={}", id, plaintext);
        if id == 400 {
            if self.session_type == CryptoSessionType::ONLY_ENC {
                return Err(CryptoError::SessionError("Only Encrypt Config".to_string()).to_string());
            }
            let crypto_cipher_spec = self.cipher_spec_hash.clone();
            let algorithm = Self::get_hash_algorithm(crypto_cipher_spec.ag.clone())?;
            let encrypted = crypto_util::hash(plaintext.as_bytes(), Some(crypto_cipher_spec.ky.as_slice()), algorithm)
                .map_err(|e| e.to_string())?;
            return Ok(crypto_cipher_spec.clone().of.encoder()(encrypted.as_slice()));
        }
        Ok(self.encrypt(plaintext).map_err(|e| e.to_string())?)
    }
    

    pub fn decrypt(&self, encrypted: String) -> Result<String, String> {
        // println!("Decrypt algorithm={}, block_mode={}, padding_mode={}", self.ag, self.bm, self.pm);
        let crypto_cipher_spec = self.cipher_spec_enc.clone();
        let _iv = match crypto_cipher_spec.iv.as_ref() {
            Some(iv) => iv.as_slice(),
            None => return Err("Initialization vector (IV) is missing".to_string()),
        };
        let data_binding = crypto_cipher_spec.of.decoder()(&encrypted)
            .map_err(|e| e.to_string())?;
        let decrypted = crypto_util::decrypt_algorithm(data_binding.as_slice(), crypto_cipher_spec.ky.as_slice(), _iv)
            .map_err(|e| e.to_string())?;
        let data = String::from_utf8(decrypted)
            .map_err(|e| CryptoError::SessionError(e.to_string()).to_string())?;
        Ok(data)
        // let decrypt_decoded = self.of.decoder()();
        // println!("Encrypt Encoded: {:?}", encrypt_encoded);
        // encrypt_encoded
    }

    pub fn decrypt_id(&self, encrypted: String, id: i32) -> Result<String, String> {
        // println!("Decrypt algorithm={}, block_mode={}, padding_mode={}", self.ag, self.bm, self.pm);
        if id == 400 {
            return Ok(encrypted);
        }
        let decrypted = self.decrypt(encrypted).map_err(|e| e.to_string())?;
        Ok(decrypted)
        // let decrypt_decoded = self.of.decoder()();
        // println!("Encrypt Encoded: {:?}", encrypt_encoded);
        // encrypt_encoded
    }

    pub fn hash(&self, plaintext: String) -> Result<String, String> {
        if plaintext.is_empty() {
            return Ok(plaintext);
        }
        // println!("Hash plaintext={}", plaintext);
        let encrypted = crypto_util::hash(plaintext.as_bytes(), None, &SHA256)
            .map_err(|e| e.to_string())?;
        Ok(crypto_util::encode_base64(encrypted.as_slice()))
    }

    pub fn hash_algorithm(&self, plaintext: String, algorithm_str: String) -> Result<String, String> {
        if plaintext.is_empty() {
            return Ok(plaintext);
        }
        // println!("Hash plaintext={}, algorithm_str={}", plaintext, algorithm_str);
        let algorithm = Self::get_hash_algorithm(algorithm_str)?;
        let encrypted = crypto_util::hash(plaintext.as_bytes(), None, algorithm)
            .map_err(|e| e.to_string())?;
        Ok(crypto_util::encode_base64(encrypted.as_slice()))
    }

    pub fn hash_algorithm_key(&self, plaintext: String, algorithm_str: String, key: &[u8]) -> Result<String, String> {
        if plaintext.is_empty() {
            return Ok(plaintext);
        }
        // println!("Hash plaintext={}, algorithm_str={}", plaintext, algorithm_str);
        let algorithm = Self::get_hash_algorithm(algorithm_str)?;
        let encrypted = crypto_util::hash(plaintext.as_bytes(), Some(key), algorithm)
            .map_err(|e| e.to_string())?;
        // println!("Hash encrypted={:?}", hex::encode(encrypted.as_slice()));
        Ok(crypto_util::encode_base64(encrypted.as_slice()))
    }

    fn get_hash_algorithm<'a>(algorithm_str: String) -> Result<&'a Algorithm, String> {
        let alg_string = algorithm_str.trim().to_uppercase();
        let alg_str = alg_string.as_str();
        let algorithm = match alg_str {
            "SHA-256" | "SHA256" => &SHA256,
            "SHA-384" | "SHA384" => &SHA384,
            "SHA-512" | "SHA512" => &SHA512,
            "SHA-512_256" | "SHA512_256" => &SHA512_256,
            _ => return Err("Invalid Algorithm Type".to_string()),
            // _ => &SHA512,
        };
        Ok(algorithm)
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
        let crypto_session = Self::get_cipher_spec_vec(crypto_config.clone())
            .map_err(|e| e.to_string())?;
        Ok(crypto_session)
    }

    pub fn get_cipher_spec_vec(config: CryptoConfig) -> Result<CryptoSession, String> {
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
        let credential_vec = match config.credential {
            Some(ref credential) => crypto_util::decrypt_algorithm(credential.as_slice(), &cred_key_vec, &cred_iv_vec)
                .map_err(|e| e.to_string())?,
            None => return Err("Credential is missing".to_string()),
        };
        // println!("credential_vec: {:?}", String::from_utf8(credential_vec.clone()).unwrap());
        let result: Result<Vec<CryptoCipherSpec>, _> = serde_json::from_slice(credential_vec.as_slice());
        // println!("result: {:?}", result);
        let crypto_session = match result {
            Ok(vec) => CryptoSession {
                session_type: CryptoSessionType::ENC_HASH,
                cipher_spec_vec: vec.clone(),
                cipher_spec_enc: vec.iter().filter(|it| it.id == 100).next().ok_or_else(|| CryptoError::SessionError("Encryption(100) spec not found".to_string()).to_string())?.clone(),
                cipher_spec_hash: vec.iter().filter(|it| it.id == 400).cloned().next().unwrap_or_else(|| CryptoCipherSpec::default()).clone()
            },
            Err(_) => {
                let enc: CryptoCipherSpec = serde_json::from_slice(credential_vec.as_slice())
                    .map_err(|e| e.to_string())?;
                CryptoSession {
                    session_type: CryptoSessionType::ONLY_ENC,
                    cipher_spec_vec: Vec::new(),
                    cipher_spec_enc: enc,
                    cipher_spec_hash: CryptoCipherSpec::default(), // 빈 벡터로 초기화
                }
            }
        };
        Ok(crypto_session)
    }

    fn get_seed_vec(config: CryptoConfig) -> Result<Vec<u8>, String> {
        if config.clone().crypto_type == Some(CryptoType::AWS.to_string()) {
            let kms_service = AwsKmsService::get_kms_service(config.clone()).map_err(|e| e.to_string())?;
            let seed_vec = match config.seed {
                Some(seed) => kms_service.decrypt(seed.as_slice()).map_err(|e| e.to_string())?,
                None => return Err("Seed is missing".to_string()),
            };
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
}