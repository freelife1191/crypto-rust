# crypto-rust

DB 암호화 솔루션 외부 공개용 프로젝트 입니다  
외부 공개용 프로젝트에서는 AWS KMS 를 사용하지 않고 LOCAL 에서만 암호화/복호화를 수행합니다

본 프로젝트는 Rust 로 개발된 LOCAL 암호화 라이브러리를 JNI 로 빌드할 수 있도록 구성되었습니다

Cross Compile 을 위해 `cross` 를 사용해 컴파일하며 `podman` 을 사용하여 Docker 환경을 구성하고 `flapigen` 을 사용하여 Rust FFI 를 생성합니다

위의 과정을 Gradle Task 로 구성하여 쉽고 빠르게 빌드할 수 있도록 구성되었습니다

이 암호화 솔루션은 의존성을 최소화하여 순수 Java JNI 라이브러리 구성으로 Java Application 에서 쉽고 빠르게 적용하고 사용할 수 있도록 구성되었으며   
강력하고 안전하게 암호화를 수행합니다

또한 성능도 뛰어나서 Application 에서 암호화/복호화를 수행할 때 성능 저하가 거의 없습니다

아래의 성능 테스트 결과를 참고하시기 바랍니다

## 성능 테스트 결과

아래의 성능 테스트 결과를 보면 암호화/복호화를 수행할 때 성능 저하가 거의 없음을 확인할 수 있습니다

- 성능 테스트 조건
  - 암호화 테스트 (100 RPS, 100 건 Bulk Insert)
  - 복호화 테스트 (100 RPS, 1000 ROW Select)

| 등록시 암호화 미적용      | 조회시 복호화 미적용       |
|:-----------------|:------------------|
| 99.5 / 100 RPS   | 93.9 / 100 RPS    |
| 최소 14ms, 평균 23ms | 최소 60ms, 평균 120ms |

| 등록시 암호화 적용       | 조회시 복호화 적용       |
|:-----------------|:-----------------|
| 99.9 / 100 RPS   | 95.3 / 100 RPS   |
| 최소 17ms, 평균 27ms | 최소 44ms, 평균 79ms |


## Specification

- [Rust 1.83.0](https://www.rust-lang.org/)
- [Gradle 8.12.1](https://gradle.org/)
- Java 9+
- [Podman 5.3.2](https://podman.io/)
- [flapigen 0.8.0](https://github.com/Dushistov/flapigen-rs)
- [cross 0.2.5](https://github.com/cross-rs/cross)

## 프로젝트에서 사용되는 암호화 알고리즘

- AES256-GCM
- AES256-CBC
- SHA256
- MD5

## 지원되는 MultiPlatform

- **Windows**
    - x86_64(AMD64)
- **Linux**
    - x86_64(AMD64)
    - AARCH64(ARM64)
- **MacOS**
    - Apple Silicon(ARM64)

## Rust 프로젝트 구성

- `crypto-build`
    - Cross Compile MultiPlatform JNI Build Module
- `crypto-lib`
    - Rust Library Core
- `flapigen`
    - Rust FFI Generator