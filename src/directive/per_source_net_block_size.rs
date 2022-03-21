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

/// Specifies the number of bits of source address that are grouped together for the purposes of applying PerSourceMaxStartups limits.
///
/// Values for IPv4 and optionally IPv6 may be specified, separated by a colon.
/// The default is **32:128**, which means each address is considered individually.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PerSourceNetBlockSize<'a>(Cow<'a, str>);
impl<'a> PerSourceNetBlockSize<'a> {
    pub fn new(value: &'a str) -> Self {
        Self(value.into())
    }
}
impl<'a> From<&'a str> for PerSourceNetBlockSize<'a> {
    fn from(value: &'a str) -> Self {
        PerSourceNetBlockSize(value.into())
    }
}

impl<'a> crate::Parse<'a> for PerSourceNetBlockSize<'a> {
    type Output = PerSourceNetBlockSize<'a>;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("PerSourceNetBlockSize"),
            preceded(
                space1,
                map(
                    preceded(space0, take_while1(|c: char| !c.is_whitespace())),
                    PerSourceNetBlockSize::from,
                ),
            ),
        )(input)
    }
}

impl<'a> From<PerSourceNetBlockSize<'a>> for crate::Directive<'a> {
    fn from(directive: PerSourceNetBlockSize<'a>) -> Self {
        crate::directive::Directive::PerSourceNetBlockSize(directive)
    }
}

