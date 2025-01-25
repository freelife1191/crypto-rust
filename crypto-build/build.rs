use flapigen::{JavaConfig, JavaReachabilityFence, LanguageConfig};
use std::{
    env, fs,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

fn main() {
    env_logger::init();

    let out_dir = env::var("OUT_DIR").unwrap();
    let jni_c_headers_rs = Path::new(&out_dir).join("jni_c_header.rs");
    gen_jni_bindings(&jni_c_headers_rs);

    let have_java_9 = fs::read_to_string(&jni_c_headers_rs)
        .unwrap()
        .contains("JNI_VERSION_9");

    // let project_root = env::var("CARGO_MANIFEST_DIR").unwrap();
    let java_cfg = JavaConfig::new(

        Path::new("lib")
            .join("src")
            .join("main")
            .join("java")
            .join("com")
            .join("freelife")
            .join("crypto")
            .join("core"),
        "com.freelife.crypto.core".into())
        // .use_null_annotation_from_package("jakarta.annotation".into())
        // .use_optional_package("com.fasterxml.jackson.databind.ObjectMapper".into())
        // .use_optional_package("com.fasterxml.jackson.databind.SerializationFeature".into())
        // .use_optional_package("java.util.List".into())
    // .use_optional_package("java.util".into())
    // .use_optional_package("com.google.common.base".into())
    // .use_optional_package("com.hadisatrio.optional".into())
    .use_reachability_fence(if have_java_9 {
        JavaReachabilityFence::Std
    } else {
        JavaReachabilityFence::GenerateFence(8)
    });

    //.use_null_annotation_from_package("android.support.annotation".into())

    eprintln!("java_cfg {:?}", java_cfg);

    let out_dir = env::var("OUT_DIR").unwrap();
    let in_src = Path::new("src").join("java_glue.rs.in");
    let out_src = Path::new(&out_dir).join("java_glue.rs");
    // Generator::new(TypeCases::CamelCase, Language::Java, "src").generate_interface(&in_src);
    let flap_gen = flapigen::Generator::new(LanguageConfig::JavaConfig(java_cfg))
        .merge_type_map("typemaps", include_str!("src/jni_typemaps.rs"));
        // .remove_not_generated_files_from_output_directory(true)
        // .rustfmt_bindings(true);
    flap_gen.expand("java bindings", &in_src, &out_src);
    eprintln!("cargo:rerun-if-changed={}", in_src.display());
}

fn gen_jni_bindings(jni_c_headers_rs: &Path) {
    let java_home = env::var("JAVA_HOME").expect("JAVA_HOME env variable not settted");
    let java_include_dir = Path::new(&java_home).join("include");
    let target = env::var("TARGET").expect("target env var not setted");

    let java_sys_include_dir = java_include_dir.join(if target.contains("windows") {
        "win32"
    } else if target.contains("darwin") {
        "darwin"
    } else {
        "linux"
    });
    // eprintln!("{:?}", list_all_files(&java_include_dir));
    // eprintln!("{:?}", list_all_files("/usr/lib/jvm"));

    let include_dirs = [java_include_dir, java_sys_include_dir];
    eprintln!("jni include dirs {:?}", include_dirs);

    let jni_h_path = match search_file_in_directory(&include_dirs[..], "jni.h") {
        Ok(path) => path,
        Err(_) => panic!("jni.h not found"),
    };
    // .expect(list_all_files("/usr/lib/jvm").join("\n").as_str());
    eprintln!("cargo:rerun-if-changed={}", jni_h_path.display());

    gen_binding(&include_dirs[..], &jni_h_path, jni_c_headers_rs).expect("gen_binding failed");
}

#[allow(unused)]
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

fn search_file_in_directory<P: AsRef<Path>>(dirs: &[P], file: &str) -> Result<PathBuf, ()> {
    for dir in dirs {
        let dir = dir.as_ref().to_path_buf();
        let file_path = dir.join(file);
        if file_path.exists() && file_path.is_file() {
            return Ok(file_path);
        }
    }
    Err(())
}

fn gen_binding<P: AsRef<Path>>(
    include_dirs: &[P],
    c_file_path: &Path,
    output_rust: &Path,
) -> Result<(), String> {
    let mut bindings: bindgen::Builder = bindgen::builder().header(c_file_path.to_str().unwrap());

    bindings = include_dirs.iter().fold(bindings, |acc, x| {
        acc.clang_arg("-I".to_string() + x.as_ref().to_str().unwrap())
    });

    let generated_bindings = bindings
        .generate()
        .map_err(|_| "Failed to generate bindings".to_string())?;

    generated_bindings
        .write_to_file(output_rust)
        .map_err(|err| err.to_string())?;

    Ok(())
}
