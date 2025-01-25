# Gradle 기반에 Crypto Rust Java Native Interface Library Project

- 예제 블로그
  - https://www.infinyon.com/blog/2021/05/java-client/#setup
- 예제 코드
  - https://github.com/infinyon/fluvio-client-java

### Cargo Other Languages Support Build Tool

- https://crates.io/crates/flapigen
- https://crates.io/crates/rifgen
- https://crates.io/crates/rust_swig/0.3.0


## 필수 설치 프로그램

### Cross Compile
  - https://podman.io
    - podman desktop 설치 후 Machine 생성이 필요
  - https://github.com/cross-rs/cross
    - 아래의 Command로 cross 설치 
    - `cargo install cross --git https://github.com/cross-rs/cross`
    - 설정 참고
      - https://github.com/cross-rs/cross/wiki
      - https://github.com/cross-rs/wiki_assets

### Code Artifact

```shell
$ brew install awscli
```


```shell


## 환경 변수 설정 및 Command

### 실습시 사용한 환경 변수 설정

DYLD_FALLBACK_LIBRARY_PATH 설정

```shell
# XCode 4.3 이전 설정 
export DYLD_FALLBACK_LIBRARY_PATH="$(xcode-select --print-path)/Toolchains/XcodeDefault.xctoolchain/usr/lib/" && cargo build --all --target=x86_64-apple-darwin

# XCode 4.3 이후 설정
#export DYLD_FALLBACK_LIBRARY_PATH="/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/lib/"
# 개인적으로는 위의 위치에도 존재하지 않아 아래의 설정을 적용
export DYLD_FALLBACK_LIBRARY_PATH="$(xcode-select --print-path)/usr/lib/"
```

JAVA_HOME 설정

```shell
# JAVA_HOME 설정
export JAVA_HOME=$(/usr/libexec/java_home -v 21)
```

Rust Rover 빌드시 `libclang.dylib`를 계속 찾음

```shell
# Rust 에서 빌드를 위해 계속 libclang.dylib 파일을 찾는데 아래의 경로에 COPY 해주면 정상적으로 빌드됨

sudo mkdir -p /usr/local/lib 
sudo cp $(xcode-select --print-path)/usr/lib/libclang.dylib /usr/local/lib/libclang.dylib
```
## Command

---

### Build Command

Cargo Build

```shell
# 위의 DYLD_FALLBACK_LIBRARY_PATH, JAVA_HOME 모두 .zshrc 환경변수에 추가하고 cargo build 수행
RUST_BACKTRACE=full cargo build -vv
```

### Gradle Command

```shell
# Gradle Test (잘 빌드가 되는지 테스트)
./gradlew test

# Gradle Clean
./gradlew clean -x test

# Java 파일이 정상적으로 Generate가 안되면 `java_glue.rs.in` 파일에 변화를 주고 다시 빌드하면 잘 생성됨
./gradlew build -x test publish && jar tf lib/build/libs/lib.jar

# JDK 1.8 버전으로 빌드
./gradlew build -x test publish -PjavaVersion=1.8
```

Window 

```shell
# lib-x86_64-window
gradlew.bat build -x test && jar tf lib/build/libs/lib.jar
```

위의 Gradle 빌드로 `lib.jar` 를 생성 아래의 경로에 `lib.jar` 파일이 생성됨

`crypto-rust/lib/build/libs/lib.jar`

### Oracle 용 JNI Jar 파일 생성

```shell
./gradlew :oracle-lib:build && jar tf oracle-lib/build/libs/CryptoOracle.jar
```


### SpringBoot 테스트

- `crypto-spring-test` 스프링부트 테스트 프로젝트가 추가되어 있음
- 해당 프로젝트의 `libs` 경로에 변경된 `lib.jar` 파일을 적용하면서 테스트
- 적용후 Gradle 재빌드가 필요함 
- 적용이 잘안되면 Gradle dependencies 설정을 잠깐 변경해서 빌드후 다시 변경해서 빌드하면 적용됨


### Gradle Command 참고

- https://docs.gradle.org/current/userguide/command_line_interface.html
- https://docs.gradle.org/current/userguide/command_line_interface_basics.html

### Jar Command

https://m.blog.naver.com/duoh20/222043010528

| option | description         | etc                                       |
| :----- | :------------------ | :---------------------------------------- |
| `-c`     | create              | 신규 jar 파일 생성                        |
| `-t`     | list                | 지정한 jar 파일의 목록 출력               |
| `-u`     | update              | 기존 jar 파일을 업데이트                  |
| `-x`     | extract             | jar 파일에서 지정한 파일을 추출           |
| `-f`     | file                | jar 파일 이름 지정                        |
| `-i`     | generate index FILE | 인덱스 정보 출력                          |
| `-C`     | DIR                 | 지정 파일 포함하여 지정된 디렉터리로 변경 |
| `-v`     | verbose             | 상세한 정보 출력                          |
| `-m`     | manifest FILE       | manifest 파일의 manifest 정보 포함        |