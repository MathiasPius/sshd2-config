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

/// Specifies the maximum number of concurrent unauthenticated connections to the SSH daemon.
///
/// Additional connections will be dropped until authentication succeeds or the **LoginGraceTime** expires for a connection.
/// The default is 10:30:100.
/// Alternatively, random early drop can be enabled by specifying the three colon separated values start:rate:full (e.g.
/// '10:30:60').
/// [sshd(8)](https://man.openbsd.org/sshd.8) will refuse connection attempts with a probability of rate/100 (30%) if there are currently start (10) unauthenticated connections.
/// The probability increases linearly and all connection attempts are refused if the number of unauthenticated connections reaches full (60).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MaxStartups<'a>(Cow<'a, str>);
impl<'a> MaxStartups<'a> {
    pub fn new(value: &'a str) -> Self {
        Self(value.into())
    }
}
impl<'a> From<&'a str> for MaxStartups<'a> {
    fn from(value: &'a str) -> Self {
        MaxStartups(value.into())
    }
}

impl<'a> crate::ParseDirective<'a> for MaxStartups<'a> {
    type Output = MaxStartups<'a>;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("MaxStartups"),
            preceded(
                space1,
                map(
                    preceded(space0, take_while1(|c: char| !c.is_whitespace())),
                    MaxStartups::from,
                ),
            ),
        )(input)
    }
}

impl<'a> From<MaxStartups<'a>> for crate::Directive<'a> {
    fn from(directive: MaxStartups<'a>) -> Self {
        crate::directive::Directive::MaxStartups(directive)
    }
}
