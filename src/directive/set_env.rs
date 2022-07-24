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

/// Specifies one or more environment variables to set in child sessions started by [sshd(8)](https://man.openbsd.org/sshd.8) as “NAME=VALUE”.
///
/// The environment value may be quoted (e.g.
/// if it contains whitespace characters).
/// Environment variables set by **SetEnv** override the default environment and any variables specified by the user via **AcceptEnv** or **PermitUserEnvironment**.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetEnv<'a>(Cow<'a, str>);
impl<'a> SetEnv<'a> {
    pub fn new(value: &'a str) -> Self {
        Self(value.into())
    }
}
impl<'a> From<&'a str> for SetEnv<'a> {
    fn from(value: &'a str) -> Self {
        SetEnv(value.into())
    }
}

impl<'a> crate::ParseDirective<'a> for SetEnv<'a> {
    type Output = SetEnv<'a>;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("SetEnv"),
            preceded(
                space1,
                map(
                    preceded(
                        space0,
                        take_while1(|c: char| !c.is_whitespace() && c != '#'),
                    ),
                    SetEnv::from,
                ),
            ),
        )(input)
    }
}

impl<'a> From<SetEnv<'a>> for crate::Directive<'a> {
    fn from(directive: SetEnv<'a>) -> Self {
        crate::directive::Directive::SetEnv(directive)
    }
}
