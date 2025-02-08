package com.freelife.crypto.factory;

import com.freelife.crypto.config.CryptoSessionType;
import com.freelife.crypto.config.factorybean.CryptoFactoryBeanConfig;
import com.freelife.crypto.config.factorybean.CryptoSessions;
import com.freelife.crypto.core.CryptoException;
import com.freelife.crypto.core.CryptoSession;
import org.junit.jupiter.api.Test;
import org.springframework.boot.test.context.SpringBootTest;
import org.springframework.context.annotation.Import;

import javax.annotation.Resource;

import static org.junit.jupiter.api.Assertions.*;

/**
 * CryptoSessionFactoryBean 을 사용한 CryptoSession 테스트
 * Created by mskwon on 2024. 10. 17..
 */
@SpringBootTest(webEnvironment = SpringBootTest.WebEnvironment.RANDOM_PORT)
@Import({CryptoFactoryBeanConfig.class})
// @ComponentScan(basePackages = "com.freelife.config.factorybean")
class CryptoFactoryBeanConfigTest {

    @Resource
    private CryptoSession hotelCryptoSession;

    @Resource
    private CryptoSession airCryptoSession;

    @Resource
    private CryptoSessions<CryptoSessionType> cryptoSessions;

    @Test
    void hotelCryptoSessionTest() {
        String plainText = "Hello Crypto!";
        String encrypt = hotelCryptoSession.encrypt(plainText);
        String decrypt = hotelCryptoSession.decrypt(encrypt);
        assertAll(
            () -> assertNotNull(hotelCryptoSession),
            () -> assertEquals(decrypt, plainText)
        );
    }

    @Test
    void airCryptoSessionTest() {
        String plainText = "Hello Crypto!";
        String encrypt = airCryptoSession.encrypt(plainText);
        String decrypt = airCryptoSession.decrypt(encrypt);
        assertAll(
            () -> assertNotNull(airCryptoSession),
            () -> assertEquals(decrypt, plainText)
        );
    }

    @Test
    void multi_Initializing_cryptoSession() {
        String plainText = "Hello Crypto!";
        String encryptHotel = cryptoSessions.getHotelCryptoSession().encrypt(plainText);
        String decryptHotel = cryptoSessions.getHotelCryptoSession().decrypt(encryptHotel);
        String encryptAir = cryptoSessions.getAirCryptoSession().encrypt(plainText);
        String decryptAir = cryptoSessions.getAirCryptoSession().decrypt(encryptAir);
        assertAll(
            () -> assertNotNull(cryptoSessions.getHotelCryptoSession()),
            () -> assertNotNull(cryptoSessions.getAirCryptoSession()),
            () -> assertEquals(decryptHotel, plainText),
            () -> assertEquals(decryptAir, plainText)
        );
    }

    @Test
    void cross_valication_test() {
        String plainText = "Hello Crypto!";
        String encryptHotel = cryptoSessions.getHotelCryptoSession().encrypt(plainText);
        String encryptAir = cryptoSessions.getAirCryptoSession().encrypt(plainText);
        assertThrows(CryptoException.class, () -> {
            cryptoSessions.getHotelCryptoSession().decrypt(encryptAir);
            cryptoSessions.getAirCryptoSession().decrypt(encryptHotel);
        });
    }

    @Test
    void failed_decrypt_test() {
        String plainText = "Hello Crypto!";
        String encrypt = cryptoSessions.getHotelCryptoSession().encrypt(plainText);
        assertThrows(CryptoException.class, () -> cryptoSessions.getAirCryptoSession().decrypt(encrypt));
    }

}