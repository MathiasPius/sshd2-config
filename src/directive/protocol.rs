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

/// The SSH protocol version.
///
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Protocol(u64);
impl Protocol {
    pub fn new(value: u64) -> Self {
        Self(value)
    }
}
impl From<u64> for Protocol {
    fn from(value: u64) -> Self {
        Protocol(value)
    }
}

impl<'a> crate::Parse<'a> for Protocol {
    type Output = Protocol;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("Protocol"),
            preceded(
                space1,
                map(
                    preceded(space0, nom::character::complete::u64),
                    Protocol::from,
                ),
            ),
        )(input)
    }
}

impl<'a> From<Protocol> for crate::Directive<'a> {
    fn from(directive: Protocol) -> Self {
        crate::directive::Directive::Protocol(directive)
    }
}

