package com.freelife.crypto.factory;

import com.freelife.crypto.core.CryptoException;
import com.freelife.crypto.core.CryptoSession;

import java.io.File;
import java.io.InputStream;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.time.OffsetDateTime;
import java.time.format.DateTimeFormatter;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.Map;
import java.util.stream.Stream;

/**
 * CryptoSessionType 별로 CryptoSession Factory Bean 을 초기화하기 위한 클래스
 * Created by mskwon on 2024. 10. 17..
 */
public class CryptoSessionInit<T extends Enum<T>> {

    /**
     * 각 서비스에서 Custom 하게 정의한 CryptoSessionType
     */
    private T cryptoSessionType;
    /**
     * CryptoSession 초기화 타입
     *
     * BASE_PATH: 기본 경로 Path 타입
     * BYTES: byte 배열 타입
     * INPUT_STREAM: InputStream 타입
     * PATH: String Path 타입
     * MAP: Map 타입
     * PARAMS: 파라메터 타입 포함되어야 하는 파라메터 요소(aws_kms_key_arn, aws_access_key_id, aws_secret_access_key, seed, credential)
     */
    private InitType initType;

    /**
     * config.json 파일을 읽어들인 byte 배열
     */
    private byte [] configBytes;
    /**
     * config.json 파일을 읽어들인 InputStream
     */
    private InputStream configInputStream;
    /**
     * config.json 파일 String Path
     */
    private String path;
    /**
     * config 파라메터를 담은 Map
     * 구성 맵에는 다음 키가 포함되어야 함
     * aws_kms_key_arn, aws_access_key_id, aws_secret_access_key, seed, credential
     */
    private Map<String, String> configMap;
    /**
     * config 파라메터를 담은 Map
     * 구성 맵에는 다음 키가 포함되어야 함
     * key, iv, seed, credential
     */
    private Map<String, String> configLocalMap;
    /**
     * aws_kms_key_arn
     */
    private String awsKmsKeyArn;
    /**
     * aws_access_key_id
     */
    private String awsAccessKeyId;
    /**
     * aws_secret_access_key
     */
    private String awsSecretAccessKey;
    /**
     * key
     */
    private String key;
    /**
     * iv
     */
    private String iv;
    /**
     * seed
     */
    private String seed;
    /**
     * credential
     */
    private String credential;

    private CryptoSessionInit(T cryptoSessionType) {
        this.cryptoSessionType = cryptoSessionType;
        this.initType = InitType.BASE_PATH;
    }

    private CryptoSessionInit(T cryptoSessionType, Object config) {
        this.cryptoSessionType = cryptoSessionType;
        if (config instanceof byte[]) {
            this.initType = InitType.BYTES;
            this.configBytes = (byte[]) config;
        } else if (config instanceof InputStream) {
            this.initType = InitType.INPUT_STREAM;
            this.configInputStream = (InputStream) config;
        } else if (config instanceof String) {
            this.initType = InitType.PATH;
            this.path = (String) config;
        } else if (config instanceof Map<?, ?>) {
            this.initType = InitType.MAP;
            this.configMap = (Map<String, String>) config;
        } else {
            throw new CryptoException("config type is not supported");
        }
    }

    private CryptoSessionInit(T cryptoSessionType, Object config, String key, String iv) {
        this.initType = InitType.LOCAL_MAP;
        this.cryptoSessionType = cryptoSessionType;
        this.configLocalMap = (Map<String, String>) config;
        this.key = key;
        this.iv = iv;
    }

    private CryptoSessionInit(T cryptoSessionType, String awsKmsKeyArn, String awsAccessKeyId, String awsSecretAccessKey, String seed, String credential) {
        this.initType = InitType.PARAMS;
        this.cryptoSessionType = cryptoSessionType;
        this.awsKmsKeyArn = awsKmsKeyArn;
        this.awsAccessKeyId = awsAccessKeyId;
        this.awsSecretAccessKey = awsSecretAccessKey;
        this.seed = seed;
        this.credential = credential;
    }

    private CryptoSessionInit(T cryptoSessionType, String key, String iv, String seed, String credential) {
        this.initType = InitType.LOCAL_PARAMS;
        this.cryptoSessionType = cryptoSessionType;
        this.key = key;
        this.iv = iv;
        this.seed = seed;
        this.credential = credential;
    }

    public T getCryptoSessionType() {
        return cryptoSessionType;
    }

    private void setCryptoSessionType(T cryptoSessionType) {
        this.cryptoSessionType = cryptoSessionType;
    }

    public InitType getInitType() {
        return initType;
    }

    private void setInitType(InitType initType) {
        this.initType = initType;
    }

