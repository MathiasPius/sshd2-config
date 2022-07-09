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

/// Include the specified configuration file(s).
///
/// Multiple pathnames may be specified and each pathname may contain [glob(7)](https://man.openbsd.org/glob.7) wildcards that will be expanded and processed in lexical order.
/// Files without absolute paths are assumed to be in /etc/ssh.
/// An **Include** directive may appear inside a **Match** block to perform conditional inclusion.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Include<'a>(Cow<'a, str>);
impl<'a> Include<'a> {
    pub fn new(value: &'a str) -> Self {
        Self(value.into())
    }
}
impl<'a> From<&'a str> for Include<'a> {
    fn from(value: &'a str) -> Self {
        Include(value.into())
    }
}

impl<'a> crate::ParseDirective<'a> for Include<'a> {
    type Output = Vec<Include<'a>>;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("Include"),
            preceded(
                space1,
                separated_list1(
                    tag(" "),
                    map(
                        preceded(space0, take_while1(|c: char| !c.is_whitespace())),
                        Include::from,
                    ),
                ),
            ),
        )(input)
    }
}

impl<'a> From<Vec<Include<'a>>> for crate::Directive<'a> {
    fn from(directive: Vec<Include<'a>>) -> Self {
        crate::directive::Directive::Include(directive)
    }
}
