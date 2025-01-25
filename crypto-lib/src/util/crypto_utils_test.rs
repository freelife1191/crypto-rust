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
        let rand_bytes = crypto_util::rand_bytes(16);
        rand_bytes.iter().for_each(|b| print!("{:02x}", b));
        // println!("{:?}", rand_bytes);
        assert_eq!(rand_bytes.len(), 16);
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
}