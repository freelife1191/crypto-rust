#[cfg(test)]
mod crypto_config_generator {
    use crate::domain::crypto_cipher_spec::{CryptoCipherSpec, OutputFormat};
    use crate::domain::crypto_config::{CryptoConfig, CryptoType, LocalConfig};
    use crate::error::crypto_error::CryptoError;
    use crate::kms::aws_kms_service::AwsKmsService;
    use crate::session::crypto_session::CryptoSession;
    use crate::util::crypto_util;
    use crate::util::crypto_util::{decode_aes_256_gcm, decode_base64, encode_aes_256_gcm, encode_base64};
    // AES-GCM 구조체 및 관련 타입
    use aes_gcm::aead::{Aead, KeyInit};
    use aes_gcm::{Aes256Gcm, Key, Nonce};
    use serde_json::to_string_pretty;
    use std::fs;
    use std::path::{self, Path};
// Aead trait 및 랜덤 생성기

    #[allow(unused)]
    fn read_file_as_bytes<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<u8>> {
        fs::read(path)
    }

    fn generate_token(config: CryptoConfig, seed_byte: &[u8], key: &str, iv: &str) -> Result<Vec<u8>, String> {
        if config.crypto_type == Some(CryptoType::AWS.to_string()) {
            let kms_service = AwsKmsService::get_kms_service(config.clone()).map_err(|e| format!("{:?}", e)).unwrap();
            let cipher_vec = kms_service.encrypt(&seed_byte)
                .map_err(|e| format!("{:?}", e))?; // seed_base64
            //let seed_hex = "0102020078eaeb9a15eee4a08a9ca09c1f94ff682c9ffb8ff4b3a80a7bbee8bdb8c4b373d701229b983282ac6a42f5760fdb95b90dbf0000006e306c06092a864886f70d010706a05f305d020100305806092a864886f70d010701301e060960864801650304012e3011040c10a0b9a8325906f8b37150df020110802ba94e0e7bde2e171e1c7aa4f5544b203d5f75c874e3911cc292595ef79286377b9c15880a29e8f112d8f577".as_bytes();
            Ok(cipher_vec) // seed_base64
        } else {
            let seed_hex = hex::encode(seed_byte);
            println!("원본 데이터: {:?}", seed_hex.clone());
            // let seed_byte = seed_byte.to_vec();
            let key = hex::decode(key).map_err(|e| format!("{:?}", e))?;
            let iv = hex::decode(iv).map_err(|e| format!("{:?}", e))?;
            let token = encode_aes_256_gcm(seed_byte, key.as_slice(), iv.as_slice()).unwrap();
                // .map_err(|e| format!("{:?}", e))?;
            println!("암호화된 데이터: {}", encode_base64(token.as_ref()));

            let decrypted = decode_aes_256_gcm(token.as_slice(), key.as_slice(), iv.as_slice()).unwrap();
                // .map_err(|e| format!("{:?}", e))?;
            // println!("복호화된 데이터: {}", String::from_utf8_lossy(&decrypted));
            println!("복호화된 데이터: {:?}", hex::encode(decrypted.clone()));
            Ok(token)
        }
    }

