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

/// Specifies the file that contains the process ID of the SSH daemon, or **none** to not write one.
///
/// The default is /var/run/sshd.pid.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PidFile<'a>(Cow<'a, str>);
impl<'a> PidFile<'a> {
    pub fn new(value: &'a str) -> Self {
        Self(value.into())
    }
}
impl<'a> From<&'a str> for PidFile<'a> {
    fn from(value: &'a str) -> Self {
        PidFile(value.into())
    }
}

impl<'a> crate::ParseDirective<'a> for PidFile<'a> {
    type Output = PidFile<'a>;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("PidFile"),
            preceded(
                space1,
                map(
                    preceded(space0, take_while1(|c: char| !c.is_whitespace())),
                    PidFile::from,
                ),
            ),
        )(input)
    }
}

impl<'a> From<PidFile<'a>> for crate::Directive<'a> {
    fn from(directive: PidFile<'a>) -> Self {
        crate::directive::Directive::PidFile(directive)
    }
}
