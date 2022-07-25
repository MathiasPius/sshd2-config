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

/// Specifies the maximum amount of data that may be transmitted before the session key is renegotiated, optionally followed by a maximum amount of time that may pass before the session key is renegotiated.
///
/// The first argument is specified in bytes and may have a suffix of ‘K’, ‘M’, or ‘G’ to indicate Kilobytes, Megabytes, or Gigabytes, respectively.
/// The default is between ‘1G’ and ‘4G’, depending on the cipher.
/// The optional second value is specified in seconds and may use any of the units documented in the TIME FORMATS section.
/// The default value for **RekeyLimit** is **default none**, which means that rekeying is performed after the cipher's default amount of data has been sent or received and no time based rekeying is done.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RekeyLimit<'a>(Cow<'a, str>);
impl<'a> RekeyLimit<'a> {
    pub fn new(value: &'a str) -> Self {
        Self(value.into())
    }
}
impl<'a> From<&'a str> for RekeyLimit<'a> {
    fn from(value: &'a str) -> Self {
        RekeyLimit(value.into())
    }
}
impl<'a> AsRef<str> for RekeyLimit<'a> {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl<'a> crate::ParseDirective<'a> for RekeyLimit<'a> {
    type Output = RekeyLimit<'a>;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("RekeyLimit"),
            preceded(
                space1,
                map(
                    preceded(
                        space0,
                        take_while1(|c: char| !c.is_whitespace() && c != '#'),
                    ),
                    RekeyLimit::from,
                ),
            ),
        )(input)
    }
}

impl<'a> From<RekeyLimit<'a>> for crate::Directive<'a> {
    fn from(directive: RekeyLimit<'a>) -> Self {
        crate::directive::Directive::RekeyLimit(directive)
    }
}