    /// config.json 생성 함수
    fn generate_config_json(mut crypto_config: CryptoConfig, param_hash_key: Option<String>, param_output_format: Option<OutputFormat>) {
        // println!("{:?}", kms_service);
        let seed_byte = if crypto_config.crypto_type == Some(CryptoType::AWS.to_string()) {
            crypto_util::rand_bytes(16) // AWS 타입: 16바이트
        } else {
            crypto_util::rand_bytes(76) // LOCAL 타입: 76바이트
        };

        let cred_key_vec = crypto_util::digest("SHA-256", &seed_byte, 10)
            .map_err(|e| format!("{:?}", e)).unwrap();
        let cred_iv_vec = crypto_util::digest("MD5", &seed_byte, 10)
            .map_err(|e| format!("{:?}", e)).unwrap();
        println!("cred_key_vec: {:?}, cred_iv_vec: {:?}", hex::encode(cred_key_vec.clone()), hex::encode(cred_iv_vec.clone()));
        //println!("cred key hex length: {:?}, iv hex length: {:?}", hex::encode(cred_key_vec.clone()).len(), hex::encode(cred_iv_vec.clone()).len());
        //println!("cred key: {:?}, iv: {:?}", cred_key_vec, cred_iv_vec);
        //println!("cred key length: {:?}, iv length: {:?}", cred_key_vec.len(), cred_iv_vec.len());

        let key = crypto_config.clone().key.unwrap_or_else(|| hex::encode(cred_key_vec.clone()));
        let iv = crypto_config.clone().iv.unwrap_or_else(|| hex::encode(cred_iv_vec.clone()));
        let output_format = param_output_format.unwrap_or(OutputFormat::b64);
        println!("seed_byte {:?}", hex::encode(seed_byte.clone()));
        println!("key {:?}", key.clone());
        println!("iv {:?}", iv.clone());
        // 넘어온 Key, IV 값으로 Token 생성
        let token = generate_token(crypto_config.clone(), &seed_byte, key.as_str(), iv.as_str())
            .map_err(|e| format!("{:?}", e)).unwrap();
        // crypto_config.seed = Some(token.clone());
        // println!("{:?}", encode_base64(&token));
        println!("key: {:?}, iv: {:?}", key, iv);

        // Credential 은 넘어온 Key, IV 값으로 생성
        let mut crypto_cipher_spec = vec![CryptoCipherSpec::new(100, key.as_bytes(), Some(iv.as_bytes()), output_format.clone())];
        // println!("{}", to_string_pretty(&crypto_cipher_spec).unwrap());

        let hash_key = param_hash_key;
        if hash_key.clone().is_some() {
            crypto_cipher_spec.push(
                CryptoCipherSpec::new(400, hash_key.clone().unwrap().as_bytes(), None, output_format)
            );
        };

        // let credencials_vec = crypto_util::encrypt(Algorithm::AES256, crypto_util::to_json_bytes(&crypto_cipher_spec).as_slice(), &crypto_cipher_spec.ky, &crypto_cipher_spec.iv)
        let credencials_vec = crypto_util::encrypt_algorithm(crypto_util::to_json_bytes(&crypto_cipher_spec).as_slice(), &cred_key_vec, &cred_iv_vec)
            .map_err(|e| format!("{:?}", e)).unwrap();
        println!("credencials_vec: {:?}", encode_base64(credencials_vec.as_slice()));
        let credencials = encode_base64(credencials_vec.as_slice());
        let decrypt = crypto_util::decrypt_algorithm(decode_base64(credencials.as_str()).as_slice(), &cred_key_vec, &cred_iv_vec)
            .map_err(|e| format!("{:?}", e)).unwrap();
        println!("decrypt: {:?}", String::from_utf8(decrypt.clone()).unwrap());

        // let credencials = kms_service.encrypt(&credencials_vec)
        //     .map_err(|e| format!("{:?}", e)).unwrap();
        crypto_config.credential = Some(credencials_vec.clone());
        // println!("{:#?}", crypto_config);
        let local_config = LocalConfig {
            key: Some(key),
            iv: Some(iv),
            seed: Some(encode_base64(&token)),
            credential: Some(encode_base64(&credencials_vec)),
        };
        println!("Generate Complete!! copy and paste 'config.json' file");
        println!("==============================================================================================================================");
        println!("{}", to_string_pretty(&local_config).unwrap());
    }

    /// config.json 파일 생성기
    #[test]
    fn config_local_generator() {
        let crypto_config = CryptoConfig {
            crypto_type: Some(CryptoType::LOCAL.to_string()),
            aws_kms_key_arn: None,
            aws_access_key_id: None,
            aws_secret_access_key: None,
            // key: Some(String::from("3974b3171e27aeba4543084e3d87c83eb4e8a27dc4209488c11c43464844f8ff")),
            // key: Some(String::from("8fc40b8e8aadbf4fe4cafe16dd52e1ea4a6abada47711097d59990eb4683b0cf")),
            key: Some(String::from("794839940f4d20dac1b6508a165d1c8a69f5dcc8c7ef5466f81ce1b0244c4e3c")),
            iv: Some(String::from("00000000000000000000000000000000")),
            key_iteration: Some(10),
            iv_iteration: Some(10),
            seed: None,
            credential: None,
        };
        // hash_key = Some(String::from("1746b1e8747fdd86d1dffda64a67bd74119cecbe63985bf5121fe0eadbcce03671d58c847a9e663825981e36e41e7ac3cb98c82b99b2ec78d770584b0bc5245c"));
        // hash_key = Some(String::from("288663ad1f148d5a87af7d25947515a53fdeed65c4ddb506cf7e1aa70e6179855b114e235d9128125ec2f4be608afa276101dbe48cbf6e041ed0dd9048c3909e"));
        let hash_key = Some(String::from("6efec156e4520c35dbb47ba0bfbf11f122076372b2f7cff8871ef17bc26e18d52b6cdf90d910dd4149e1c1b93b978daa3e4f6109a61e633bc584575a02e56f23"));
        let output_format = Some(OutputFormat::default());
        // let output_format = Some(OutputFormat::h16);
        generate_config_json(crypto_config, hash_key, output_format);
    }

