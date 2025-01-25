package com.freelife.crypto.config.factorybean;

import com.freelife.crypto.config.CryptoSessionType;
import com.freelife.crypto.factory.CryptoSessionInit;
import lombok.extern.slf4j.Slf4j;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.core.io.ClassPathResource;

import java.io.IOException;

/**
 * FactoryBean 을 이용한 CryptoSession Bean 생성 예제
 * Created by mskwon on 2024. 10. 17..
 */
@Slf4j
@Configuration
public class CryptoFactoryBeanConfig {

    /**
     * Hotel CryptoSession Bean 생성
     * FactoryBean 타입으로 리턴하지만 실제 사용시에는 CryptoSession 타입으로 사용
     * ex1: @Resource(name = "hotelCryptoSession") CryptoSession hotelCryptoSession;
     * ex2: @Autowired CryptoSession hotelCryptoSession;
     */
    @Bean
    public CryptoSessionFactoryBean<CryptoSessionType> hotelCryptoSession() throws IOException {
        ClassPathResource classPathResource = new ClassPathResource("crypto/hotel/config.json");
        return CryptoSessionFactoryBean.of(
                CryptoSessionInit.ofInputStream(
                        CryptoSessionType.HOTEL,
                        classPathResource.getInputStream()));
    }

    /**
     * Air CryptoSession Bean 생성
     * FactoryBean 타입으로 리턴하지만 실제 사용시에는 CryptoSession 타입으로 사용
     * ex1: @Resource(name = "airCryptoSession") CryptoSession airCryptoSession;
     * ex2: @Autowired CryptoSession airCryptoSession;
     */
    @Bean
    public CryptoSessionFactoryBean<CryptoSessionType> airCryptoSession() throws IOException {
        ClassPathResource classPathResource = new ClassPathResource("crypto/air/config.json");
        return CryptoSessionFactoryBean.of(
                CryptoSessionInit.ofBytes(
                        CryptoSessionType.AIR,
                        classPathResource.getInputStream().readAllBytes()));
    }

    /**
     * 생성된 FactoryBean 을 CryptoSessions 에 등록하여 다중 세션을 관리
     */
    @Bean
    public CryptoSessions<CryptoSessionType> cryptoSessions() throws Exception {
        CryptoSessions<CryptoSessionType> cryptoSessions = new CryptoSessions<>();
        cryptoSessions.setHotelCryptoSession(hotelCryptoSession().getObject());
        cryptoSessions.setAirCryptoSession(airCryptoSession().getObject());
        return cryptoSessions;
    }
}
