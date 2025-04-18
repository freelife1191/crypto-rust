#[cfg(test)]
mod crypto_utils_test {
    use crate::util::crypto_util;
    use std::path::{self, Path};
    use crate::domain::crypto_config::{JsonConfig};
    use crate::util::crypto_util::from_json_path;

    use walkdir::WalkDir;

    fn list_all_files<P: AsRef<Path>>(path: P) -> Vec<String> {

        let mut files = Vec::new();

        // path 디렉토리의 모든 파일과 하위 디렉토리를 재귀적으로 탐색한다.
        let walker = WalkDir::new(path);

        // walker의 결과를 반복자로 변환한다.
        let iterator = walker.into_iter();

        // iterator의 각 항목(`e`)에 대해 오류가 발생한 항목을 필터링하고 성공한 항목만 반환한다.
        let filtered_iterator = iterator.filter_map(|e| e.ok());

        // 필터링된 결과를 entry로 하나씩 꺼내서 처리한다.
        for entry in filtered_iterator {

            // 항목의 유형이 파일인지 확인한다.
            if entry.file_type().is_file() {

                // 해당 항목의 경로를 문자열로 변환하여 files 벡터에 추가한다.
                let file_path = entry.path().display().to_string();
                files.push(file_path);
            }
        }
        files
    }

    #[test]
    fn search_directory_test() {
        println!("{:?}", list_all_files("src/resources"))
    }


    #[test]
    fn rand_bytes_test() {
        let rand_bytes = crypto_util::rand_bytes(64);
        // rand_bytes.iter().for_each(|b| print!("{:02x}", b));
        println!("{:?}", hex::encode(rand_bytes));
        // assert_eq!(rand_bytes.len(), 16);
    }

    #[test]
    fn from_json_path_test() {
        let path_buf = path::absolute("src/resources/local/config.json").expect("Unable to get absolute path");
        let path = path_buf.as_path();
        let config: JsonConfig = match crypto_util::from_json_path(path) {
            Ok(config) => config,
            Err(e) => panic!("Error: {}", e),
        };
        println!("{:#?}", config);
    }

    #[test]
    fn from_json_byte_test() {
        let path_buf = path::absolute("src/resources/local/config.json").expect("Unable to get absolute path");
        let path = path_buf.as_path();
        let config: JsonConfig = match from_json_path(path) {
            Ok(config) => config,
            Err(e) => panic!("Error: {}", e),
        };
        // println!("{:#?}", &config);
        let config_vec = serde_json::to_vec(&config).expect("Serialization failed");
        // println!("config_byte: {:?}", config_byte);
        let json_config: JsonConfig = crypto_util::from_json_byte(config_vec).unwrap();
        println!("{:#?}", json_config);
    }

    /*
    #[test]
    fn path_to_string_test() {
        let path = path::absolute("src/resources/local/config.json").expect("Unable to get absolute path");
        let path_str = crypto_util::path_to_string(path.as_path());
        // assert_eq!(path_str, "src/resources/config.json");
        println!("{}", path_str);
    }

    #[test]
    fn path_to_json_test() {
        let path = path::absolute("src/resources/local/config.json").expect("Unable to get absolute path");
        let json = crypto_util::path_to_json(path.as_path());
        println!("{:?}", json);
    }

    #[test]
    fn string_to_path_test() {
        let path_str = "src/resources/local/config.json";
        let path = crypto_util::string_to_path(path_str);
        println!("{:?}", path);
    }
    */

    #[test]
    fn aes_encrypt_test() {
        // let key = hex::decode("fd9f59a521204e76baeed2f13c0ef241").unwrap();
        let key = hex::decode("d38a2585e3e885f5c48da5c5073aed8f").unwrap();
        // let key = hex::decode("78fc0dd6365695d011f763fd4ca8c0e3").unwrap();
        let iv = hex::decode("00000000000000000000000000000000").unwrap();
        // let iv = String::from("00000000000000000000000000000000");
        // let iv_byte = hex::decode(iv).unwrap();
        // let data = "test".as_bytes();
        let data = "".as_bytes();
        let encrypted = crypto_util::encrypt_algorithm(data, &key , &iv).unwrap();
        println!("{:?}", hex::encode(encrypted));
    }

    #[test]
    fn aes_decrypt_test() {
        // let key = hex::decode("fd9f59a521204e76baeed2f13c0ef241").unwrap();
        let key = hex::decode("d38a2585e3e885f5c48da5c5073aed8f").unwrap();
        // let key = hex::decode("78fc0dd6365695d011f763fd4ca8c0e3").unwrap();
        let iv = hex::decode("00000000000000000000000000000000").unwrap();
        // let iv = String::from("00000000000000000000000000000000");
        // let iv_byte = hex::decode(iv).unwrap();
        // let data = hex::decode("415901322fd63707d84651e1c3b4f7f2").unwrap();
        // let data = hex::decode("975fbd635ed0d77a1e369907c9282642").unwrap();
        // let data = hex::decode("83a7bb41d1d9217fd62bb66aef4970ab").unwrap();
        let data = hex::decode("56c765d37ded34e10509dde0af2b3ca2").unwrap();
        let decrypted = crypto_util::decrypt_algorithm(&data, &key , &iv).unwrap();
        println!("{:?}", String::from_utf8(decrypted).unwrap());
    }

    #[test]
    fn base64_test() {
        let hash = hex::decode("dccc5ae98a89d781e6fafdef2039100832290073a73c9528d676a0f28739c5feb76556f61968b926aacd9d784c3079c207caab6df374bfa919bb8828e05bcac0").unwrap();
        // let base = BASE64_STANDARD.encode(hash);
        // let hex = hex::encode(BASE64_STANDARD.decode(base.as_str()).unwrap());
        println!("hex: {:?}", hex::encode(hash));
    }

    #[test]
    fn b64_dec_test() {
        let dec = crypto_util::decode_base64("P9x6fxEJ8IuWPCBSjYKRbw==".to_string().as_str());
        // 3fdc7a7f1109f08b963c20528d82916f
        // let dec = crypto_util::decode_base64("5YTAflqL2EJKEAqofsB7pw==".to_string().as_str());
        // e584c07e5a8bd8424a100aa87ec07ba7
        println!("{:?}", hex::encode(dec));
    }

    #[test]
    fn b64_enc_test() {
        let enc = crypto_util::encode_base64(crypto_util::rand_bytes(16).as_slice());
        println!("{:?}", enc);
    }
}