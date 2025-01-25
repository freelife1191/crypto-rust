package com.freelife.crypto.config;

import com.fasterxml.jackson.databind.PropertyNamingStrategies;
import com.fasterxml.jackson.databind.annotation.JsonNaming;
import com.freelife.crypto.core.CryptoException;
import com.freelife.util.JsonUtils;
import lombok.Data;
import org.junit.platform.commons.util.StringUtils;
import org.springframework.core.io.ClassPathResource;
import org.springframework.core.io.Resource;

import java.io.IOException;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.List;
import java.util.Map;

@Data
@JsonNaming(PropertyNamingStrategies.SnakeCaseStrategy.class)
public class ConfigProperties {
    private String awsKmsKeyArn;
    private String awsAccessKeyId;
    private String awsSecretAccessKey;
    private String key;
    private String iv;
    private String seed;
    private String credential;

    public static Resource getResource(String path) {
        if (StringUtils.isBlank(path)) path = "default";
        return new ClassPathResource(Path.of("crypto", path, "config.json").toString());
    }

    public static Resource getResource() {
        return getResource("default");
    }

    public static <T extends Enum<T>> Resource getResource(T type) {
        return getClassPathResource(type.name().toLowerCase());
    }

    public static ClassPathResource getClassPathResource(String path) {
        if (StringUtils.isBlank(path)) path = "default";
        return new ClassPathResource(Path.of("crypto", path, "config.json").toString());
    }

    public static ClassPathResource getClassPathResource() {
        return getClassPathResource("default");
    }

    public static <T extends Enum<T>> ClassPathResource getClassPathResource(T type) {
        return getClassPathResource(type.name().toLowerCase());
    }

    public static Path getPath(String path) {
        try {
            return getResource().getFile().toPath();
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
    }

    public static Map<String, String> toMap(String path) {
        if (StringUtils.isBlank(path)) path = "default";
        ConfigProperties configProperties = to(path);
        return toMap(configProperties.getAwsKmsKeyArn(), configProperties.getAwsAccessKeyId(), configProperties.getAwsSecretAccessKey(), configProperties.getSeed(), configProperties.getCredential());
    }

    public static Map<String, String> toMap() {
        ConfigProperties configProperties = to("default");
        return toMap(configProperties.getAwsKmsKeyArn(), configProperties.getAwsAccessKeyId(), configProperties.getAwsSecretAccessKey(), configProperties.getSeed(), configProperties.getCredential());
    }

    public static Map<String, String> toLocalMap(String path) {
        if (StringUtils.isBlank(path)) path = "default";
        ConfigProperties configProperties = to(path);
        return toLocalMap(configProperties.getKey(), configProperties.getIv(), configProperties.getSeed(), configProperties.getCredential());
    }

    public static Map<String, String> toLocalMap() {
        ConfigProperties configProperties = to("default");
        return toLocalMap(configProperties.getKey(), configProperties.getIv(), configProperties.getSeed(), configProperties.getCredential());
    }

    private static Map<String, String> toLocalMap(String key, String iv, String seed, String credential) {
        List<String> errors = new ArrayList<>();
        if (key == null || key.isBlank()) errors.add("key");
        if (iv == null || iv.isBlank()) errors.add("iv");
        if (seed == null || seed.isBlank()) errors.add("seed");
        if (credential == null || credential.isBlank()) errors.add("credential");
        if (!errors.isEmpty())
            throw new CryptoException("The following parameters are required: " + errors);
        return Map.of(
                "key", key,
                "iv", iv,
                "seed", seed,
                "credential", credential);
    }

    public static Map<String, String> toMap(String awsKmsKeyArn, String awsAccessKeyId, String awsSecretAccessKey, String seed, String credential) {
        List<String> errors = new ArrayList<>();
        if (awsKmsKeyArn == null || awsKmsKeyArn.isBlank()) errors.add("aws_kms_key_arn");
        if (awsAccessKeyId == null || awsAccessKeyId.isBlank()) errors.add("aws_access_key_id");
        if (awsSecretAccessKey == null || awsSecretAccessKey.isBlank()) errors.add("aws_secret_access_key");
        if (seed == null || seed.isBlank()) errors.add("seed");
        if (credential == null || credential.isBlank()) errors.add("credential");
        if (!errors.isEmpty())
            throw new IllegalArgumentException("The following parameters are required: " + errors);
        return Map.of(
                "aws_kms_key_arn", awsKmsKeyArn,
                "aws_access_key_id", awsAccessKeyId,
                "aws_secret_access_key", awsSecretAccessKey,
                "seed", seed,
                "credential", credential);
    }


    public static ConfigProperties to() {
        return to("default");
    }

    public static <T extends Enum<T>> ConfigProperties to(T type) {
        return to(type.name().toLowerCase());
    }

    public static ConfigProperties to(String path) {
        try {
            return JsonUtils.toMapperObject(getResource(path).getFile(), ConfigProperties.class);
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
    }
}
