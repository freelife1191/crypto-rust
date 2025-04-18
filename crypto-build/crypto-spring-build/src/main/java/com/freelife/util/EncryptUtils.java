package com.freelife.util;

import java.security.SecureRandom;
import java.util.HexFormat;

/**
 * Created by mskwon on 2025. 4. 2..
 */
public class EncryptUtils {

    /**
     * 주어진 길이의 랜덤 키를 생성합니다
     * @param length 생성할 키의 길이
     * @return 랜덤 키
     */
    public static String generateKey(int length) {
        return encodeHex(randBytes(length));
    }

    /**
     * 바이트 배열을 Hex 문자열로 인코딩합니다
     * @param bytes 인코딩할 바이트 배열
     * @return Hex 문자열
     */
    public static String encodeHex(byte[] bytes) {
        HexFormat hexFormat = HexFormat.of();
        return hexFormat.formatHex(bytes);
    }

    /**
     * Hex 문자열을 바이트 배열로 디코딩합니다
     * @param hex 디코딩할 Hex 문자열
     * @return 바이트 배열
     */
    public static byte[] decodeHex(String hex) {
        HexFormat hexFormat = HexFormat.of();
        return hexFormat.parseHex(hex);
    }

    /**
     * Hex 문자열을 바이트 배열로 디코딩하고, 이를 문자열로 변환합니다
     * @param hex 디코딩할 Hex 문자열
     * @return 문자열
     */
    public static String decodeHexToString(String hex) {
        byte[] bytes = decodeHex(hex);
        return new String(bytes);
    }

    /**
     * 바이트 배열을 Hex 문자열로 변환합니다
     * @param a 변환할 바이트 배열
     * @return Hex 문자열
     */
    public static String byteArrayToHex(byte[] a) {
        StringBuilder sb = new StringBuilder(a.length * 2);
        for (byte b: a) {
           sb.append(String.format("%02x", b));
        }
        return sb.toString();
    }

    /**
     * Hex 문자열을 바이트 배열로 변환합니다
     * @param hex 변환할 Hex 문자열
     * @return 바이트 배열
     */
    public static byte[] hexToByteArray(String hex) {
        byte[] result = new byte[hex.length() / 2];
        for (int i = 0; i < hex.length(); i += 2) {
            // 각 2자리의 Hex 문자열을 Integer로 변환하여 byte로 저장
            // result[i / 2] = (byte) ((Character.digit(hex.charAt(i), 16) << 4) + Character.digit(hex.charAt(i + 1), 16));
            // 각 2자리 Hex 문자열을 Byte로 변환
            result[i / 2] = (byte) Integer.parseInt(hex.substring(i, i + 2), 16);
        }
        return result;
    }

    /**
     * 랜덤 바이트 배열을 생성합니다
     * @param length 생성할 바이트 배열의 길이
     * @return 랜덤 바이트 배열
     */
    public static byte[] randBytes(int length) {
        // 주어진 길이의 바이트 배열을 생성합니다.
        byte[] bytes = new byte[length];
        randBytes(bytes);
        // 랜덤 값으로 채워진 바이트 배열을 반환합니다
        return bytes;
    }

    /**
     * SecureRandom 클래스는 자바에서 암호학적으로 강력한 난수 생성기를 제공하는 클래스
     * @param bytes
     */
    private static void randBytes(byte[] bytes) {
        // SecureRandom 클래스는 자바에서 암호학적으로 강력한 난수 생성기를 제공하는 클래스입니다
        // 이 클래스는 예측 불가능한 난수를 생성하여 보안이 중요한 작업에 사용됩니다
        SecureRandom secureRandom = new SecureRandom();
        // 메서드를 호출하여 바이트 배열을 랜덤 값으로 채웁니다
        secureRandom.nextBytes(bytes);
    }

}
