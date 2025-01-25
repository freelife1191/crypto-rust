#[allow(unused_imports)]
use core::fmt::Display;
use std::io;
use std::num::ParseIntError;
use thiserror::Error;

#[allow(unused_macros)]
macro_rules! unimplemented_display {
    ($ty:ty) => {
        impl Display for $ty {
            fn fmt(&self, _formatter: &mut fmt::Formatter) -> fmt::Result {
                unimplemented!()
            }
        }
    };
}

#[allow(unused)]
#[derive(Error, Debug)]
enum EnumError {
    // #[error("error code : 1000, error : `{0}`")]
    #[error("error code : 1000, error : `{0}`")]
    Braced(io::Error),
}

#[allow(unused)]
#[derive(Error, Debug)]
enum Error {
    #[error("cannot parse integer")]
    CannotParseInteger(#[source] ParseIntError),
    #[error("cannot deserialize JSON")]
    CannotDeserializeJson(#[source] serde_json::Error),
}

#[test]
fn test_braced() {
    let error = EnumError::Braced(io::Error::new(io::ErrorKind::Other, "oh no!"));
    println!("{}", error);
    // assert_eq!(format!("{}", error), "braced error: oh no!");
}

#[test]
fn test_tuple() {
    let error = Error::CannotParseInteger("not a number".parse::<i32>().unwrap_err());
    println!("{}", error);
    // assert_eq!(format!("{}", error), "tuple error: cannot parse integer");
}