//! Generated file, do not edit by hand

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

#[doc = "Specify one or more overrides to LogLevel. An override consists of a pattern lists that matches the source file, function and line number to force detailed logging for. For example, an override pattern of:"]
#[doc = "> kex.c:*:1000,*:kex_exchange_identification():*,packet.c:*"]
#[doc = ""]
#[doc = "would enable detailed logging for line 1000 of kex.c, everything in the kex_exchange_identification() function, and all code in the packet.c file. This option is intended for debugging and no overrides are enabled by default."]
#[doc = "See also: [LogVerbose](https://man.openbsd.org/sshd_config#LogVerbose)"]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LogVerbose<'a>(Cow<'a, str>);
impl<'a> LogVerbose<'a> {
    pub fn new(value: &'a str) -> Self {
        Self(value.into())
    }
}
impl<'a> From<&'a str> for LogVerbose<'a> {
    fn from(value: &'a str) -> Self {
        LogVerbose(value.into())
    }
}

impl<'a> crate::Parse<'a> for LogVerbose<'a> {
    type Output = LogVerbose<'a>;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("LogVerbose"),
            preceded(
                space1,
                map(
                    preceded(space0, take_while1(|c: char| !c.is_whitespace())),
                    LogVerbose::from,
                ),
            ),
        )(input)
    }
}

impl<'a> From<LogVerbose<'a>> for crate::Directive<'a> {
    fn from(directive: LogVerbose<'a>) -> Self {
        crate::directive::Directive::LogVerbose(directive)
    }
}