    /// 생성된 config.json 파일을 읽어서 CryptoSession 을 생성 및 암복호화 테스트
    #[test]
    fn local_enc_dec_test() -> Result<(), String> {
        let path_buf = path::absolute("src/resources/default/config.json").expect("Unable to get absolute path");
        println!("path_buf {:?}", path_buf);
        let path = path_buf.as_path();
        let bytes = read_file_as_bytes(path).unwrap();
        let session = match CryptoSession::of_byte(bytes.as_slice()) {
            Ok(session) => session,
            Err(e) => return Err(e.to_string()),
        };

        let encrypted = session.encrypt("Hello, world!".to_string())
            .map_err(|e| format!("{:?}", e))?;
        println!("Encrypted: {:?}", encrypted);

        let decrypted = session.decrypt(encrypted).map_err(|e| format!("{:?}", e))?;
        println!("Decrypted: {:?}", decrypted);
        Ok(())
        /*
        let session = match CryptoSession::of_byte(bytes.as_slice()) {
            Ok(session) => session,
            Err(e) => return Err(e.to_string()),
        };
        let session = match CryptoSession::create(path) {
            Ok(session) => session,
            Err(e) => return Err(e.to_string()),
        };
        let session = match CryptoSession::of_config(
            JsonConfig {
                aws_kms_key_arn: None,
                aws_access_key_id: None,
                aws_secret_access_key: None,
                seed: Some(String::from("YYI19Iy6I0dFr+vW6HWkqre9s7JuCDWbcChNas3vU0JuUtxoG60t6Uk8l63pArjfWzOp4Q1sXuE7kJQaFeoU9u96H60SVWD2OZxSOJuYgSRleEoc7U03gjFxkTCS7sntVNBLQ0xn4jNynjxyjX0B4UyH7dgnE47BXR182CXNk0/tULcR0NoC/eJ3y7+qHHpixxo2ZXtDM49gTyswAYc0Y+w0se8rNrq5")),
                credential: Some(String::from("rtdEXdoI26AYhLh92WBWPZn5dMQUXD/PkmmRAs5wXjzmCuGQdisx0n/cE2Nidkhz4GxcQzv3BWfuu20SxVZwufkkfJkSHhQGf80rwCSrXUzLIrzLGXs5vCuNeigacYur3H4zNJxz4sVyMnXuEE0MrfRb7kHLcX/ljl/SkYb0ptGCkQZN2WRUoYYq4c+NoIaFWK5jA8tozkV3R3VitRLKMw==")),
            }
        ) {
            Ok(session) => session,
            Err(e) => return Err(e.to_string()),
        };
        let session = match CryptoSession::of(String::from(""), String::from(""),String::from(""),
                                              String::from("Xw21Azd62vh8W0NAxsCkpw=="),
                                              String::from("J6WAjEbFZKay3j+j/TTbwtzJdS/fc8dCtKIOtDCeAU0f22nyZp0/RjzzvYrkt5TPnOxIzZ9hA3Y/fgZKcm4X+XG9w4puOqCdRurqohPW//8xTVtbXgSN1S/3IHnKv8ae5ah1vp5IOTRYSwS+c6N2zTptQYvhSRWjPPKTFpvv3Ifzz8zGauNHB62w38MX+9f3Hlw6H4XZnkA1g/m/AAF6eQ==")
        ) {
            Ok(session) => session,
            Err(e) => return Err(e.to_string()),
        };
        */
    }

    /// 생성된 config.json 파일을 읽어서 EnigmaSession 을 생성 및 hash 테스트
    #[test]
    fn crypto_session_hash_test() -> Result<(), String> {
        let path_buf = path::absolute("src/resources/default/config.json").expect("Unable to get absolute path");
        // println!("{:?}", path_buf);
        let path = path_buf.as_path();
        let bytes = read_file_as_bytes(path).unwrap();
        let session = match CryptoSession::of_byte(bytes.as_slice()) {
            Ok(session) => session,
            Err(e) => return Err(e.to_string()),
        };
        let hashed = session.encrypt_id("we are the champion".to_string(),400)
            .map_err(|e| format!("{:?}", e))?;
        println!("hashed: {:?}", hashed);

        Ok(())
    }

