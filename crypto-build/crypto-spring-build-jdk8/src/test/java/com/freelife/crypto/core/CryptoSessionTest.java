package com.freelife.crypto.core;

import com.fasterxml.jackson.core.type.TypeReference;
import com.freelife.util.JsonUtils;
import org.junit.jupiter.api.*;

import java.io.ByteArrayInputStream;
import java.io.IOException;
import java.io.InputStream;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.Map;

import static com.freelife.util.ByteUtils.toByteArray;
import static org.assertj.core.api.Assertions.assertThat;

/**
 * CryptoSession 기본 테스트
 * Created by mskwon on 2024. 10. 23..
 */
// @Disabled
@TestMethodOrder(MethodOrderer.OrderAnnotation.class)
class CryptoSessionTest {

    @Test
    @Order(1)
    void cryptoSessionDefaultTest() {
        CryptoSession session = new CryptoSession();
        String plaintext = "Hello Crypto!";
        String encrypt = session.encrypt(plaintext);
        String decrypt = session.decrypt(encrypt);
        assertThat(decrypt).isEqualTo(plaintext);
    }

    @Test
    @Order(2)
    void cryptoSessionByPathTest() {
        Path path = Paths.get("crypto", "config.json").toAbsolutePath();
        CryptoSession session = new CryptoSession(path.toString());
        String plaintext = "Hello Crypto!";
        String encrypt = session.encrypt(plaintext);
        String decrypt = session.decrypt(encrypt);
        assertThat(decrypt).isEqualTo(plaintext);
    }

    @Test
    @Order(3)
    void cryptoSessionByInputStreamTest() throws IOException {
        Path path = Paths.get("crypto", "config.json").toAbsolutePath();
        String plaintext = "Hello Crypto!";
        assertThat(Files.exists(path)).isTrue();
        byte[] bytes = toByteArray(Files.newInputStream(path));
        InputStream inputStream = new ByteArrayInputStream(bytes);
        CryptoSession session = new CryptoSession(inputStream);
        String encrypt = session.encrypt(plaintext);
        String decrypt = session.decrypt(encrypt);
        assertThat(decrypt).isEqualTo(plaintext);
    }

    @Test
    @Order(4)
    void cryptoSessionByByteTest() throws IOException {
        Path path = Paths.get("crypto", "config.json").toAbsolutePath();
        String plaintext = "Hello Crypto!";
        assertThat(Files.exists(path)).isTrue();
        byte[] bytes = toByteArray(Files.newInputStream(path));
        CryptoSession session = new CryptoSession(bytes);
        String encrypt = session.encrypt(plaintext);
        String decrypt = session.decrypt(encrypt);
        assertThat(decrypt).isEqualTo(plaintext);
    }

    @Test
    @Disabled
    @Order(5)
    void cryptoSessionByMapTest() throws Exception {
        String plaintext = "Hello Crypto!";
        Path path = Paths.get("crypto", "config.json").toAbsolutePath();

        byte[] bytes = toByteArray(Files.newInputStream(path));
        InputStream inputStream = new ByteArrayInputStream(bytes);
        Map<String, String> configMap = JsonUtils.getObjectMapper().readValue(inputStream, new TypeReference<Map<String, String>>() {});

        CryptoSession session = new CryptoSession(configMap);
        String encrypt = session.encrypt(plaintext);
        String decrypt = session.decrypt(encrypt);
        assertThat(decrypt).isEqualTo(plaintext);
    }

    @Test
    @Order(6)
    void cryptoSessionByLocalMapTest() throws Exception {
        String plaintext = "Hello Crypto!";
        Path path = Paths.get("crypto", "config.json").toAbsolutePath();

        byte[] bytes = toByteArray(Files.newInputStream(path));
        InputStream inputStream = new ByteArrayInputStream(bytes);
        Map<String, String> configLocalMap = JsonUtils.getObjectMapper().readValue(inputStream, new TypeReference<Map<String, String>>() {});

        CryptoSession session = new CryptoSession(configLocalMap, configLocalMap.get("key"), configLocalMap.get("iv"));
        String encrypt = session.encrypt(plaintext);
        String decrypt = session.decrypt(encrypt);
        assertThat(decrypt).isEqualTo(plaintext);
    }

    @Test
    @Disabled
    @Order(7)
    void cryptoSessionByArumentsTest() throws Exception {
        String plaintext = "Hello Crypto!";
        Path path = Paths.get("crypto", "config.json").toAbsolutePath();

        byte[] bytes = toByteArray(Files.newInputStream(path));
        InputStream inputStream = new ByteArrayInputStream(bytes);
        Map<String, String> configMap = JsonUtils.getObjectMapper().readValue(inputStream, new TypeReference<Map<String, String>>() {});

        String awsKmsKeyArn = configMap.get("aws_kms_key_arn");
        String awsAccessKeyId = configMap.get("aws_access_key_id");
        String awsSecretAccessKey = configMap.get("aws_secret_access_key");
        String seed = configMap.get("seed");
        String credential = configMap.get("credential");

        CryptoSession session = new CryptoSession(awsKmsKeyArn, awsAccessKeyId, awsSecretAccessKey, seed, credential);
        String encrypt = session.encrypt(plaintext);
        String decrypt = session.decrypt(encrypt);
        assertThat(decrypt).isEqualTo(plaintext);
    }

    @Test
    @Order(8)
    void cryptoSessionByLocalArumentsTest() throws Exception {
        String plaintext = "Hello Crypto!";
        Path path = Paths.get("crypto", "config.json").toAbsolutePath();

        byte[] bytes = toByteArray(Files.newInputStream(path));
        InputStream inputStream = new ByteArrayInputStream(bytes);
        Map<String, String> configMap = JsonUtils.getObjectMapper().readValue(inputStream, new TypeReference<Map<String, String>>() {});

        String key = configMap.get("key");
        String iv = configMap.get("iv");
        String seed = configMap.get("seed");
        String credential = configMap.get("credential");

        CryptoSession session = new CryptoSession(key, iv, seed, credential);
        String encrypt = session.encrypt(plaintext);
        String decrypt = session.decrypt(encrypt);
        assertThat(decrypt).isEqualTo(plaintext);
    }
}