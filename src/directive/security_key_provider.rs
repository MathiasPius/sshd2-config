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

/// Specifies a path to a library that will be used when loading FIDO authenticator-hosted keys, overriding the default of using the built-in USB HID support.
///
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SecurityKeyProvider<'a>(Cow<'a, str>);
impl<'a> SecurityKeyProvider<'a> {
    pub fn new(value: &'a str) -> Self {
        Self(value.into())
    }
}
impl<'a> From<&'a str> for SecurityKeyProvider<'a> {
    fn from(value: &'a str) -> Self {
        SecurityKeyProvider(value.into())
    }
}

impl<'a> crate::ParseDirective<'a> for SecurityKeyProvider<'a> {
    type Output = SecurityKeyProvider<'a>;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("SecurityKeyProvider"),
            preceded(
                space1,
                map(
                    preceded(space0, take_while1(|c: char| !c.is_whitespace())),
                    SecurityKeyProvider::from,
                ),
            ),
        )(input)
    }
}

impl<'a> From<SecurityKeyProvider<'a>> for crate::Directive<'a> {
    fn from(directive: SecurityKeyProvider<'a>) -> Self {
        crate::directive::Directive::SecurityKeyProvider(directive)
    }
}
