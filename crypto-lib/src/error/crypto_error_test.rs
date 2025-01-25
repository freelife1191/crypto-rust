#[allow(unused)]
use crate::error::crypto_error::CryptoError;

#[allow(unused)]
#[test]
fn generic_error_test() {
    CryptoError::Generic("Generic Error".to_string());
}