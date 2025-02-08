package com.freelife.crypto.config.factorybean;

import com.freelife.crypto.core.CryptoSession;
import lombok.AllArgsConstructor;
import lombok.Data;
import lombok.NoArgsConstructor;

import java.util.HashMap;
import java.util.Map;

/**
 * CryptoSessionType 별로 Bean 으로 설정한 CryptoSession 을 사용하기 위한 클래스
 * Created by mskwon on 2024. 10. 17..
 */
@Data
@NoArgsConstructor
@AllArgsConstructor
public class CryptoSessions<T extends Enum<T>> {
    private Map<T, CryptoSession> cryptoSessionMap = new HashMap<T, CryptoSession>();

    private CryptoSession hotelCryptoSession;
    private CryptoSession airCryptoSession;
}
