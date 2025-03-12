use crate::domain::crypto_config::CryptoConfig;
use crate::error::crypto_error::CryptoError;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_kms::config::Credentials;
use aws_sdk_kms::primitives::Blob;
use aws_sdk_kms::Client;
use serde::Deserialize;
use tokio;

#[allow(unused)]
#[derive(Deserialize, Debug)]
pub struct AwsConfig {
    aws_kms_key_arn: String,
    aws_access_key_id: String,
    aws_secret_access_key: String,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct AwsKmsService {
    aws_kms_key_arn: String,
    aws_access_key_id: String,
    aws_secret_access_key: String,
    client: Client,
}

pub type AwsKmsServiceResult = Result<AwsKmsService, CryptoError>;

#[allow(unused)]
impl AwsKmsService {
    // fn read_config<P: AsRef<Path>>(path: P) -> Result<AwsConfig, Box<dyn std::error::Error>> {
    pub fn read_config() -> Result<AwsConfig, CryptoError> {
        // let config_content = fs::read_to_string("../resources/config.json")
        //     .map_err(|e| CryptoError::ConfigPathError(e))?;
        let config_content = include_str!("../resources/default/config.json");
        // let config_content = fs::read_to_string(path)
        //     .map_err(|e| CryptoError::ConfigPathError(e))?;

        let config = serde_json::from_str(&config_content)
            .map_err(|e| CryptoError::ConfigParsingError(e.to_string()))?;

        Ok(config)
    }

    pub fn new() -> AwsKmsServiceResult {
        let config = Self::read_config()?;
        Ok(Self::init(config)?)
    }

    pub fn get_kms_service(config: CryptoConfig) -> Result<AwsKmsService, CryptoError> {

        let aws_config = AwsConfig {
            aws_kms_key_arn: config.aws_kms_key_arn.unwrap().to_string(),
            aws_access_key_id: config.aws_access_key_id.unwrap(),
            aws_secret_access_key: config.aws_secret_access_key.unwrap(),
        };

        // println!("{:?}", aws_config);

        Self::init(aws_config)
    }

    fn init(config: AwsConfig) -> Result<AwsKmsService, CryptoError> {
        // todo: match 에러 체크

        let region_provider = RegionProviderChain::default_provider().or_else("ap-northeast-2");
        // let access_key_id = env::var("AWS_ACCESS_KEY_ID").expect("AWS_ACCESS_KEY_ID not set");
        // let secret_access_key = env::var("AWS_SECRET_ACCESS_KEY").expect("AWS_SECRET_ACCESS_KEY not set");
        // println!("AWS_ACCESS_KEY_ID: {}, AWS_SECRET_ACCESS_KEY: {}", &config.access_key_id, &config.secret_access_key);
        let credentials = Credentials::new(&config.aws_access_key_id, &config.aws_secret_access_key, None, None, "crypto");
        let runtime = tokio::runtime::Runtime::new()
            .map_err(|e| CryptoError::AwsKmsInitError(e.to_string()))?;
        let aws_config = runtime.block_on(async {
            aws_config::from_env()
                .region(region_provider)
                .credentials_provider(credentials)
                .load()
                .await
        });

        Ok(AwsKmsService {
            aws_kms_key_arn: config.aws_kms_key_arn,
            aws_access_key_id: config.aws_access_key_id,
            aws_secret_access_key: config.aws_secret_access_key,
            client: Client::new(&aws_config),
        })
    }

    /*
    pub async fn encrypt(plaintext: &[u8]) -> Result<String, Error> {
        let config = Self::new();
        let resp = config.client.encrypt()
            .key_id(config.aws_kms_key)
            .plaintext(Blob::new(plaintext))
            .send()
            .await?;

        let ciphertext_blob = resp.ciphertext_blob().expect("Ciphertext blob should be present");
        Ok(BASE64_STANDARD.encode(ciphertext_blob.as_ref()))
    }
    */

    /*
    pub async fn decrypt(ciphertext: &str) -> Result<Vec<u8>, Error> {
        let config = Self::new().await;;
        let ciphertext_blob = Blob::new(BASE64_STANDARD.decode(ciphertext).expect("Failed to decode base64 ciphertext"));
        let resp = config.client.decrypt()
            .ciphertext_blob(ciphertext_blob)
            .send()
            .await?;

        let plaintext = resp.plaintext().expect("Plaintext should be present");
        Ok(plaintext.as_ref().to_vec())
    }
    */

    // pub fn encrypt(plaintext: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
    pub fn encrypt(&self, plain: &[u8]) -> Result<Vec<u8>, CryptoError> {
        // let config = Self::new();

        let runtime = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
        let resp = runtime.block_on(async {
            self.client.encrypt()
                .key_id(&self.aws_kms_key_arn)
                .plaintext(Blob::new(plain))
                .send()
                .await
        });
        let ciphertext_blob = resp.map_err(|e| CryptoError::AwsKmsEncryptError(e.to_string()))?;
        let ciphertext = ciphertext_blob.ciphertext_blob().ok_or_else(|| CryptoError::AwsKmsEncryptError("Ciphertext blob should be present".to_string()))?;
        Ok(ciphertext.as_ref().to_vec())
    }

    pub fn decrypt(&self, encrypted: &[u8]) -> Result<Vec<u8>, CryptoError> {
        // let config = Self::new();

        /*
        let ciphertext_blob = match BASE64_STANDARD.decode(cipher) {
            Ok(decoded) => Blob::new(decoded),
            // Err(e) => return Err(Box::new(e)),
            Err(e) => panic!("Base64 Decoded failed: {}", e),
        };
        */

        let runtime = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
        let resp = runtime.block_on(async {
            self.client.decrypt()
                .ciphertext_blob(Blob::new(encrypted))
                .send()
                .await
        });
        let plaintext_blob = resp.map_err(|e| CryptoError::AwsKmsDecryptError(e.to_string()))?;
        let plaintext = plaintext_blob.plaintext().ok_or_else(|| CryptoError::AwsKmsDecryptError("Plaintext should be present".to_string()))?;
        Ok(plaintext.as_ref().to_vec())
    }
}
