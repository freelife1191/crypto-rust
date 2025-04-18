# AWS Code Artifact Publish/Subscribe

https://aws.amazon.com/ko/codeartifact/

- 참고
  - https://docs.gradle.org/current/userguide/declaring_repositories.html
  - https://docs.aws.amazon.com/ko_kr/codeartifact/latest/ug/maven-gradle.html
  - https://docs.gradle.org/current/userguide/publishing_maven.html
  - https://velog.io/@leeseojune53/build.gradle%EB%A5%BC-%EC%84%B8%EC%84%B8%ED%95%98%EA%B2%8C-%ED%8C%8C%EB%B3%B4%EC%9E%90
  - https://stackoverflow.com/questions/15233935/how-do-i-release-with-bitbucketgitmaven
  - https://cloudest.oopy.io/posting/101
  - https://junhyeong-jang.tistory.com/4


### Code Artifact 사용을 위한 AWS CLI 설정

```shell
# MFA Session Token 발급
aws sts get-session-token --profile codeartifact --serial-number arn:aws:iam::123456789012:mfa/S21+ --duration-second 129600 --token-code 341119

# MFA 및 Code Artifact 용 AWS Config 생성 `~/.aws/config`
[artifactAccount]
mfa_arn = arn:aws:iam::123456789012:mfa/S21+
output = json
[profile artifact]
user_arn = arn:aws:iam::123456789012:user/ts1180
source_profile = sessionAccount
region = ap-northeast-1

# AWS CLI Credential Profile 생성
[artifact]
aws_access_key_id = AXXXXXXXXXXXXXXXXXX
aws_secret_access_key = 53XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX

# MFA Session Token 발급
aws sts get-session-token --serial-number arn:aws:iam::123456789012:mfa/test --token-code MFA-NUMBER –-profile artifactAccount

# AWS CLI session Profile 생성
aws configure set region ap-northeast-1 --profile session

# 결과 값 ~/.aws/credentials 의 session profile로 등록
[session]
aws_access_key_id: AXXXXXXXXXXXXXXXXXX
aws_secret_access_key: 53XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
aws_session_token: IQoJb3JpXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX=

# CODEARTIFACT_AUTH_TOKEN 스크립트 `~/.zshrc`에 등록 (session profile 사용)
bash -c "export CODEARTIFACT_AUTH_TOKEN=\`aws codeartifact get-authorization-token --domain crypto-dev-repo --domain-owner 123456789012 --profile session --region ap-northeast-1 --query authorizationToken --output text\` >> ~/.zshrc"
```


### Gradle Publish 설정

```groovy
plugins {
    id 'maven-publish'
}

publishing {
  publications {
      mavenJava(MavenPublication) {
          groupId = 'com.freelife.crypto'
          artifactId = 'crypto'
          version = '0.1.0.RC1'
          from components.java

          pom {
              name = "CryptoSession"
              description = "Crypto Session Database Encryption/Decryption Library"
              url = "https://freelife.com"
              licenses {
                  license {
                      name = 'The Apache License, Version 2.0'
                      url = 'http://www.apache.org/licenses/LICENSE-2.0.txt'
                  }
              }
              developers {
                  developer {
                      id = "freelife"
                      name = "Free Life"
                      email = "freelife@gmail.com"
                  }
              }
              scm {
                  connection = "scm:git:git://github.com/freelife1191/crypto-rust.git"
                  developerConnection = "scm:git:ssh://github.com/freelife1191/crypto-rust.git"
                  url = "https://github.com/freelife1191/crypto-rust"
              }
          }
      }
  }
  repositories {
      maven {
          url 'https://crypto-dev-repo-123456789012.d.codeartifact.ap-northeast-1.amazonaws.com/maven/crypto-dev-repo/'
          credentials {
              username "aws"
              password System.env.CODEARTIFACT_AUTH_TOKEN
              //password project.ext.CODEARTIFACT_AUTH_TOKEN
          }
      }
  }
}
```


### 게시 확인
```shell
aws codeartifact list-package-versions --domain my_domain --domain-owner 123456789012 --repository crypto-dev-repo --format maven\
--namespace com.freelife.crypto --package crypto
```



### Application Code Artifact에 Gradle 연동

```shell
# CODEARTIFACT_AUTH_TOKEN 발급 (session profile 사용)
export CODEARTIFACT_AUTH_TOKEN=`aws codeartifact get-authorization-token --domain crypto-dev-repo --domain-owner 123456789012 --profile session --region ap-northeast-1 --query authorizationToken --output text`

# gradle.properties 에 CODEARTIFACT_AUTH_TOKEN 등록\
echo "codeartifactToken=$CODEARTIFACT_AUTH_TOKEN" > ~/.gradle/gradle.properties
```

```groovy
// gradle repository 추가 
repositories {
    maven {
        url 'https://crypto-dev-repo-123456789012.d.codeartifact.ap-northeast-1.amazonaws.com/maven/crypto-dev-repo/'
        credentials {
            username "aws"
            // password System.env.CODEARTIFACT_AUTH_TOKEN
            password "$codeartifactToken"
        }
    }
}

// gradle dependency 추가
dependencies {
    implementation 'com.freelife.crypto:crypto:0.1.0.RC1'
}
```