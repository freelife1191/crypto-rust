package com.freelife.crypto.config.factory;

import com.freelife.crypto.config.CryptoSessionType;
import com.freelife.crypto.factory.CryptoSessionFactory;
import com.freelife.crypto.factory.CryptoSessionInit;
import org.springframework.boot.test.context.TestConfiguration;
import org.springframework.context.annotation.Bean;
import org.springframework.core.io.ClassPathResource;

import java.io.IOException;
import java.util.List;

/**
 * CryptoSessionFactory 객체를 사용해 CryptoSessionFactory Bean을 생성
 * Created by mskwon on 2024. 10. 17..
 */
@TestConfiguration
public class CryptoFactoryConfig {

    /**
     * CryptoSessionFactory Bean 을 생성하여 다중 세션을 관리
     */
    @Bean
    public CryptoSessionFactory<CryptoSessionType> cryptoSessionFactory() throws IOException {
        List<CryptoSessionInit<CryptoSessionType>> cryptoSessionInits = List.of(
                CryptoSessionInit.ofInputStream(
                        CryptoSessionType.HOTEL,
                        new ClassPathResource("crypto/hotel/config.json").getInputStream()),
                // CryptoSessionInit.ofInputStream(
                //         CryptoSessionType.AIR,
                //         new ClassPathResource("crypto/air/config.json").getInputStream())
                CryptoSessionInit.ofBasePath(CryptoSessionType.AIR)
        );
        return new CryptoSessionFactory<>(cryptoSessionInits);
    }
    
}