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

/// This keyword can be followed by a list of group name patterns, separated by spaces.
///
/// Login is disallowed for users whose primary group or supplementary group list matches one of the patterns.
/// Only group names are valid; a numerical group ID is not recognized.
/// By default, login is allowed for all groups.
/// The allow/deny groups directives are processed in the following order: **DenyGroups**, **AllowGroups**.
/// See PATTERNS in [ssh_config(5)](https://man.openbsd.org/ssh_config.5) for more information on patterns.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DenyGroups<'a>(Cow<'a, str>);
impl<'a> DenyGroups<'a> {
    pub fn new(value: &'a str) -> Self {
        Self(value.into())
    }
}
impl<'a> From<&'a str> for DenyGroups<'a> {
    fn from(value: &'a str) -> Self {
        DenyGroups(value.into())
    }
}
impl<'a> AsRef<str> for DenyGroups<'a> {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl<'a> crate::ParseDirective<'a> for DenyGroups<'a> {
    type Output = Vec<DenyGroups<'a>>;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("DenyGroups"),
            preceded(
                space1,
                separated_list1(
                    tag(" "),
                    map(
                        preceded(
                            space0,
                            take_while1(|c: char| !c.is_whitespace() && c != '#'),
                        ),
                        DenyGroups::from,
                    ),
                ),
            ),
        )(input)
    }
}

impl<'a> From<Vec<DenyGroups<'a>>> for crate::Directive<'a> {
    fn from(directive: Vec<DenyGroups<'a>>) -> Self {
        crate::directive::Directive::DenyGroups(directive)
    }
}
