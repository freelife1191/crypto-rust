package com.freelife.crypto.factory;

import com.freelife.crypto.config.CryptoSessionType;
import com.freelife.crypto.config.factory.CryptoFactoryConfig;
import com.freelife.crypto.core.CryptoException;
import com.freelife.crypto.core.CryptoSession;
import jakarta.annotation.Resource;
import lombok.extern.slf4j.Slf4j;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;
import org.springframework.boot.test.context.SpringBootTest;
import org.springframework.context.annotation.Import;

import static org.junit.jupiter.api.Assertions.*;

/**
 * CryptoSessionFactory 를 사용한 CryptoSession 테스트
 * Created by mskwon on 2024. 10. 23..
 */
@Slf4j
// @Disabled
@SpringBootTest(webEnvironment = SpringBootTest.WebEnvironment.RANDOM_PORT)
@Import({CryptoFactoryConfig.class})
class CryptoFactoryConfigTest {

    @Resource
    private CryptoSessionFactory<CryptoSessionType> cryptoSessionFactory;

    @Test
    void base_Initializing_cryptoSession() {
        String plainText = "Hello Crypto!";
        CryptoSession hotelCryptoSession = cryptoSessionFactory.of(CryptoSessionType.HOTEL);
        CryptoSession airCryptoSession = cryptoSessionFactory.of(CryptoSessionType.AIR);

        String encryptHotel = hotelCryptoSession.encrypt(plainText);
        String decryptHotel = hotelCryptoSession.decrypt(encryptHotel);
        String encryptAir = airCryptoSession.encrypt(plainText);
        String decryptAir = airCryptoSession.decrypt(encryptAir);
        assertAll(
                () -> assertNotNull(hotelCryptoSession),
                () -> assertNotNull(airCryptoSession),
                () -> assertEquals(decryptHotel, plainText),
                () -> assertEquals(decryptAir, plainText)
        );
    }

    @Test
    void multi_Initializing_cryptoSession() {
        String plainText = "Hello Crypto!";
        CryptoSession hotelCryptoSession = cryptoSessionFactory.of(CryptoSessionType.HOTEL);
        CryptoSession airCryptoSession = cryptoSessionFactory.of(CryptoSessionType.AIR);

        String encryptHotel = hotelCryptoSession.encrypt(plainText);
        String decryptHotel = hotelCryptoSession.decrypt(encryptHotel);
        String encryptAir = airCryptoSession.encrypt(plainText);
        String decryptAir = airCryptoSession.decrypt(encryptAir);
        assertAll(
                () -> assertNotNull(hotelCryptoSession),
                () -> assertNotNull(airCryptoSession),
                () -> assertEquals(decryptHotel, plainText),
                () -> assertEquals(decryptAir, plainText)
        );
    }

    @Test
    void cryptoSessionDuplicateCallTest() {
        String plainText = "Hello Crypto!";
        for (int i = 0; i < 10; i++) {
            CryptoSession hotelCryptoSession = cryptoSessionFactory.of(CryptoSessionType.HOTEL);
            CryptoSession airCryptoSession = cryptoSessionFactory.of(CryptoSessionType.AIR);

            String encryptHotel = hotelCryptoSession.encrypt(plainText);
            log.debug("CryptoSession[{}] encrypt: {}", i, encryptHotel);
            String decryptHotel = hotelCryptoSession.decrypt(encryptHotel);
            log.debug("CryptoSession[{}] decrypt: {}", i, decryptHotel);
            String encryptAir = airCryptoSession.encrypt(plainText);
            log.debug("CryptoSession[{}] encrypt: {}", i, encryptAir);
            String decryptAir = airCryptoSession.decrypt(encryptAir);
            log.debug("CryptoSession[{}] decrypt: {}", i, decryptAir);
            assertAll(
                    () -> assertNotNull(hotelCryptoSession),
                    () -> assertNotNull(airCryptoSession),
                    () -> assertEquals(decryptHotel, plainText),
                    () -> assertEquals(decryptAir, plainText)
            );
        }
    }

    @Test
    @DisplayName("다른 타입의 CryptoSession을 사용하여 암호화된 문자열을 복호화하면 CryptoException이 발생")
    void failed_decrypt_test() {
        String plainText = "Hello Crypto!";
        String encrypt = cryptoSessionFactory.of(CryptoSessionType.HOTEL).encrypt(plainText);
        assertThrows(CryptoException.class, () -> cryptoSessionFactory.of(CryptoSessionType.AIR).decrypt(encrypt));
    }

}