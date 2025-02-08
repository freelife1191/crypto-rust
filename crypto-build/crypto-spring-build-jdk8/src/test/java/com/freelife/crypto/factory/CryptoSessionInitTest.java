package com.freelife.crypto.factory;

import com.freelife.crypto.config.ConfigProperties;
import com.freelife.crypto.config.CryptoSessionType;
import com.freelife.util.JsonUtils;
import com.google.common.collect.ImmutableMap;
import org.junit.jupiter.api.*;
import org.springframework.core.io.ClassPathResource;
import org.springframework.core.io.Resource;

import java.io.IOException;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.Map;

import static com.freelife.util.ByteUtils.toByteArray;
import static org.assertj.core.api.Assertions.assertThat;
import static org.junit.jupiter.api.Assertions.assertAll;

/**
 * CryptoSessionInit 초기화 객체 테스트
 * Created by mskwon on 2024. 10. 22..
 */
@TestMethodOrder(MethodOrderer.OrderAnnotation.class)
public class CryptoSessionInitTest {

    @Test
    @Order(1)
    @DisplayName("ClassPathResource Test")
    void classPathResourceTest() throws IOException {
        Resource resource = new ClassPathResource(Paths.get("crypto","default", "config.json").toString());
        System.out.println(resource.getFile().getAbsoluteFile());
    }

    @Test
    @Order(2)
    @DisplayName("BASE_PATH 타입 CryptoSessionInit 객체 생성 테스트")
    void basePathTest() {
        // HOTEL CryptoSession 생성
        CryptoSessionInit<CryptoSessionType> basePathInit = CryptoSessionInit.ofBasePath(CryptoSessionType.HOTEL);
        assertThat(basePathInit.getInitType()).isEqualTo(CryptoSessionInit.InitType.BASE_PATH);
        System.out.println(basePathInit);
    }

    @Test
    @Order(3)
    @DisplayName("BYTES 타입 CryptoSessionInit 객체 생성 테스트")
    void bytesTest() throws IOException {
        // HOTEL CryptoSession 생성
        // byte[] bytes = ConfigProperties.getClassPathResource(CryptoSessionType.HOTEL).getInputStream().readAllBytes();
        ClassPathResource resource = new ClassPathResource(Paths.get("crypto", "hotel", "config.json").toString());
        CryptoSessionInit<CryptoSessionType> bytesInit = CryptoSessionInit.ofBytes(
                CryptoSessionType.HOTEL,
                toByteArray(resource.getInputStream())
        );
        assertThat(bytesInit.getInitType()).isEqualTo(CryptoSessionInit.InitType.BYTES);
        assertAll(
                () -> assertThat(bytesInit.getConfigBytes()).isNotNull()
        );
    }

    @Test
    @Order(4)
    @DisplayName("INPUT_STREAM 타입 CryptoSessionInit 객체 생성 테스트")
    void inputStreamTest() throws IOException {
        // HOTEL CryptoSession 생성
        ClassPathResource resource = new ClassPathResource(Paths.get("crypto", "hotel", "config.json").toString());
        CryptoSessionInit<CryptoSessionType> inputStreamInit = CryptoSessionInit.ofInputStream(
                CryptoSessionType.HOTEL,
                resource.getInputStream()
        );
        assertThat(inputStreamInit.getInitType()).isEqualTo(CryptoSessionInit.InitType.INPUT_STREAM);
        assertAll(
                () -> assertThat(inputStreamInit.getConfigInputStream()).isNotNull()
        );
    }

    @Test
    @Order(5)
    @DisplayName("PATH 타입 CryptoSessionInit 객체 생성 테스트")
    void pathTest() throws IOException {
        // HOTEL CryptoSession 생성
        Resource resource = new ClassPathResource(Paths.get("crypto", "hotel", "config.json").toString());
        CryptoSessionInit<CryptoSessionType> pathInit = CryptoSessionInit.ofPath(
                CryptoSessionType.HOTEL,
                resource.getFile().getPath());
        assertThat(pathInit.getInitType()).isEqualTo(CryptoSessionInit.InitType.PATH);
        assertAll(
                () -> assertThat(pathInit.getPath()).isEqualTo(resource.getFile().getPath())
        );
    }

    @Test
    @Disabled
    @Order(6)
    @DisplayName("MAP 타입 CryptoSessionInit 객체 생성 테스트")
    void mapTest() throws IOException {
        // HOTEL CryptoSession 생성
        Resource resource = new ClassPathResource(Paths.get("crypto", "hotel", "config.json").toString());
        ConfigProperties configProperties = JsonUtils.getObjectMapper().readValue(resource.getFile(), ConfigProperties.class);

        CryptoSessionInit<CryptoSessionType> mapInit = CryptoSessionInit.ofMap(
                CryptoSessionType.HOTEL,
                ImmutableMap.of(
                        "aws_kms_key_arn", configProperties.getAwsKmsKeyArn(),
                        "aws_access_key_id", configProperties.getAwsAccessKeyId(),
                        "aws_secret_access_key", configProperties.getAwsSecretAccessKey(),
                        "seed", configProperties.getSeed(),
                        "credential", configProperties.getCredential()
                )
        );
        assertThat(mapInit.getInitType()).isEqualTo(CryptoSessionInit.InitType.MAP);
        assertAll(
                () -> assertThat(mapInit.getConfigMap().get("aws_kms_key_arn")).isEqualTo(configProperties.getAwsKmsKeyArn()),
                () -> assertThat(mapInit.getConfigMap().get("aws_access_key_id")).isEqualTo(configProperties.getAwsAccessKeyId()),
                () -> assertThat(mapInit.getConfigMap().get("aws_secret_access_key")).isEqualTo(configProperties.getAwsSecretAccessKey()),
                () -> assertThat(mapInit.getConfigMap().get("seed")).isEqualTo(configProperties.getSeed()),
                () -> assertThat(mapInit.getConfigMap().get("credential")).isEqualTo(configProperties.getCredential())
        );
    }

