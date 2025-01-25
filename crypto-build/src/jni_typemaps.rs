mod swig_foreign_types_map {}

foreign_typemap!(
    ($p:r_type) &Path => internal_aliases::JStringPath {
        let s = $p.to_str().expect("path is not utf-8");
        let c_str = ::std::ffi::CString::new(s).unwrap();
        $out = unsafe { (**env).NewStringUTF.unwrap()(env, c_str.as_ptr()) };
    };
    ($p:f_type, unique_prefix="/*Path*/") => "/*Path*/String";
);

/*
foreign_typemap!(
    ($p:r_type) <T> Result<T> => swig_i_type!(T) {
        $out = match $p {
            Ok(x) => {
                swig_from_rust_to_i_type!(T, x, ret)
                ret
            }
            Err(err) => {
                let msg = err.to_string();
                let exception_class = match err {
                    CryptoError::Generic(_) => swig_jni_find_class!(Crypto_CryptoException, "com/freelife/crypto/core/CryptoException"),
                    CryptoError::SessionError(_) => swig_jni_find_class!(Crypto_CryptoException, "com/freelife/crypto/core/CryptoException"),
                    CryptoError::ConfigPathError(_) => swig_jni_find_class!(Crypto_CryptoException, "com/freelife/crypto/core/CryptoException"),
                    CryptoError::ConfigParsingError(_) => swig_jni_find_class!(Crypto_CryptoException, "com/freelife/crypto/core/CryptoException"),
                    CryptoError::ConfigFileError(_) => swig_jni_find_class!(Crypto_CryptoException, "com/freelife/crypto/core/CryptoException"),
                    CryptoError::AwsKmsError(_) => swig_jni_find_class!(Crypto_CryptoException, "com/freelife/crypto/core/CryptoException"),
                    CryptoError::AwsKmsInitError(_) => swig_jni_find_class!(Crypto_CryptoException, "com/freelife/crypto/core/CryptoException"),
                    CryptoError::EncryptError(_) => swig_jni_find_class!(Crypto_CryptoException, "com/freelife/crypto/core/CryptoException"),
                    CryptoError::DecryptError(_) => swig_jni_find_class!(Crypto_CryptoException, "com/freelife/crypto/core/CryptoException"),
                    CryptoError::UtilError(_) => swig_jni_find_class!(Crypto_CryptoException, "com/freelife/crypto/core/CryptoException"),
                    _ => swig_jni_find_class!(Crypto_CryptoException, "com/freelife/crypto/core/CryptoException"),
                };
                jni_throw(env, exception_class, &msg);
                return <swig_i_type!(T)>::jni_invalid_value();
            }
        };
    };
    ($p:f_type, unique_prefix="/*error::crypto_error::CryptoResult<swig_subst_type!(T)>*/") => "/*error::crypto_error::CryptoResult<swig_subst_type!(T)>*/swig_f_type!(T)"
        "swig_foreign_from_i_type!(T, $p)";
);
*/

/**
 * This is a test for the `Vec<u8>` type.
 */

// Not sure how I feel about this but:
// https://github.com/Dushistov/flapigen-rs/issues/143#issuecomment-664131615
foreign_typemap!(
    ($p:r_type) Vec<u8> => jbyteArray {
        let slice = &($p)[..];
        let slice = unsafe { std::mem::transmute::<&[u8], &[i8]>(slice) };
        let raw = JavaByteArray::from_slice_to_raw(slice, env);
        $out = raw;
    };
    ($p:f_type) => "jbyteArray";
);

foreign_typemap!(
    ($p:r_type) &'a [u8] => jbyteArray {
        let slice = unsafe { std::mem::transmute::<&[u8], &[i8]>($p) };
        let raw = JavaByteArray::from_slice_to_raw(slice, env);
        $out = raw;
    };
    ($p:f_type) => "jbyteArray";
    ($p:r_type) &'a [u8] <= jbyteArray {
        let arr = JavaByteArray::new(env, $p);
        let slice = arr.to_slice();
        let slice = unsafe { std::mem::transmute::<&[i8], &[u8]>(slice) };
        $out = slice;
    };
    ($p:f_type) <= "jbyteArray";
);


foreign_typemap!(
    ($p:r_type) Option<&'a [u8]> <= jbyteArray {
        $out = if !$p.is_null() {
            let arr = JavaByteArray::new(env, $p);
            let slice = arr.to_slice();
            let slice = unsafe { std::mem::transmute::<&[i8], &[u8]>(slice) };
            Some(slice)
        } else {
            None
        };
    };
    ($p:f_type) <= "jbyteArray";
);