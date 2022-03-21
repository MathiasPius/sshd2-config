//! This file has been automatically generated. Any changes made to it will be overwritten upon subsequent runs!
#[allow(unused_imports)]
use crate::Modifier;
#[allow(unused_imports)]
use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_until, take_while1},
    character::complete::{alphanumeric1, one_of, space0, space1},
    combinator::{into, map, not, opt, value},
    multi::{many1, separated_list1},
    sequence::{preceded, tuple},
    IResult,
};
#[allow(unused_imports)]
use std::borrow::Cow;

/// Specifies the maximum number of authentication attempts permitted per connection.
///
/// Once the number of failures reaches half this value, additional failures are logged.
/// The default is 6.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MaxAuthTries(u64);
impl MaxAuthTries {
    pub fn new(value: u64) -> Self {
        Self(value)
    }
}
impl From<u64> for MaxAuthTries {
    fn from(value: u64) -> Self {
        MaxAuthTries(value)
    }
}

impl<'a> crate::Parse<'a> for MaxAuthTries {
    type Output = MaxAuthTries;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("MaxAuthTries"),
            preceded(
                space1,
                map(
                    preceded(space0, nom::character::complete::u64),
                    MaxAuthTries::from,
                ),
            ),
        )(input)
    }
}

impl<'a> From<MaxAuthTries> for crate::Directive<'a> {
    fn from(directive: MaxAuthTries) -> Self {
        crate::directive::Directive::MaxAuthTries(directive)
    }
}