    @Test
    @Order(7)
    @DisplayName("LOCAL_MAP 타입 CryptoSessionInit 객체 생성 테스트")
    void localMapTest() throws IOException {
        // HOTEL CryptoSession 생성
        Resource resource = new ClassPathResource(Paths.get("crypto", "hotel", "config.json").toString());
        ConfigProperties configProperties = JsonUtils.getObjectMapper().readValue(resource.getFile(), ConfigProperties.class);

        CryptoSessionInit<CryptoSessionType> mapInit = CryptoSessionInit.ofLocalMap(
                CryptoSessionType.HOTEL,
                ImmutableMap.of(
                        "key", configProperties.getKey(),
                        "iv", configProperties.getIv(),
                        "seed", configProperties.getSeed(),
                        "credential", configProperties.getCredential()
                )
        );
        assertThat(mapInit.getInitType()).isEqualTo(CryptoSessionInit.InitType.LOCAL_MAP);
        assertAll(
                () -> assertThat(mapInit.getConfigLocalMap().get("key")).isEqualTo(configProperties.getKey()),
                () -> assertThat(mapInit.getConfigLocalMap().get("iv")).isEqualTo(configProperties.getIv()),
                () -> assertThat(mapInit.getConfigLocalMap().get("seed")).isEqualTo(configProperties.getSeed()),
                () -> assertThat(mapInit.getConfigLocalMap().get("credential")).isEqualTo(configProperties.getCredential())
        );
    }

    @Test
    @Disabled
    @Order(8)
    @DisplayName("PARAMS 타입 CryptoSessionInit 객체 생성 테스트")
    void paramsTest() throws IOException {
        Resource resource = new ClassPathResource(Paths.get("crypto", "hotel", "config.json").toString());
        ConfigProperties configProperties = JsonUtils.getObjectMapper().readValue(resource.getFile(), ConfigProperties.class);
        // HOTEL CryptoSession 생성
        CryptoSessionInit<CryptoSessionType> paramsInit = CryptoSessionInit.ofParams(
                CryptoSessionType.HOTEL,
                configProperties.getAwsKmsKeyArn(),
                configProperties.getAwsAccessKeyId(),
                configProperties.getAwsSecretAccessKey(),
                configProperties.getSeed(),
                configProperties.getCredential()
        );
        assertThat(paramsInit.getInitType()).isEqualTo(CryptoSessionInit.InitType.PARAMS);
        assertAll(
                () -> assertThat(paramsInit.getAwsKmsKeyArn()).isEqualTo(configProperties.getAwsKmsKeyArn()),
                () -> assertThat(paramsInit.getAwsAccessKeyId()).isEqualTo(configProperties.getAwsAccessKeyId()),
                () -> assertThat(paramsInit.getAwsSecretAccessKey()).isEqualTo(configProperties.getAwsSecretAccessKey()),
                () -> assertThat(paramsInit.getSeed()).isEqualTo(configProperties.getSeed()),
                () -> assertThat(paramsInit.getCredential()).isEqualTo(configProperties.getCredential())
        );
    }

    @Test
    @Order(9)
    @DisplayName("LOCAL_PARAMS 타입 CryptoSessionInit 객체 생성 테스트")
    void localParamsTest() throws IOException {
        Resource resource = new ClassPathResource(Paths.get("crypto", "hotel", "config.json").toString());
        ConfigProperties configProperties = JsonUtils.getObjectMapper().readValue(resource.getFile(), ConfigProperties.class);
        // HOTEL CryptoSession 생성
        CryptoSessionInit<CryptoSessionType> paramsInit = CryptoSessionInit.ofLocalParams(
                CryptoSessionType.HOTEL,
                configProperties.getKey(),
                configProperties.getIv(),
                configProperties.getSeed(),
                configProperties.getCredential()
        );
        assertThat(paramsInit.getInitType()).isEqualTo(CryptoSessionInit.InitType.LOCAL_PARAMS);
        assertAll(
                () -> assertThat(paramsInit.getKey()).isEqualTo(configProperties.getKey()),
                () -> assertThat(paramsInit.getIv()).isEqualTo(configProperties.getIv()),
                () -> assertThat(paramsInit.getSeed()).isEqualTo(configProperties.getSeed()),
                () -> assertThat(paramsInit.getCredential()).isEqualTo(configProperties.getCredential())
        );
    }
}
