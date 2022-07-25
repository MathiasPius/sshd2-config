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

/// Identifies the UNIX-domain socket used to communicate with an agent that has access to the private host keys.
///
/// If the string 'SSH_AUTH_SOCK' is specified, the location of the socket will be read from the `SSH_AUTH_SOCK` environment variable.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct HostKeyAgent<'a>(Cow<'a, str>);
impl<'a> HostKeyAgent<'a> {
    pub fn new(value: &'a str) -> Self {
        Self(value.into())
    }
}
impl<'a> From<&'a str> for HostKeyAgent<'a> {
    fn from(value: &'a str) -> Self {
        HostKeyAgent(value.into())
    }
}
impl<'a> AsRef<str> for HostKeyAgent<'a> {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl<'a> crate::ParseDirective<'a> for HostKeyAgent<'a> {
    type Output = HostKeyAgent<'a>;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("HostKeyAgent"),
            preceded(
                space1,
                map(
                    preceded(
                        space0,
                        take_while1(|c: char| !c.is_whitespace() && c != '#'),
                    ),
                    HostKeyAgent::from,
                ),
            ),
        )(input)
    }
}

impl<'a> From<HostKeyAgent<'a>> for crate::Directive<'a> {
    fn from(directive: HostKeyAgent<'a>) -> Self {
        crate::directive::Directive::HostKeyAgent(directive)
    }
}
