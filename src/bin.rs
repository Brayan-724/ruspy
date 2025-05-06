use core::fmt;
use std::env;
use std::str::FromStr;

pub fn required_first_arg<T>() -> T
where
    T: FromStr,
    T::Err: fmt::Debug,
{
    T::from_str(&env::args().nth(1).expect("Requires at least one argument"))
        .expect("Cannot parse argument")
}
