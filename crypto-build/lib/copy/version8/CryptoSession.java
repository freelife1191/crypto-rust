// Automatically generated by flapigen
package com.freelife.crypto.core;

import java.io.ByteArrayOutputStream;
import java.io.File;
import java.io.IOException;
import java.io.InputStream;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.time.OffsetDateTime;
import java.time.format.DateTimeFormatter;
import java.util.*;
import java.util.stream.Stream;

public final class CryptoSession {

    /*package*/ private volatile long mNativeObj;

    /**
     * 기본 경로에 있는 config.json 파일을 읽어서 CryptoSession 객체를 생성합니다.
     * 기본 경로1: ${projectDir}/crypto/config.json
     * 기본 경로2: /var/opt/crypto/config.json
     */
    public CryptoSession() {
        List<Path> cryptoBasePaths = new ArrayList<>();
        cryptoBasePaths.add(Paths.get("crypto","config.json").toAbsolutePath());
        cryptoBasePaths.add(Paths.get("crypto","config.json"));
        cryptoBasePaths.add(Paths.get(File.separator, "opt", "crypto", "config.json"));
        cryptoBasePaths.add(Paths.get(File.separator, "var", "crypto", "config.json"));
        writeLog("Default Crypto Config Paths: " + cryptoBasePaths);
        Path cryptoApplyPath = null;
        for (Path path : cryptoBasePaths) {
            if (Files.exists(path)) {
                cryptoApplyPath = path;
                break;
            }
        }
        if (cryptoApplyPath == null)
            throw new CryptoException("Could not find config.json file in default path");
        writeLog("Apply Crypto Config Path: " + cryptoApplyPath);
        try {
            mNativeObj = init(cryptoApplyPath.toString());
        } catch (Exception e) {
            throw new CryptoException(e.getMessage());
        }
    }
    /**
     * path 경로에 있는 config.json 파일을 읽어서 CryptoSession 객체를 생성합니다.
     * @param path config.json 파일의 경로
     */
    public CryptoSession(String path) {
        if (path == null)
            throw new CryptoException("path is required");
        if (!Files.exists(Paths.get(path)))
            throw new CryptoException("Could not find 'config.json' file at that path");
        try {
            mNativeObj = init(path);
        } catch (Exception e) {
            throw new CryptoException(e.getMessage());
        }
    }
    /**
     * config.json 파일을 InputStream 형태로 읽어서 CryptoSession 객체를 생성합니다.
     * @param inputStream config.json 파일을 읽어들인 InputStream
     */
    public CryptoSession(InputStream inputStream) {
        if (inputStream == null)
            throw new CryptoException("inputStream is required");
        byte [] bytes;
        try {
           bytes = toByteArray(inputStream);
        } catch (java.io.IOException e) {
            throw new CryptoException(e.getMessage());
        }
        try {
            mNativeObj = init(bytes);
        } catch (Exception e) {
            throw new CryptoException(e.getMessage());
        }
    }

    public static byte[] toByteArray(InputStream inputStream) throws IOException {
        ByteArrayOutputStream buffer = new ByteArrayOutputStream();
        byte[] data = new byte[8192];
        int bytesRead;
        while ((bytesRead = inputStream.read(data, 0, data.length)) != -1) {
            buffer.write(data, 0, bytesRead);
        }
        return buffer.toByteArray();
    }

    /**
     * config.json 파일을 읽어서 CryptoSession 객체를 생성합니다.
     * @param bytes config.json 파일을 읽어들인 byte 배열
     */
    public CryptoSession(byte [] bytes) {
        if (bytes == null)
            throw new CryptoException("bytes is required");
        try {
            mNativeObj = init(bytes);
        } catch (Exception e) {
            throw new CryptoException(e.getMessage());
        }
    }

