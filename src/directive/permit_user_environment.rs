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

/// Specifies whether ~/.ssh/environment and **environment=** options in ~/.ssh/authorized_keys are processed by [sshd(8)](https://man.openbsd.org/sshd.8).
///
/// Valid options are **yes**, **no** or a pattern-list specifying which environment variable names to accept (for example 'LANG,LC_*').
/// The default is **no**.
/// Enabling environment processing may enable users to bypass access restrictions in some configurations using mechanisms such as `LD_PRELOAD`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PermitUserEnvironment<'a>(Cow<'a, str>);
impl<'a> PermitUserEnvironment<'a> {
    pub fn new(value: &'a str) -> Self {
        Self(value.into())
    }
}
impl<'a> From<&'a str> for PermitUserEnvironment<'a> {
    fn from(value: &'a str) -> Self {
        PermitUserEnvironment(value.into())
    }
}

impl<'a> crate::Parse<'a> for PermitUserEnvironment<'a> {
    type Output = PermitUserEnvironment<'a>;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("PermitUserEnvironment"),
            preceded(
                space1,
                map(
                    preceded(space0, take_while1(|c: char| !c.is_whitespace())),
                    PermitUserEnvironment::from,
                ),
            ),
        )(input)
    }
}

impl<'a> From<PermitUserEnvironment<'a>> for crate::Directive<'a> {
    fn from(directive: PermitUserEnvironment<'a>) -> Self {
        crate::directive::Directive::PermitUserEnvironment(directive)
    }
}

