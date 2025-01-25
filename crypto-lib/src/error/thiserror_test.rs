#![allow(clippy::needless_raw_string_hashes, clippy::uninlined_format_args)]

use core::fmt::Display;
#[allow(unused)]
use thiserror::Error;

#[allow(unused)]
fn assert<T: Display>(expected: &str, value: T) {
    assert_eq!(expected, value.to_string());
}

#[test]
fn test_braced() {
    #[derive(Error, Debug)]
    #[error("braced error: {msg}")]
    struct Error {
        msg: String,
    }

    let msg = "T".to_owned();
    println!("{}", Error { msg: msg.clone() });
    assert("braced error: T", Error { msg });
}

#[test]
fn test_braced_unused() {
    #[derive(Error, Debug)]
    #[error("braced error")]
    struct Error {
        extra: usize,
    }

    println!("{}", Error { extra: 2 });
    assert("braced error", Error { extra: 0 });
}


#[test]
fn test_tuple() {
    #[derive(Error, Debug)]
    #[error("tuple error: {0}")]
    struct Error(usize);

    println!("{}", Error(2));
    assert("tuple error: 0", Error(0));
}

#[test]
fn test_enum() {
    #[derive(Error, Debug)]
    enum Error {
        #[error("braced error: {id}")]
        Braced { id: usize },
        #[error("tuple error: {0}")]
        Tuple(usize),
        #[error("unit error")]
        Unit,
    }

    assert("braced error: 0", Error::Braced { id: 0 });
    assert("tuple error: 0", Error::Tuple(0));
    assert("unit error", Error::Unit);
}