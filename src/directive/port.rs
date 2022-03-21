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

/// Specifies the port number that [sshd(8)](https://man.openbsd.org/sshd.8) listens on.
///
/// The default is 22.
/// Multiple options of this type are permitted.
/// See also **ListenAddress**.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Port(u64);
impl Port {
    pub fn new(value: u64) -> Self {
        Self(value)
    }
}
impl From<u64> for Port {
    fn from(value: u64) -> Self {
        Port(value)
    }
}

impl<'a> crate::ParseDirective<'a> for Port {
    type Output = Port;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("Port"),
            preceded(
                space1,
                map(preceded(space0, nom::character::complete::u64), Port::from),
            ),
        )(input)
    }
}

impl<'a> From<Port> for crate::Directive<'a> {
    fn from(directive: Port) -> Self {
        crate::directive::Directive::Port(directive)
    }
}