    public byte[] getConfigBytes() {
        return configBytes;
    }

    private void setConfigBytes(byte[] configBytes) {
        this.configBytes = configBytes;
    }

    public InputStream getConfigInputStream() {
        return configInputStream;
    }

    private void setConfigInputStream(InputStream configInputStream) {
        this.configInputStream = configInputStream;
    }

    public String getPath() {
        return path;
    }

    private void setPath(String path) {
        this.path = path;
    }

    public Map<String, String> getConfigMap() {
        return configMap;
    }

    private void setConfigMap(Map<String, String> configMap) {
        this.configMap = configMap;
    }

    public Map<String, String> getConfigLocalMap() {
        return configLocalMap;
    }

    private void setConfigLocalMap(Map<String, String> configLocalMap) {
        this.configLocalMap = configLocalMap;
    }

    public String getAwsKmsKeyArn() {
        return awsKmsKeyArn;
    }

    private void setAwsKmsKeyArn(String awsKmsKeyArn) {
        this.awsKmsKeyArn = awsKmsKeyArn;
    }

    public String getAwsAccessKeyId() {
        return awsAccessKeyId;
    }

    private void setAwsAccessKeyId(String awsAccessKeyId) {
        this.awsAccessKeyId = awsAccessKeyId;
    }

    public String getAwsSecretAccessKey() {
        return awsSecretAccessKey;
    }

    private void setAwsSecretAccessKey(String awsSecretAccessKey) {
        this.awsSecretAccessKey = awsSecretAccessKey;
    }

    public String getKey() {
        return key;
    }

    private void setKey(String key) {
        this.key = key;
    }

    public String getIv() {
        return iv;
    }

    private void setIv(String iv) {
        this.iv = iv;
    }

    public String getSeed() {
        return seed;
    }

    private void setSeed(String seed) {
        this.seed = seed;
    }

    public String getCredential() {
        return credential;
    }

    private void setCredential(String credential) {
        this.credential = credential;
    }

    /**
     * 기본 경로에 있는 config.json 파일을 읽어서 CryptoSession 객체를 생성
     * 기본 경로1: ${projectDir}/crypto/config.json
     * 기본 경로2: /var/opt/crypto/config.json
     * @param <T> CryptoSessionType
     * @return CryptoSessionInit
     */
    public static <T extends Enum<T>> CryptoSessionInit<T> ofBasePath(T cryptoSessionType) {
        List<Path> cryptoBasePaths = new ArrayList<>();
        cryptoBasePaths.add(Paths.get("crypto","config.json").toAbsolutePath());
        cryptoBasePaths.add(Paths.get("crypto","config.json"));
        cryptoBasePaths.add(Paths.get(File.separator,"var", "opt", "crypto","config.json"));
        Path cryptoApplyPath = null;
        for (Path path : cryptoBasePaths) {
            if (Files.exists(path)) {
                cryptoApplyPath = path;
                break;
            }
        }
        if (cryptoApplyPath == null)
            throw new CryptoException("Could not find config.json file in default path");
        return new CryptoSessionInit<>(cryptoSessionType);
    }

    /**
     * config.json 파일을 읽어서 CryptoSession 객체를 생성
     * config.json 파일에는 다음 키가 포함되어야 함
     * aws_kms_key_arn, aws_access_key_id, aws_secret_access_key, seed, credential
     * @param cryptoSessionType CryptoSessionType
     * @param configBytes config.json 파일을 읽어들인 byte 배열
     * @return CryptoSessionInit
     */
    public static <T extends Enum<T>> CryptoSessionInit<T> ofBytes(T cryptoSessionType, byte[] configBytes) {
        if (configBytes == null)
            throw new CryptoException("configBytes is required");
        return new CryptoSessionInit<>(cryptoSessionType, configBytes);
    }

    /**
     * config.json 파일을 InputStream 형태로 읽어서 CryptoSession 객체를 생성
     * config.json 파일에는 다음 키가 포함되어야 함
     * aws_kms_key_arn, aws_access_key_id, aws_secret_access_key, seed, credential
     * @param cryptoSessionType CryptoSessionType
     * @param configInputStream config.json 파일을 읽어들인 InputStream
     * @return CryptoSessionInit
     */
    public static <T extends Enum<T>> CryptoSessionInit<T> ofInputStream(T cryptoSessionType, InputStream configInputStream) {
        if (configInputStream == null)
            throw new CryptoException("configInputStream is required");
        return new CryptoSessionInit<>(cryptoSessionType, configInputStream);
    }

