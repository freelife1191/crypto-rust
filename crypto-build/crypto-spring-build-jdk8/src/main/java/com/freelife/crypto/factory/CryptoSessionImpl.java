package com.freelife.crypto.factory;

import com.freelife.crypto.core.CryptoException;
import com.freelife.crypto.core.CryptoSession;

/**
 * 단일 CryptoSession Singleton 인스턴스를 생성하는 클래스
 * 무조건 Singleton 인스턴스를 보장하는 클래스
 * Created by mskwon on 2024. 10. 16..
 */
public class CryptoSessionImpl<T extends Enum<T>> {

    private static volatile CryptoSession CRYPTO_SESSION_INSTANCE = null;

    /**
     * CryptoSessionInit 로 CryptoSession Config 구성 객체를 생성하고 초기화 객체를 사용하여
     * CryptoSession Singleton 인스턴스를 생성
     * @param cryptoSessionInit getInstance 인스턴스를 생성하기 위한 객체
     * @return CryptoSession instance
     */
    public static <T extends Enum<T>> CryptoSession getInstance(CryptoSessionInit<T> cryptoSessionInit) {
        if (cryptoSessionInit == null)
            throw new CryptoException("CryptoSessionInit is required!");
        return GetCryptoSession(cryptoSessionInit);
    }

    /**
     * Singleton CryptoSession 인스턴스를 생성
     * @param cryptoSessionInit CryptoSession 인스턴스를 생성하기 위한 객체
     * @return CryptoSession instance
     */
    private static <T extends Enum<T>> CryptoSession GetCryptoSession(CryptoSessionInit<T> cryptoSessionInit) {
        if (CRYPTO_SESSION_INSTANCE == null) {
            synchronized (CryptoSession.class) {
                if (CRYPTO_SESSION_INSTANCE == null) {
                    try {
                        initialize(cryptoSessionInit);
                    } catch (Exception e) {
                        throw new CryptoException(e);
                    }
                }
            }
        }
        return CRYPTO_SESSION_INSTANCE;
    }

    /**
     * InitType 별로 CryptoSession 인스턴스를 초기화
     * @param cryptoSessionInit CryptoSessionInit
     */
    private static <T extends Enum<T>> void initialize(CryptoSessionInit<T> cryptoSessionInit) {
        if (cryptoSessionInit == null) {
            throw new CryptoException("CryptoSessionInit is required!");
        }

        CRYPTO_SESSION_INSTANCE = CryptoSessionInit.initCryptoSession(cryptoSessionInit);

        if (CRYPTO_SESSION_INSTANCE == null)
            throw new CryptoException("Config file loading failed");
    }

}
