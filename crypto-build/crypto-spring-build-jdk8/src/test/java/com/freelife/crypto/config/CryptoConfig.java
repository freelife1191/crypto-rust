package com.freelife.crypto.config;

import com.freelife.crypto.core.CryptoSession;
import lombok.extern.slf4j.Slf4j;
import org.springframework.beans.factory.config.ConfigurableBeanFactory;
import org.springframework.boot.test.context.TestConfiguration;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Primary;
import org.springframework.context.annotation.Scope;
import org.springframework.core.io.ClassPathResource;

import java.io.IOException;

import static com.freelife.util.ByteUtils.toByteArray;

@Slf4j
@TestConfiguration
public class CryptoConfig {

    /**
     * Singleton CryptoSession Bean 을 생성
     * @return CryptoSession
     */
    @Primary
    @Bean
    @Scope(value = ConfigurableBeanFactory.SCOPE_SINGLETON)
    public CryptoSession basicCryptoSession() throws Exception {
        ClassPathResource classPathResource = new ClassPathResource("crypto/default/config.json");
        CryptoSession cryptoSession = new CryptoSession(toByteArray(classPathResource.getInputStream()));
        log.info("Initializing basicCryptoSession: {}", cryptoSession);
        return cryptoSession;
    }

    /**
     * Singleton hotelBasicCryptoSession Bean 을 생성
     * @return CryptoSession
     */
    @Bean
    @Scope(value = ConfigurableBeanFactory.SCOPE_SINGLETON)
    public CryptoSession hotelBasicCryptoSession() throws IOException {
        ClassPathResource classPathResource = new ClassPathResource("crypto/hotel/config.json");
        CryptoSession cryptoSession = new CryptoSession(toByteArray(classPathResource.getInputStream()));
        log.info("Initializing hotelBasicCryptoSession: {}", cryptoSession);
        return cryptoSession;
    }

    /**
     * Singleton airBasicCryptoSession Bean 을 생성
     * @return CryptoSession
     */
    @Bean
    @Scope(value = ConfigurableBeanFactory.SCOPE_SINGLETON)
    public CryptoSession airBasicCryptoSession() throws IOException {
        ClassPathResource classPathResource = new ClassPathResource("crypto/air/config.json");
        CryptoSession cryptoSession = new CryptoSession(toByteArray(classPathResource.getInputStream()));
        log.info("Initializing airBasicCryptoSession: {}", cryptoSession);
        return cryptoSession;
    }
}