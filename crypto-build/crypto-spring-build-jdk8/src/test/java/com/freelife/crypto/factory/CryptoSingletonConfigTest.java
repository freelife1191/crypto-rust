package com.freelife.crypto.factory;

import com.freelife.crypto.config.impl.CryptoSessionSingleton;
import com.freelife.crypto.core.CryptoSession;
import lombok.extern.slf4j.Slf4j;
import org.junit.jupiter.api.Test;
import org.springframework.boot.test.context.SpringBootTest;

import java.io.IOException;

import static org.junit.jupiter.api.Assertions.*;

/**
 * CryptoSessionImpl 싱글턴 인스턴스 테스트
 * Created by mskwon on 2024. 10. 23..
 */
// @Disabled
@Slf4j
@SpringBootTest(webEnvironment = SpringBootTest.WebEnvironment.RANDOM_PORT)
class CryptoSingletonConfigTest {

    @Test
    void cryptoSessionInstance() throws IOException {
        String plainText = "Hello Crypto!";
        for (int i = 0; i < 10; i++) {
            CryptoSession hotelInstance = CryptoSessionSingleton.getHotelInstance();
            String encrypt = hotelInstance.encrypt(plainText);
            String decrypt = hotelInstance.decrypt(encrypt);
            assertAll(
                    () -> assertNotNull(hotelInstance),
                    () -> assertEquals(decrypt, plainText)
            );
        }

    }

    @Test
    void cryptoSessionDuplicateCallTest() throws IOException {
        String plainText = "Hello Crypto!";
        for (int i = 0; i < 10; i++) {
            CryptoSession hotelInstance = CryptoSessionSingleton.getHotelInstance();
            String encrypt = hotelInstance.encrypt(plainText);
            String decrypt = hotelInstance.decrypt(encrypt);
            assertAll(
                    () -> assertNotNull(hotelInstance),
                    () -> assertEquals(decrypt, plainText)
            );
        }
    }

    @Test
    void failed_decrypt_test() throws IOException {
        String plainText = "Hello Crypto!";
        // HOTEL 싱글턴 인스턴스 생성
        CryptoSession hotelInstance = CryptoSessionSingleton.getHotelInstance();
        // AIR 싱글턴 인스턴스 생성 했지만 HOTEL 싱글턴 인스턴스가 이미 생성되어있으므로 생성되지 않음
        CryptoSession airInstance = CryptoSessionSingleton.getAirInstance();
        String encrypt = hotelInstance.encrypt(plainText);
        // AIR 인스턴스로 decrypt 시도 시 실패해야되지만 HOTEL 싱글턴 인스턴스이기 때문에 decrypt 성공
        String decrypt = airInstance.decrypt(encrypt);
        assertAll(
                () -> assertNotNull(hotelInstance),
                () -> assertNotNull(airInstance),
                // AIR 인스턴스도 HOTEL 싱글턴 인스턴스이므로 decrypt 성공
                () -> assertEquals(decrypt, plainText)
        );
    }

}