# Crypto SpringBoot Test Project

AWS CodeArtifact를 사용한 Repository 관리 방법

### 1. MFA Session Token 발급

MFA 등록 참고 : https://velog.io/@dldldl1022/AWS-%EB%8B%A4%EC%A4%91%EC%9D%B8%EC%A6%9DMFA-%EC%84%A4%EC%A0%95%ED%95%98%EA%B8%B0

```shell
$ aws sts get-session-token --profile codeartifact --serial-number arn:aws:iam::339927058960:mfa/S21+ --duration-second 129600 --token-code 341119
```

### 2. AWS CLI session Profile 생성

```shell
$ aws configure set region ap-northeast-1 --profile session
```

### 3. 결과 값 ~/.aws/credentials 의 session profile로 등록

```shell
[session]
aws_access_key_id: AXXXXXXXXXXXXXXXXXXX
aws_secret_access_key: 53gXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
aws_session_token: IQoJb3JpXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX=
```


### 4. CODEARTIFACT_AUTH_TOKEN 발급 스크립트 `.zshrc`에 추가

```shell
bash -c "export CODEARTIFACT_AUTH_TOKEN=\`aws codeartifact get-authorization-token --domain crypto-dev-repo --domain-owner 123456789012 --profile session --region ap-northeast-1 --query authorizationToken --output text\` >> ~/.zshrc"
```      

### 5. Code Artifact Auth Token 발급 후 gradle.properties 파일에 저장

```shell
echo "codeartifactToken=$CODEARTIFACT_AUTH_TOKEN" > ~/.gradle/gradle.properties
```

### 6. Code Artifact Repository 추가

```groovy
repositories {
    mavenCentral()
    maven {
        url 'https://crypto-dev-repo-123456789012.d.codeartifact.ap-northeast-1.amazonaws.com/maven/crypto-dev-repo/'
        credentials {
            username "aws"
            // password System.env.CODEARTIFACT_AUTH_TOKEN
            password "$codeartifactToken"
        }
    }
}
```

### 7. Crypto Dependency 추가

```groovy
dependencies {
    implementation 'com.freelife.crypto:crypto:0.0.1'
}
```