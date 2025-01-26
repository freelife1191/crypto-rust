# Encryption Algorithm

## Encryption Config Generation

---

암호화 솔루션 Config 파일을 생성하는 과정

### 1. seed_byte Generation

76 byte 의 랜덤 바이트 생성

동일한 `seed_byte`를 사용하면 항상 동일한 **Credential Key** 와 **Credential IV** 가 생성됨

### 2. Credential Key Generation

`seed_byte` 를 `SHA-256` 해시 알고리즘을 사용해 10라운드 digest 갱신을 통해 256 bit 로 만들어진 64 byte **Credential Key** 를 생성

### 3. Credential IV Generation

`seed_byte` 를 MD5 해시 알고리즘을 사용해 10라운드 digest 갱신을 통해 128 bit 로 만들어진 32 byte **Credential IV(Initialization Vector)** 를 생성

### 4. seed Encryption

#### AWS 타입
**AWS KMS** 로 `seed_byte` 를 암호화 함, KMS 키와 `AWS ACCESS KEY`, `AWS SECRET ACCESS KEY`로 **AWS KMS** 에 접속해 세션을 만들고  
`SYMMETRIC_DEFAULT` 키 사양을 사용해 `AES-256-GCM` 대칭 알고리즘으로 데이터를 암호화함  
336 바이트의 암호화된 `seed` 를 생성  

#### LOCAL 타입
전달받은 Config 의 64 byte **Secret Key** 와 32 byte **IV(Initialization Vector)** 로 `seed_byte` 를  
**LOCAL** 환경에서 `AES-256-GCM` 대칭 알고리즘으로 암호화함  
152 바이트의 암호화된 `seed` 를 생성

### 5. Credential Generation

아래의 Spec 으로 **Cipher Spec** JSON 데이터를 생성한다

- **Cipher Spec**
  - `Algorithm`: `AES`
  - `block Mode`: `CBC`
  - `Padding Mode`: `PKCS7`
  - `OutputFormat`: `Base64`, `Hex`
  - `Key`: Config 의 32 byte **Secret Key**
  - `IV`: Config 의 16 byte **IV(Initialization Vector)**

### 6. Credential Encryption

**Credential Key**와 **Credential IV**로 **Cipher Spec**에 기재된대로 `AES256-CBC-PKCS7` 알고리즘으로 암호화 한다 
이때 **Credential Key**는 64 byte `hex` 문자열을 **Vector**로 변환하면 32 byte **KEY**로 `AES256` KEY 조건에 부합하게 된다  
**Credential IV** 역시 32byte 의 `hex` 문자열을 **Vector**로 변환하면 16 byte 의 **IV**로 `AES256` IV 조건에 부합하게 된다

최종적으로 암호화된 `Seed` 와 `Credential` 를 `Base64`로 Encoding 하여 Config 파일로 제공한다

#### AWS 타입

**AWS TYPE**에서는 `seed` 는 단지 KMS 를 통해서만 암호화 하며 **Secret Key**와 **IV** 값은 Config 생성시 **Cipher Spec** 에만 포함시켜
데이터를 암복호화할때만 사용된다 그러므로 **AWS TYPE**의 경우 **Secret Key**와 **IV** 값이 절대 외부에 공개되서는 안되는 매우 중요한 정보이다

#### LOCAL 타입

**LOCAL TYPE**에서는 `seed_byte` 를 암/복호화하기 위해 반드시 **Secret Key**와 **IV** 값을 `hex`로 Encoding 하여 Config 정보로 포함시켜줘야된다

**LOCAL TYPE**의 경우 편의상 고정된 `seed` 암/복호화를 위해서 **Secret Key**와 **IV**값을 사용하며 동일한 **Secret Key**와 **IV**값을 **Cipher Spec** 에 포함시켜
데이터를 암복호화할때도 사용한다

### 8. Config Generation

위에서 생성된 데이터 중 `key`, `iv`, `seed`, `credential` 값을 Config 파일로 생성한다

- key: `seed` 암/복호화 및 데이터 암/복호화에 사용되는 **Secret Key**
- iv: `seed` 암/복호화 및 데이터 암/복호화에 사용되는 **IV(Initialization Vector)**
- seed: `seed_byte` 를 암호화한 값
  - `AWS TYPE`의 경우 `AWS KMS`로 `seed_byte`를 암호화 데이터
  - `LOCAL TYPE`의 경우 `LOCAL` 환경에서 **Secret Key**와 **IV**로 AES-256-GCM 대칭 알고리즘으로 `seed_byte`를 암호화한 데이터
- credential
  - **Credential Key**와 **Credential IV**로 **Cipher Spec** 데이터를 `AES256-CBC-PKCS7` 대칭 알고리즘으로 암호화한 데이터
  - 결론적으로 해당 암호화 솔루션은 `credential` 데이터를 복호화 한 **Cipher Spec** 으로 데이터 암/복호화를 수행한다

## Encryption Algorithm Usage

---

암호화 솔루션 사용시 처리 과정

## 1. Loading Config & Creation Session

Config 파일을 로드하여 설정 정보를 읽어들여 암호화 솔루션 세션을 생성한다

설정 정보를 읽어들여 **Credential** 데이터를 복호화하고 **Cipher Spec** 정보를 얻어 데이터 암/복호화를 수행한다

## 2. Seed Decryption

#### AWS 타입
**AWS KMS** 로 `seed_byte` 를 복호화 함, KMS 키와 `AWS ACCESS KEY`, `AWS SECRET ACCESS KEY`로 AWS KMS 에 접속해 세션을 만들고    
`SYMMETRIC_DEFAULT` 키 사양을 사용해 `AES-256-GCM` 대칭 알고리즘으로 데이터를 복호화함  
336 바이트의 암호화된 `seed` 를 생성  

#### LOCAL 타입
전달받은 Config 의 32 byte **Secret Key** 와 16 byte **IV(Initialization Vector)** 로 `seed_byte` 를  
**LOCAL** 환경에서 `AES-256-GCM` 대칭 알고리즘으로 복호화함  
152 바이트의 암호화된 `seed` 를 생성

### 3. Credential Generation

`seed_byte` 를 `SHA-256` 해시 알고리즘을 사용해 10라운드 digest 갱신을 통해 256 bit 로 만들어진 64 byte **key** 를 생성  
Config 파일 생성 시 사용했던 `seed_byte`와 동일한 `seed_byte`를 사용하면 항상 동일한 **Credential Key** 가 생성됨  

### 4. Credential IV Generation

`seed_byte` 를 MD5 해시 알고리즘을 사용해 10라운드 digest 갱신을 통해 128 bit 로 만들어진 32 byte **IV(Initialization Vector)** 를 생성  
Config 파일 생성 시 사용했던 `seed_byte`와 동일한 `seed_byte`를 사용하면 항상 동일한 **Credential IV** 가 생성됨

### 5. Credential Decryption

**Credential Key**와 **Credential IV**로 **Cipher Spec** JSON 데이터를 **Cipher Spec** 에 기재된대로 `AES256-CBC-PKCS7` 알고리즘으로 복호화 한다  
본 암호화 솔루션은 **Cipher Spec** 에 기재된 알고리즘이 다르면 다른 알고리즘으로 암/복호화를 진행하도록 구성되어 있다

### 6. Encryption/Decryption

이후 복호화된 **Cipher Spec** 의 **Secret Key**와 **Secret IV**로 **AES256-CBC-PKCS7** 알고리즘을 사용하여 데이터를 암호화/복호화 한다