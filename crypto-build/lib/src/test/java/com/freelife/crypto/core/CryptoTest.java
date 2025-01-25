package com.freelife.crypto.core;

// import com.fasterxml.jackson.core.type.TypeReference;
// import com.freelife.crypto.lib.JsonUtils;
import org.assertj.core.api.Assertions;
import org.junit.jupiter.api.Test;
import com.freelife.crypto.core.CryptoSession;

import java.io.File;
import java.io.IOException;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.Map;

public class CryptoTest {
    @Test public void cryptoSessionTest() {

        /*
        Path path = Path.of("crypto", "config.json").toAbsolutePath();
        System.out.println("Path="+path);

        File configFile = new File(path.toString());
        Map<String, String> jsonMap;
        String json;
        try {
            jsonMap = JsonUtils.getObjectMapper().readValue(configFile, new TypeReference<>() {});
            json = JsonUtils.getObjectMapper().writeValueAsString(jsonMap);
        } catch (IOException e) {
            throw new RuntimeException(e.getMessage());
        }
        String awsKmsKey = jsonMap.get("aws_kms_key");
        String accessKeyId = jsonMap.get("access_key_id");
        String secretAccessKey = jsonMap.get("secret_access_key");
        String seed = jsonMap.get("seed");
        String credential = jsonMap.get("credential");
        */

        //CryptoSession session = new CryptoSession(awsKmsKey, accessKeyId, secretAccessKey, seed, credential);
        CryptoSession session = new CryptoSession("crypto/config.json");

        String plaintext = "Hello World!";

        String encrypt = session.encrypt(plaintext);
        System.out.println("encrypt="+encrypt);
        String decrypt = session.decrypt(encrypt);
        System.out.println("decrypt="+decrypt);
        Assertions.assertThat(decrypt).isEqualTo(plaintext);
    }
}