    /**
     * path 경로에 있는 config.json 파일을 읽어서 CryptoSession 객체를 생성
     * @param cryptoSessionType CryptoSessionType
     * @param path config.json 파일의 경로
     * @return CryptoSessionInit
     */
    public static <T extends Enum<T>> CryptoSessionInit<T> ofPath(T cryptoSessionType, String path) {
        if (path == null)
            throw new CryptoException("path is required");
        if (!Files.exists(Paths.get(path)))
            throw new CryptoException("Could not find 'config.json' file at that path");
        return new CryptoSessionInit<>(cryptoSessionType, path);
    }

    /**
     * Map 형태로 전달된 구성을 사용하여 CryptoSession 객체를 생성
     * 구성 맵에는 다음 키가 포함되어야 함
     * @param cryptoSessionType CryptoSessionType
     * @param configLocalMap configMap(key, iv, seed, credential)
     * @return CryptoSessionInit
     */
    public static <T extends Enum<T>> CryptoSessionInit<T> ofLocalMap(T cryptoSessionType, Map<String, String> configLocalMap) {
        List<String> errors = new ArrayList<>();
        if (configLocalMap == null)
            throw new CryptoException("configLocalMap is required: Elements in configLocalMap must contain values for the keys (key, iv, seed, credential)");
        String key = configLocalMap.get("key");
        String iv = configLocalMap.get("iv");
        String seed = configLocalMap.get("seed");
        String credential = configLocalMap.get("credential");
        configLocalMap.forEach((k, v) -> {
            if (v == null || v.isEmpty()) {
                errors.add(k);
            }
        });
        boolean allMatch = Stream.of(key, iv, seed, credential)
                .allMatch(s -> s != null && !s.isEmpty());
        if (!allMatch)
            throw new CryptoException("The config local map does not contain the required keys: " + errors);
        return new CryptoSessionInit<>(cryptoSessionType, configLocalMap, key, iv);
    }

    /**
     * Map 형태로 전달된 구성을 사용하여 CryptoSession 객체를 생성
     * 구성 맵에는 다음 키가 포함되어야 함
     * @param cryptoSessionType CryptoSessionType
     * @param configMap configMap(aws_kms_key_arn, aws_access_key_id, aws_secret_access_key, seed, credential)
     * @return CryptoSessionInit
     */
    public static <T extends Enum<T>> CryptoSessionInit<T> ofMap(T cryptoSessionType, Map<String, String> configMap) {
        List<String> errors = new ArrayList<>();
        if (configMap == null)
            throw new CryptoException("configMap is required: Elements in configMap must contain values for the keys (aws_kms_key_arn, aws_access_key_id, aws_secret_access_key, seed, credential)");
        String aws_kms_key_arn = configMap.get("aws_kms_key_arn");
        String aws_access_key_id = configMap.get("aws_access_key_id");
        String aws_secret_access_key = configMap.get("aws_secret_access_key");
        String seed = configMap.get("seed");
        String credential = configMap.get("credential");
        configMap.forEach((k, v) -> {
            if (v == null || v.isEmpty()) {
                errors.add(k);
            }
        });
        boolean allMatch = Stream.of(aws_kms_key_arn, aws_access_key_id, aws_secret_access_key, seed, credential)
                .allMatch(s -> s != null && !s.isEmpty());
        if (!allMatch)
            throw new CryptoException("The config map does not contain the required keys: " + errors);
        return new CryptoSessionInit<>(cryptoSessionType, configMap);
    }

    /**
     * 파라메터를 전달하여 CryptoSessionInit 객체를 생성
     * 파라메터는 다음 값들이 필수적으로 추가되어야 함
     * @param cryptoSessionType CryptoSessionType
     * @param key key
     * @param iv iv
     * @param seed seed
     * @param credential credential
     * @return CryptoSessionInit
     */
    public static <T extends Enum<T>> CryptoSessionInit<T> ofLocalParams(T cryptoSessionType, String key, String iv, String seed, String credential) {
        List<String> errors = new ArrayList<>();
        if (key == null || key.isEmpty()) errors.add("key");
        if (iv == null || iv.isEmpty()) errors.add("iv");
        if (seed == null || seed.isEmpty()) errors.add("seed");
        if (credential == null || credential.isEmpty()) errors.add("credential");
        if (!errors.isEmpty())
            throw new CryptoException("The following parameters are required: " + errors);
        return new CryptoSessionInit<>(cryptoSessionType, key, iv, seed, credential);
    }