    #[test]
    fn seed_generator() {
        let generate_key = crypto_util::rand_bytes(76);
        println!("Generate generate_key {:?}", generate_key.clone());
        let seed = encode_base64(generate_key.as_slice());
        println!("seed {:?}", seed.clone());
        let seed_hex = hex::encode(generate_key.clone());
        println!("seed_hex {:?}", seed_hex.clone());

        // println!("key: {:?}", hex::encode(generate_key.clone()));
        // let cred_key_vec = crypto_util::digest("SHA-256", &generate_key, 10)
        //     .map_err(|e| format!("{:?}", e)).unwrap();
        // println!(" cred_key: {:?}", hex::encode(cred_key_vec.clone()));

        // let key = hmac::Key::new(HMAC_SHA512, &cred_key_vec);
        // let tag = hmac::sign(&key, &cred_key_vec);
        // println!("tag: {:?}", tag);
    }

    #[test]
    fn hash_key_generator() {
        let generate_hash_key = crypto_util::rand_bytes(64); //128 bytes
        println!("Generate generate_hash_key {:?}", generate_hash_key.clone());
        let hash_key = encode_base64(generate_hash_key.as_slice());
        println!("hash_key {:?}", hash_key.clone());
        let hash_key_hex = hex::encode(generate_hash_key.clone());
        println!("hash_key_hex {:?}", hash_key_hex.clone());
    }

    #[test]
    fn aes_256_gcm_key_generator() {
        // 85173a7f3a991a09f4470c37a11b3f586ab84c350b1cf27ab57f3ed9dde63997
        // 794839940f4d20dac1b6508a165d1c8a69f5dcc8c7ef5466f81ce1b0244c4e3c
        let generate_key = crypto_util::rand_bytes(32);
        println!("Generate generate_key {:?}", generate_key.clone());
        let key_hex = hex::encode(generate_key.clone());
        println!("key_hex {:?}", key_hex.clone());

        // println!("key: {:?}", hex::encode(generate_key.clone()));
        // let cred_key_vec = crypto_util::digest("SHA-256", &generate_key, 10)
        //     .map_err(|e| format!("{:?}", e)).unwrap();
        // println!(" cred_key: {:?}", hex::encode(cred_key_vec.clone()));

        // let key = hmac::Key::new(HMAC_SHA512, &cred_key_vec);
        // let tag = hmac::sign(&key, &cred_key_vec);
        // println!("tag: {:?}", tag);
    }

    #[test]
    fn decode_base64_test() {
        let data = "1i89fnbreAyUTIxXSHwFSA==";
        let decode = crypto_util::decode_base64(data);
        println!("{:?}", decode);
    }

    #[test]
    fn aes_gcm_test() {
        let key = hex::decode("85173a7f3a991a09f4470c37a11b3f586ab84c350b1cf27ab57f3ed9dde63997").unwrap();
        let iv  = hex::decode("000000000000000000000000").unwrap();
        // 1. 키 생성 (32바이트, AES-256)
        // let mut key = [0u8; 32];
        // rand::thread_rng().fill_bytes(&mut key_bytes);
        let key = Key::<Aes256Gcm>::from_slice(&key);
        let cipher = Aes256Gcm::new(key);

        // 2. 랜덤 Nonce 생성 (12바이트)
        let nonce = Nonce::from_slice(&iv);

        // 3. 평문 (Plaintext) 정의
        let plaintext = "4d7b900c31ac0f4072c6b5673740700461eab2ef3ac163a3a5a0a28bdd074cfdb06919e3cd4421c8bff3d95cbe3f8ed71d93dc3149d71745ccd041be6c6402415008d126a25df7c31421b60e".as_bytes();

        // 4. 암호화
        let ciphertext = cipher.encrypt(nonce, plaintext.as_ref())
            .expect("암호화 실패");

        println!("암호화된 데이터: {:?}", ciphertext);
        println!("암호화된 데이터: {}", encode_base64(ciphertext.as_ref()));

        // 5. 복호화
        let decrypted_text = cipher.decrypt(nonce, ciphertext.as_ref())
            .expect("복호화 실패");

        println!("복호화된 데이터: {}", String::from_utf8_lossy(&decrypted_text));
    }

