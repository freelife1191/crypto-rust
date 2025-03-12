#[cfg(test)]
mod aws_kms_service_test {
    use base64::Engine;
    use base64::prelude::BASE64_STANDARD;
    use crate::kms::aws_kms_service::AwsKmsService;
    use crate::domain::crypto_config::JsonConfig;

    fn get_seed() -> String {
        let config_content = include_str!("../resources/default/config.json");
        println!("{}", config_content);
        // let config_content = fs::read_to_string(path)?;
        let config: JsonConfig = serde_json::from_str(&config_content).unwrap();
        config.seed.unwrap_or_else(|| String::from(""))
    }

    #[test]
    fn kms_encrypt_test() {
        let kms_service = AwsKmsService::new().unwrap();
        /*
        let seed = match BASE64_STANDARD.decode(get_seed()) {
            Ok(decoded) => decoded,
            // Err(e) => return Err(Box::new(e)),
            Err(e) => panic!("Base64 Decoded failed: {}", e),
        };
        */
        let seed = BASE64_STANDARD.decode(get_seed()).unwrap();
        let seed_dec = kms_service.decrypt(&seed).unwrap();
        let seed_enc = kms_service.encrypt(&seed_dec).unwrap();

        println!("{:?}", seed);
        println!("Seed Enc: {:?}", seed_enc);
    }

    #[test]
    fn kms_decrypt_test() {
        let kms_service = AwsKmsService::new().unwrap();
        /*
        let seed = match BASE64_STANDARD.decode(get_seed()) {
            Ok(decoded) => decoded,
            // Err(e) => return Err(Box::new(e)),
            Err(e) => panic!("Base64 Decoded failed: {}", e),
        };
        */
        let seed = BASE64_STANDARD.decode(get_seed()).unwrap();
        let seed_dec = kms_service.decrypt(&seed).unwrap();
        println!("{:?}", seed);
        println!("{:?}", BASE64_STANDARD.encode(seed_dec));
    }
}

/*
#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = read_config().expect("Unable to read config file");

    let region_provider = RegionProviderChain::default_provider().or_else("ap-northeast-2");
    // let access_key_id = env::var("AWS_ACCESS_KEY_ID").expect("AWS_ACCESS_KEY_ID not set");
    // let secret_access_key = env::var("AWS_SECRET_ACCESS_KEY").expect("AWS_SECRET_ACCESS_KEY not set");
    println!("AWS_ACCESS_KEY_ID: {}, AWS_SECRET_ACCESS_KEY: {}", config.access_key_id, config.secret_access_key);
    let credentials = Credentials::new(config.access_key_id, config.secret_access_key, None, None, "user-provided");

    let aws_config = aws_config::from_env()
        .region(region_provider)
        .credentials_provider(credentials)
        .load()
        .await;

    let client = Client::new(&aws_config);

    let key_id = config.aws_kms_key;
    let plaintext = b"Hello, KMS!";

    // Encrypt the plaintext
    let ciphertext = encrypt(&client, &key_id, plaintext).await?;
    println!("Ciphertext: {}", ciphertext);

    // Decrypt the ciphertext
    let decrypted_plaintext = decrypt(&client, &ciphertext).await?;
    println!("Decrypted plaintext: {:?}", String::from_utf8(decrypted_plaintext).expect("Failed to convert to string"));

    Ok(())
}*/