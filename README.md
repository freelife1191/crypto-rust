# crypto-rust

사내 DB 암호화 솔루션 내재화를 위해 개발된 프로젝트로 외부 공개를 위해 AWS KMS를 사용하지 않고  
LOCAL에서 암호화/복호화를 수행하도록 개발된 Rust 기반의 암호화 라이브러리입니다

본 프로젝트는 Rust 기반의 LOCAL 암호화 라이브러리를 JNI 로 빌드할 수 있도록 구성됨

- `crypto-build`
  - Cross Compile MultiPlatform JNI Build Module
- `crypto-lib`
  - Rust Library Core
- `flapigen`
  - Rust FFI Generator