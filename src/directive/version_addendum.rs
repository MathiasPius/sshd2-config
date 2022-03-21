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

/// Optionally specifies additional text to append to the SSH protocol banner sent by the server upon connection.
///
/// The default is **none**.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct VersionAddendum<'a>(Cow<'a, str>);
impl<'a> VersionAddendum<'a> {
    pub fn new(value: &'a str) -> Self {
        Self(value.into())
    }
}
impl<'a> From<&'a str> for VersionAddendum<'a> {
    fn from(value: &'a str) -> Self {
        VersionAddendum(value.into())
    }
}

impl<'a> crate::Parse<'a> for VersionAddendum<'a> {
    type Output = VersionAddendum<'a>;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("VersionAddendum"),
            preceded(
                space1,
                map(
                    preceded(space0, take_while1(|c: char| !c.is_whitespace())),
                    VersionAddendum::from,
                ),
            ),
        )(input)
    }
}

impl<'a> From<VersionAddendum<'a>> for crate::Directive<'a> {
    fn from(directive: VersionAddendum<'a>) -> Self {
        crate::directive::Directive::VersionAddendum(directive)
    }
}

