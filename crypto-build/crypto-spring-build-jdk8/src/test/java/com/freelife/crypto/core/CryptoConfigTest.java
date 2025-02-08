package com.freelife.crypto.core;

import com.freelife.crypto.config.CryptoConfig;
import org.junit.jupiter.api.Test;
import org.springframework.boot.test.context.SpringBootTest;
import org.springframework.context.annotation.Import;

import javax.annotation.Resource;

import static org.junit.jupiter.api.Assertions.*;

/**
 * CryptoSession 빈을 사용한 CryptoSession 테스트
 */
// @Disabled
@SpringBootTest(webEnvironment = SpringBootTest.WebEnvironment.RANDOM_PORT)
@Import({CryptoConfig.class})
class CryptoConfigTest {

    // @Autowired
    // @Qualifier("basicCryptoSession")
    @Resource
    private CryptoSession basicCryptoSession;

    // @Autowired: 필드 타입을 기준으로 빈을 찾음
    // @Resource: 필드 이름을 기준으로 빈을 찾음

    // @Autowired
    // @Qualifier("hotelBasicCryptoSession")
    @Resource
    private CryptoSession hotelBasicCryptoSession;

    // @Autowired
    // @Qualifier("airBasicCryptoSession")
    @Resource
    private CryptoSession airBasicCryptoSession;

    @Test
    void multi_Initializing_cryptoSession() {
        assertAll(
            () -> assertNotNull(basicCryptoSession),
            () -> assertNotNull(hotelBasicCryptoSession),
            () -> assertNotNull(airBasicCryptoSession)
        );
    }

    @Test
    void cryptoSessionTest() {
        String plaintext = "Hello Crypto!";
        String encrypt = basicCryptoSession.encrypt(plaintext);
        String decrypt = basicCryptoSession.decrypt(encrypt);
        assertEquals(decrypt, plaintext);
    }

    @Test
    void hotelCryptoSessionTest() {
        String plaintext = "Hello Crypto!";
        String encrypt = hotelBasicCryptoSession.encrypt(plaintext);
        String decrypt = hotelBasicCryptoSession.decrypt(encrypt);
        assertEquals(decrypt, plaintext);
    }

    @Test
    void airCryptoSessionTest() {
        String plaintext = "Hello Crypto!";
        String encrypt = airBasicCryptoSession.encrypt(plaintext);
        String decrypt = airBasicCryptoSession.decrypt(encrypt);
        assertEquals(decrypt, plaintext);
    }

    @Test
    void hotelCryptoSessionAllTest() {
        String plaintext = "Hello Crypto!";
        String encrypt = basicCryptoSession.encrypt(plaintext);
        String decrypt = basicCryptoSession.decrypt(encrypt);
        String encryptHotel = hotelBasicCryptoSession.encrypt(plaintext);
        String decryptHotel = hotelBasicCryptoSession.decrypt(encryptHotel);
        String encryptAir = airBasicCryptoSession.encrypt(plaintext);
        String decryptAir = airBasicCryptoSession.decrypt(encryptAir);
        assertAll(
            () -> assertEquals(decrypt, plaintext),
            () -> assertEquals(decryptHotel, plaintext),
            () -> assertEquals(decryptAir, plaintext)
        );
    }

    @Test
    void cross_valication_test() {
        String plaintext = "Hello Crypto!";
        String encryptHotel = hotelBasicCryptoSession.encrypt(plaintext);
        String encryptAir = airBasicCryptoSession.encrypt(plaintext);
        String decryptHotel = hotelBasicCryptoSession.decrypt(encryptHotel);
        String decryptAir = airBasicCryptoSession.decrypt(encryptAir);
        assertAll(
            () -> assertEquals(decryptHotel, plaintext),
            () -> assertEquals(decryptAir, plaintext)
        );
        assertThrows(CryptoException.class, () -> {
            hotelBasicCryptoSession.decrypt(encryptAir);
            airBasicCryptoSession.decrypt(encryptHotel);
        });
    }

}