    /**
     * 파라메터를 전달하여 CryptoSessionInit 객체를 생성
     * 파라메터는 다음 값들이 필수적으로 추가되어야 함
     * @param cryptoSessionType CryptoSessionType
     * @param aws_kms_key_arn aws_kms_key_arn
     * @param aws_access_key_id aws_access_key_id
     * @param aws_secret_access_key aws_secret_access_key
     * @param seed seed
     * @param credential credential
     * @return CryptoSessionInit
     */
    public static <T extends Enum<T>> CryptoSessionInit<T> ofParams(T cryptoSessionType, String aws_kms_key_arn, String aws_access_key_id, String aws_secret_access_key, String seed, String credential) {
        List<String> errors = new ArrayList<>();
        if (aws_kms_key_arn == null || aws_kms_key_arn.isEmpty()) errors.add("aws_kms_key_arn");
        if (aws_access_key_id == null || aws_access_key_id.isEmpty()) errors.add("aws_access_key_id");
        if (aws_secret_access_key == null || aws_secret_access_key.isEmpty()) errors.add("aws_secret_access_key");
        if (seed == null || seed.isEmpty()) errors.add("seed");
        if (credential == null || credential.isEmpty()) errors.add("credential");
        if (!errors.isEmpty())
            throw new CryptoException("The following parameters are required: " + errors);
        return new CryptoSessionInit<>(cryptoSessionType, aws_kms_key_arn, aws_access_key_id, aws_secret_access_key, seed, credential);
    }

    public enum InitType {
        BASE_PATH, BYTES, INPUT_STREAM, PATH, MAP, LOCAL_MAP, PARAMS, LOCAL_PARAMS
    }

    /**
     * InitType 별로 CryptoSession 객체를 초기화
     * @param cryptoSessionInit CryptoSessionInit
     */
    static <T extends Enum<T>> CryptoSession initCryptoSession(CryptoSessionInit<T> cryptoSessionInit) {

        if (cryptoSessionInit == null)
            throw new CryptoException("CryptoSessionInit is required!");

        CryptoSession cryptoSession = null;

        writeLog("Initialize CryptoSession Bean");

        switch (cryptoSessionInit.getInitType()) {
            case BYTES:
                cryptoSession = new CryptoSession(cryptoSessionInit.getConfigBytes());
                break;
            case INPUT_STREAM:
                cryptoSession = new CryptoSession(cryptoSessionInit.getConfigInputStream());
                break;
            case PATH:
                cryptoSession = new CryptoSession(cryptoSessionInit.getPath());
                break;
            case MAP:
                cryptoSession = new CryptoSession(cryptoSessionInit.getConfigMap());
                break;
            case LOCAL_MAP:
                cryptoSession = new CryptoSession(cryptoSessionInit.getConfigLocalMap(), cryptoSessionInit.getKey(), cryptoSessionInit.getIv());
                break;
            case PARAMS:
                cryptoSession = new CryptoSession(cryptoSessionInit.getAwsKmsKeyArn(), cryptoSessionInit.getAwsAccessKeyId(), cryptoSessionInit.getAwsSecretAccessKey(), cryptoSessionInit.getSeed(), cryptoSessionInit.getCredential());
                break;
            case LOCAL_PARAMS:
                cryptoSession = new CryptoSession(cryptoSessionInit.getKey(), cryptoSessionInit.getIv(), cryptoSessionInit.getSeed(), cryptoSessionInit.getCredential());
                break;
            case BASE_PATH:
                cryptoSession = new CryptoSession();
                break;
        }
        if (cryptoSession == null)
            throw new CryptoException("Config file loading failed");
        return cryptoSession;
    }

    private static void writeLog(String message) {
        // return new SimpleDateFormat("yyyy-MM-dd HH:mm:ss:SSS").format(new Date());
        String prefix = "[Crypto][INFO]";
        String now = OffsetDateTime.now().format(DateTimeFormatter.ISO_OFFSET_DATE_TIME);
        System.out.format("%s[%s] %s\n",prefix, now, message);
    }

    @Override
    public String toString() {
        return "CryptoSessionInit{" +
                "cryptoSessionType=" + cryptoSessionType +
                ", initType=" + initType +
                ", configBytes=" + Arrays.toString(configBytes) +
                ", configInputStream=" + configInputStream +
                ", path='" + path + '\'' +
                ", configMap=" + configMap +
                ", configLocalMap=" + configLocalMap +
                ", awsKmsKeyArn='" + awsKmsKeyArn + '\'' +
                ", awsAccessKeyId='" + awsAccessKeyId + '\'' +
                ", awsSecretAccessKey='" + awsSecretAccessKey + '\'' +
                ", key='" + key + '\'' +
                ", iv='" + iv + '\'' +
                ", seed='" + seed + '\'' +
                ", credential='" + credential + '\'' +
                '}';
    }
}
