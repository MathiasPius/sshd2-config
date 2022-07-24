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

/// Specifies the destinations to which TCP port forwarding is permitted.
///
/// The forwarding specification must be one of the following forms:
///
///
///
/// * **PermitOpen** host:port
/// * **PermitOpen** IPv4_addr:port
/// * **PermitOpen** [IPv6_addr]:port
///
///
/// Multiple forwards may be specified by separating them with whitespace.
/// An argument of **any** can be used to remove all restrictions and permit any forwarding requests.
/// An argument of **none** can be used to prohibit all forwarding requests.
/// The wildcard ‘*’ can be used for host or port to allow all hosts or ports respectively.
/// Otherwise, no pattern matching or address lookups are performed on supplied names.
/// By default all port forwarding requests are permitted.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PermitOpen<'a>(Cow<'a, str>);
impl<'a> PermitOpen<'a> {
    pub fn new(value: &'a str) -> Self {
        Self(value.into())
    }
}
impl<'a> From<&'a str> for PermitOpen<'a> {
    fn from(value: &'a str) -> Self {
        PermitOpen(value.into())
    }
}

impl<'a> crate::ParseDirective<'a> for PermitOpen<'a> {
    type Output = PermitOpen<'a>;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("PermitOpen"),
            preceded(
                space1,
                map(
                    preceded(
                        space0,
                        take_while1(|c: char| !c.is_whitespace() && c != '#'),
                    ),
                    PermitOpen::from,
                ),
            ),
        )(input)
    }
}

impl<'a> From<PermitOpen<'a>> for crate::Directive<'a> {
    fn from(directive: PermitOpen<'a>) -> Self {
        crate::directive::Directive::PermitOpen(directive)
    }
}
