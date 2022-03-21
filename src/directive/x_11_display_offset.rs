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

#[doc = "Specifies the first display number available for [sshd(8)](https://man.openbsd.org/sshd.8)'s X11 forwarding. This prevents sshd from interfering with real X11 servers. The default is 10."]
#[doc = "See also: [X11DisplayOffset](https://man.openbsd.org/sshd_config#X11DisplayOffset)"]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct X11DisplayOffset(u64);
impl X11DisplayOffset {
    pub fn new(value: u64) -> Self {
        Self(value)
    }
}
impl From<u64> for X11DisplayOffset {
    fn from(value: u64) -> Self {
        X11DisplayOffset(value)
    }
}

impl<'a> crate::Parse<'a> for X11DisplayOffset {
    type Output = X11DisplayOffset;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("X11DisplayOffset"),
            preceded(
                space1,
                map(
                    preceded(space0, nom::character::complete::u64),
                    X11DisplayOffset::from,
                ),
            ),
        )(input)
    }
}

impl<'a> From<X11DisplayOffset> for crate::Directive<'a> {
    fn from(directive: X11DisplayOffset) -> Self {
        crate::directive::Directive::X11DisplayOffset(directive)
    }
}
