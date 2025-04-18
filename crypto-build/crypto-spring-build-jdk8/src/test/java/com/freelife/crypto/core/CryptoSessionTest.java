package com.freelife.crypto.core;

import com.fasterxml.jackson.core.type.TypeReference;
import com.freelife.util.EncryptUtils;
import com.freelife.util.JsonUtils;
import org.junit.jupiter.api.*;

import java.io.ByteArrayInputStream;
import java.io.IOException;
import java.io.InputStream;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.Map;

import static com.freelife.util.ByteUtils.toByteArray;
import static org.assertj.core.api.Assertions.assertThat;
import static org.junit.jupiter.api.Assertions.*;
import static org.junit.jupiter.api.Assertions.assertTrue;

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
        executeSessionTest(session);
    }

    @Test
    @Order(2)
    void cryptoSessionByPathTest() {
        Path path = Paths.get("crypto", "config.json").toAbsolutePath();
        CryptoSession session = new CryptoSession(path.toString());
        executeSessionTest(session);
    }

    @Test
    @Order(3)
    void cryptoSessionByInputStreamTest() throws IOException {
        Path path = Paths.get("crypto", "config.json").toAbsolutePath();
        assertThat(Files.exists(path)).isTrue();
        byte[] bytes = toByteArray(Files.newInputStream(path));
        InputStream inputStream = new ByteArrayInputStream(bytes);
        CryptoSession session = new CryptoSession(inputStream);
        executeSessionTest(session);
    }

    @Test
    @Order(4)
    void cryptoSessionByByteTest() throws IOException {
        Path path = Paths.get("crypto", "config.json").toAbsolutePath();
        assertThat(Files.exists(path)).isTrue();
        byte[] bytes = toByteArray(Files.newInputStream(path));
        CryptoSession session = new CryptoSession(bytes);
        executeSessionTest(session);
    }

    @Test
    @Disabled
    @Order(5)
    void cryptoSessionByMapTest() throws Exception {
        Path path = Paths.get("crypto", "config.json").toAbsolutePath();

        byte[] bytes = toByteArray(Files.newInputStream(path));
        InputStream inputStream = new ByteArrayInputStream(bytes);
        Map<String, String> configMap = JsonUtils.getObjectMapper().readValue(inputStream, new TypeReference<Map<String, String>>() {});

        CryptoSession session = new CryptoSession(configMap);
        executeSessionTest(session);
    }

    @Test
    @Order(6)
    void cryptoSessionByLocalMapTest() throws Exception {
        Path path = Paths.get("crypto", "config.json").toAbsolutePath();

        byte[] bytes = toByteArray(Files.newInputStream(path));
        InputStream inputStream = new ByteArrayInputStream(bytes);
        Map<String, String> configLocalMap = JsonUtils.getObjectMapper().readValue(inputStream, new TypeReference<Map<String, String>>() {});

        CryptoSession session = new CryptoSession(configLocalMap, configLocalMap.get("key"), configLocalMap.get("iv"));
        executeSessionTest(session);
    }

    @Test
    @Disabled
    @Order(7)
    void cryptoSessionByArumentsTest() throws Exception {
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
        executeSessionTest(session);
    }

    @Test
    @Order(8)
    void cryptoSessionByLocalArumentsTest() throws Exception {
        Path path = Paths.get("crypto", "config.json").toAbsolutePath();

        byte[] bytes = toByteArray(Files.newInputStream(path));
        InputStream inputStream = new ByteArrayInputStream(bytes);
        Map<String, String> configMap = JsonUtils.getObjectMapper().readValue(inputStream, new TypeReference<Map<String, String>>() {});

        String key = configMap.get("key");
        String iv = configMap.get("iv");
        String seed = configMap.get("seed");
        String credential = configMap.get("credential");

        CryptoSession session = new CryptoSession(key, iv, seed, credential);
        executeSessionTest(session);
    }

    private void executeSessionTest(CryptoSession session) {
        String plaintext = "Hello Crypto!";
        String encrypt = session.encrypt(plaintext);
        String decrypt = session.decrypt(encrypt);
        String hash = session.hash(plaintext);
        String hashAlg = session.hash(plaintext, "SHA512");
        String hashAlgKey = session.hash(plaintext, "SHA512_256", EncryptUtils.generateKey(16).getBytes(StandardCharsets.UTF_8));
        // assertThat(decrypt).isEqualTo(plaintext);
        assertAll(
                () -> assertEquals(decrypt, plaintext),
                () -> assertTrue(hash.length() >= 44),
                () -> assertTrue(hashAlg.length() >= 88),
                () -> assertTrue(hashAlgKey.length() >= 44)
        );
    }
}