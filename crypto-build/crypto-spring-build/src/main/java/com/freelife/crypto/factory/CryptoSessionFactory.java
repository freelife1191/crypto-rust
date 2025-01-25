package com.freelife.crypto.factory;

import com.freelife.crypto.core.CryptoException;
import com.freelife.crypto.core.CryptoSession;

import java.util.HashMap;
import java.util.List;
import java.util.Map;

import static com.freelife.crypto.factory.CryptoSessionInit.initCryptoSession;

/**
 * Multiple Database 설정시 CryptoSession 을 여러개 설정해서 사용할 필요가 있을 경우 사용
 * Created by mskwon on 2024. 10. 16..
 */
public class CryptoSessionFactory<T extends Enum<T>> {

    private final Map<T, CryptoSession> cryptoSessionMap = new HashMap<>();

    /**
     * CryptoSessionFactory 생성자
     * CryptoSessionInit List 를 받아서 CryptoSession 을 초기화
     * 여러 개의 Database 사용시 각 Database 별로 Config 를 CryptoSessionFactory 로 CryptoSession 을 다중 설정
     * @param cryptoSessionInitList CryptoSessionInit List
     */
    public CryptoSessionFactory(List<CryptoSessionInit<T>> cryptoSessionInitList) {
        if (cryptoSessionInitList == null || cryptoSessionInitList.isEmpty())
            throw new CryptoException("CryptoSessionInitList is required!");
        cryptoSessionInitList.forEach(init -> cryptoSessionMap.put(init.getCryptoSessionType(), initCryptoSession(init)));
    }

    /**
     * CryptoSessionFactory 에 등록된 CryptoSession 객체를 CryptoSessionType 별로 가져옴
     * @param cryptoSessionType CryptoSessionType
     * @return CryptoSession
     */
    public CryptoSession of(T cryptoSessionType) {
        if (cryptoSessionMap.isEmpty())
            throw new CryptoException("CryptoSessionFactory must be initialized first!");
        if (cryptoSessionType == null)
            throw new CryptoException("CryptoSessionType is required!");
        if (!cryptoSessionMap.containsKey(cryptoSessionType))
            throw new CryptoException("There is no CryptoSession instance corresponding to the requested cryptoSessionType!");
        return cryptoSessionMap.get(cryptoSessionType);
    }

    /**
     * CryptoSessionFactory 에 등록된 CryptoSession 객체를 Map 형태로 가져옴
     * @return CryptoSession Map
     */
    public Map<T, CryptoSession> getCryptoSessionMap() {
        if (cryptoSessionMap.isEmpty())
            throw new CryptoException("CryptoSessionFactory must be initialized first!");
        return cryptoSessionMap;
    }
}
