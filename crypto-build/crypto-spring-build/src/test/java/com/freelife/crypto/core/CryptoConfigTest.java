package com.freelife.crypto.core;

import com.freelife.crypto.config.CryptoConfig;
import jakarta.annotation.Resource;
import org.junit.jupiter.api.Test;
import org.springframework.boot.test.context.SpringBootTest;
import org.springframework.context.annotation.Import;

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
        executeSessionTest(basicCryptoSession);
    }

    @Test
    void hotelCryptoSessionTest() {
        executeSessionTest(hotelBasicCryptoSession);
    }

    @Test
    void airCryptoSessionTest() {
        executeSessionTest(airBasicCryptoSession);
    }

    @Test
    void hotelCryptoSessionAllTest() {
        executeSessionTest(basicCryptoSession);
        executeSessionTest(hotelBasicCryptoSession);
        executeSessionTest(airBasicCryptoSession);
    }

    @Test
    void cross_valication_test() {
        String plaintext = "Hello Crypto!";
        executeSessionTest(hotelBasicCryptoSession);
        executeSessionTest(airBasicCryptoSession);
        String encryptHotel = hotelBasicCryptoSession.encrypt(plaintext);
        String encryptAir = airBasicCryptoSession.encrypt(plaintext);
        assertThrows(CryptoException.class, () -> {
            hotelBasicCryptoSession.decrypt(encryptAir);
            airBasicCryptoSession.decrypt(encryptHotel);
        });
    }

    private void executeSessionTest(CryptoSession session) {
        String plaintext = "Hello Crypto!";
        String encrypt = session.encrypt(plaintext);
        String decrypt = session.decrypt(encrypt);
        String hash = session.encrypt_id(plaintext, 400);
        // assertThat(decrypt).isEqualTo(plaintext);
        assertAll(
            () -> assertEquals(decrypt, plaintext),
            () -> assertTrue(hash.length() >= 88)
        );
    }

}