    #[test]
    fn aes_256_gcm_util_test() {
        let key = hex::decode("85173a7f3a991a09f4470c37a11b3f586ab84c350b1cf27ab57f3ed9dde63997").unwrap();
        let iv  = hex::decode("000000000000000000000000").unwrap();
        // 3. 평문 (Plaintext) 정의
        let plaintext = "4d7b900c31ac0f4072c6b5673740700461eab2ef3ac163a3a5a0a28bdd074cfdb06919e3cd4421c8bff3d95cbe3f8ed71d93dc3149d71745ccd041be6c6402415008d126a25df7c31421b60e".as_bytes();

        let cipher_vec = encode_aes_256_gcm(plaintext, key.as_slice(), iv.as_slice()).unwrap();
        println!("암호화된 데이터: {}", encode_base64(cipher_vec.as_ref()));

        let decrypted = decode_aes_256_gcm(cipher_vec.as_slice(), key.as_slice(), iv.as_slice()).unwrap();
        println!("복호화된 데이터: {}", String::from_utf8_lossy(&decrypted));
    }

    #[test]
    fn aes_256_gcm_local_test() {
        let key_str = String::from("8fc40b8e8aadbf4fe4cafe16dd52e1ea4a6abada47711097d59990eb4683b0cf");
        let iv_str  = String::from("00000000000000000000000000000000");
        let key = hex::decode(key_str).unwrap();
        let iv  = hex::decode(iv_str).unwrap();
        // 3. 평문 (Plaintext) 정의
        // let plaintext = "424287d41814926e0505920e5e8d0a1f88d7cc66a0413f900a5b55c9c07394a6c73778bdd28fb468b9675771a6bc714469f25349ac64cc5fdd747442a0caf95ace61494c539fd8e53cf40212".as_bytes();
        // let plaintext = "9c31d1e25ddd5ba6d48e00e6e8042a0b024a081c5b6027cd2cfb64c0ea4b2533e7e6b5b327b155a75dceb4c7557d66ea655cb2887f112cbab0c9c5029429a0106b18e8c1f20e656e9358bc17".as_bytes();
        let plaintext = "701c692edfff3532a74f90235b3e047b487c9dab05890688c930f0a30dff3c68e9c06a0d6c108bc6c09c789727fe79a88fb1643e2f8cb47ad2256d207ad602eacc91f9cd5e95d178a0d5e238".as_bytes();

        let cipher_vec = encode_aes_256_gcm(plaintext, key.as_slice(), iv.as_slice()).unwrap();
        println!("암호화된 데이터: {}", encode_base64(cipher_vec.as_ref()));

        // let seed = decode_base64("bYNjpo28fhdGrbbT7y7zpOe4suQ4C2ScIyVKPJ22ABI8Wdg/SPAtsk1ula7pUejZWWCs4Q1hC+VrlcYZEO4ToO51H6kWUmTzPJpVPsiY2HI1KR1BuUoxhTcmx2bG6sq5BNBLFRptvm4hzjsgjyZb4R+GudZ3Qo3EUkx6g3bOw0fiAbFI1NJR++d2ne34Gyo+zkpibHoVNdpqKOMnHX0Up8Q1Vh1vOelZ");
        // let decrypted = decode_aes_256_gcm(cipher_vec.as_slice(), key.as_slice(), iv.as_slice()).unwrap();
        // let decrypted = decode_aes_256_gcm(seed.as_slice(), key.as_slice(), iv.as_slice()).unwrap();
        let decrypted = decode_aes_256_gcm(cipher_vec.as_slice(), key.as_slice(), iv.as_slice()).unwrap();
        println!("복호화된 데이터: {}", String::from_utf8_lossy(&decrypted));
    }

    #[test]
    fn decrypt_test() -> Result<(), String> {
        let cred_key = hex::decode("61fcd6ad85895d0e46a259757a3ed5054f472f049b22f40303ec3f8bae4d8d2a").unwrap();
        let cred_iv = hex::decode("204b2f52bc3d27bf0c4c60a2a185cfa0").unwrap();
        let credential_vec = decode_base64("ndvGY/0/KJJKgiwpjC3t0GJ20qMZmx02nD5KISdkbnd67xLKsL2aV/RyJRryripoc1JbT0OB2WyXZPQ+1PGkrlqR76U7T7Eunp1Prb+V9dwKCPwABCNr71yLDf5AI8b7nedrcpIB1xg+nWvYLqFJVtq7S6OJSYuoZQFDhZTAwK1Jf7MeBkrEoQhHKrlsY42ls/T5FSFtA5vBCwH/WkceWg==");
        let decrypted =
            crypto_util::decrypt_algorithm(credential_vec.as_slice(), cred_key.as_slice(), cred_iv.as_slice())
                .map_err(|e| e.to_string())?;
        let data = String::from_utf8(decrypted)
            .map_err(|e| CryptoError::SessionError(e.to_string()).to_string())?;
        println!("decrypted: {:?}", data);
        Ok(())
    }
}