    /**
     * Map 형태로 전달된 구성을 사용하여 CryptoSession 객체를 생성합니다.
     * 구성 맵에는 다음 키가 포함되어야 합니다.
     * @param configLocalMap configLocalMap(key, iv, seed, credential)
     * @param key key
     * @param iv iv
     */
    public CryptoSession(Map<String, String> configLocalMap, String key, String iv) {
        if (configLocalMap == null)
            throw new CryptoException("configLocalMap is required: Elements in configLocalMap must contain values for the keys (key, iv, seed, credential)");
        List<String> errors = new ArrayList<>();
        String _key = configLocalMap.get("key");
        String _iv = configLocalMap.get("iv");
        String seed = configLocalMap.get("seed");
        String credential = configLocalMap.get("credential");
        configLocalMap.forEach((k, v) -> {
            if (v == null || v.isEmpty()) {
                errors.add(k);
            }
        });
        boolean allMatch = Stream.of(_key, _iv, seed, credential)
                .allMatch(s -> s != null && !s.isEmpty());
        if (!allMatch)
            throw new CryptoException("The config map does not contain the required keys: " + errors);
        try {
            mNativeObj = init(_key, _iv, seed, credential);
        } catch (Exception e) {
            throw new CryptoException(e.getMessage());
        }
    }

    /**
     * Map 형태로 전달된 구성을 사용하여 CryptoSession 객체를 생성합니다.
     * 구성 맵에는 다음 키가 포함되어야 합니다.
     * @param configMap configMap(aws_kms_key_arn, aws_access_key_id, aws_secret_access_key, seed, credential)
     */
    public CryptoSession(Map<String, String> configMap) {
        if (configMap == null)
            throw new CryptoException("configMap is required: Elements in configMap must contain values for the keys (aws_kms_key_arn, aws_access_key_id, aws_secret_access_key, seed, credential)");
        List<String> errors = new ArrayList<>();
        String awsKmsKeyArn = configMap.get("aws_kms_key_arn");
        String awsAccessKeyId = configMap.get("aws_access_key_id");
        String awsSecretAccessKey = configMap.get("aws_secret_access_key");
        String seed = configMap.get("seed");
        String credential = configMap.get("credential");
        configMap.forEach((k, v) -> {
            if (v == null || v.isEmpty()) {
                errors.add(k);
            }
        });
        boolean allMatch = Stream.of(awsKmsKeyArn, awsAccessKeyId, awsSecretAccessKey, seed, credential)
                .allMatch(s -> s != null && !s.isEmpty());
        if (!allMatch)
            throw new CryptoException("The config map does not contain the required keys: " + errors);
        try {
            mNativeObj = init(awsKmsKeyArn, awsAccessKeyId, awsSecretAccessKey, seed, credential);
        } catch (Exception e) {
            throw new CryptoException(e.getMessage());
        }
    }

    /**
     * 파라메터를 전달하여 CryptoSession 객체를 생성합니다.
     * 파라메터는 다음 값들이 필수적으로 추가되어야 합니다.
     * @param key key
     * @param iv iv
     * @param seed seed
     * @param credential credential
     */
    public CryptoSession(String key, String iv, String seed, String credential) {
        List<String> errors = new ArrayList<>();
        if (key == null || key.isEmpty()) errors.add("key");
        if (iv == null || iv.isEmpty()) errors.add("iv");
        if (seed == null || seed.isEmpty()) errors.add("seed");
        if (credential == null || credential.isEmpty()) errors.add("credential");
        if (!errors.isEmpty())
            throw new CryptoException("The following parameters are required: " + errors);
        try {
            mNativeObj = init(key, iv, seed, credential);
        } catch (Exception e) {
            throw new CryptoException(e.getMessage());
        }
    }

