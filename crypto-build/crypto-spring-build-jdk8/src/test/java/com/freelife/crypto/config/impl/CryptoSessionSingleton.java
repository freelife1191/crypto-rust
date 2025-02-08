package com.freelife.crypto.config.impl;

import com.freelife.crypto.config.CryptoSessionType;
import com.freelife.crypto.core.CryptoSession;
import com.freelife.crypto.factory.CryptoSessionImpl;
import com.freelife.crypto.factory.CryptoSessionInit;
import lombok.extern.slf4j.Slf4j;
import org.springframework.core.io.ClassPathResource;
import org.springframework.core.io.Resource;

import java.io.IOException;
import java.nio.file.Path;
import java.nio.file.Paths;

/**
 * CryptoSession Singleton 생성
 * 여러개의 인스턴스 메서드를 생성하여도 단 한개의 인스턴스만 생성되므로 사용에 주의할 것
 * Created by mskwon on 2024. 10. 17..
 */
@Slf4j
public class CryptoSessionSingleton {

    /**
     * Hotel CryptoSession 생성
     */
    public static CryptoSession getHotelInstance() throws IOException {
        Resource resource =  new ClassPathResource(Paths.get("crypto", "hotel", "config.json").toString());
        return CryptoSessionImpl.getInstance(
                CryptoSessionInit.ofPath(
                        CryptoSessionType.HOTEL, resource.getFile().getAbsolutePath()));
    }

    /**
     * Air CryptoSession 생성
     */
    public static CryptoSession getAirInstance() throws IOException {
        return CryptoSessionImpl.getInstance(
                CryptoSessionInit.ofInputStream(
                        CryptoSessionType.AIR,
                        new ClassPathResource(Paths.get("crypto", "air", "config.json").toString()).getInputStream()));
    }

}