    /**
     * 파라메터를 전달하여 CryptoSession 객체를 생성합니다.
     * 파라메터는 다음 값들이 필수적으로 추가되어야 합니다.
     * @param aws_kms_key_arn aws_kms_key_arn
     * @param aws_access_key_id aws_access_key_id
     * @param aws_secret_access_key aws_secret_access_key
     * @param seed seed
     * @param credential credential
     */
    public CryptoSession(String aws_kms_key_arn, String aws_access_key_id, String aws_secret_access_key, String seed, String credential) {
        List<String> errors = new ArrayList<>();
        if (aws_kms_key_arn == null || aws_kms_key_arn.isEmpty()) errors.add("aws_kms_key_arn");
        if (aws_access_key_id == null || aws_access_key_id.isEmpty()) errors.add("aws_access_key_id");
        if (aws_secret_access_key == null || aws_secret_access_key.isEmpty()) errors.add("aws_secret_access_key");
        if (seed == null || seed.isEmpty()) errors.add("seed");
        if (credential == null || credential.isEmpty()) errors.add("credential");
        if (!errors.isEmpty())
            throw new CryptoException("The following parameters are required: " + errors);
        try {
            mNativeObj = init(aws_kms_key_arn, aws_access_key_id, aws_secret_access_key, seed, credential);
        } catch (Exception e) {
            throw new CryptoException(e.getMessage());
        }
    }

    public final String encrypt(String plaintext) {
        try {
            return do_encrypt(mNativeObj, plaintext);
        } catch (Exception e) {
            throw new CryptoException(e.getMessage());
        }
    }
    public final String decrypt(String encrypted) {
        try {
            return do_decrypt(mNativeObj, encrypted);
        } catch (Exception e) {
            throw new CryptoException(e.getMessage());
        }
    }

    public final String encrypt_id(String plaintext, int id) {
        if (id != 100 && id != 400)
            throw new CryptoException("Invalid ID: " + id + ", Available IDs: 100, 400");
        try {
            return do_encrypt_id(mNativeObj, plaintext, id);
        } catch (Exception e) {
            throw new CryptoException(e.getMessage());
        }
    }
    public final String decrypt_id(String encrypted, int id) {
        if (id != 100 && id != 400)
            throw new CryptoException("Invalid ID: " + id + ", Available IDs: 100, 400");
        try {
            return do_decrypt_id(mNativeObj, encrypted, id);
        } catch (Exception e) {
            throw new CryptoException(e.getMessage());
        }
    }

    public synchronized void delete() {
        if (mNativeObj != 0) {
            do_delete(mNativeObj);
            mNativeObj = 0;
        }
    }

    @Override
    protected void finalize() throws Throwable {
        try {
            delete();
        }
        finally {
            super.finalize();
        }
    }

    private static native long init(String path) throws Exception;
    private static native long init(byte[] bytes) throws Exception;
    private static native long init(String key, String iv, String seed, String credential) throws Exception;
    private static native long init(String aws_kms_key, String access_key_id, String secret_access_key, String seed, String credential) throws Exception;
    private static native String do_encrypt(long self, String plaintext) throws Exception;
    private static native String do_decrypt(long self, String encrypted) throws Exception;
    private static native String do_encrypt_id(long self, String plaintext, int id) throws Exception;
    private static native String do_decrypt_id(long self, String encrypted, int id) throws Exception;
    private static native void do_delete(long me);
    /*package*/ CryptoSession(InternalPointerMarker marker, long ptr) {
        assert marker == InternalPointerMarker.RAW_PTR;
        this.mNativeObj = ptr;
    }

    private static void writeLog(String message) {
        // return new SimpleDateFormat("yyyy-MM-dd HH:mm:ss:SSS").format(new Date());
        String prefix = "[Crypto][INFO]";
        String now = OffsetDateTime.now().format(DateTimeFormatter.ISO_OFFSET_DATE_TIME);
        System.out.format("%s[%s] %s\n",prefix, now, message);
    }

    static {
        try {
            NativeUtils.loadLibraryFromJar();
        } catch (java.io.IOException e) {
            throw new CryptoException(e.getMessage());
        }
    }
}