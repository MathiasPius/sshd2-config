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

#[doc = "This keyword can be followed by a list of group name patterns, separated by spaces. If specified, login is allowed only for users whose primary group or supplementary group list matches one of the patterns. Only group names are valid; a numerical group ID is not recognized. By default, login is allowed for all groups. The allow/deny groups directives are processed in the following order: [`DenyGroups`], [`AllowGroups`].\n\nSee PATTERNS in [ssh_config(5)](https://man.openbsd.org/ssh_config.5#PATTERNS) for more information on patterns."]
#[doc = "See also: [AllowGroups](https://man.openbsd.org/sshd_config#AllowGroups)"]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AllowGroups<'a>(Cow<'a, str>);
impl<'a> AllowGroups<'a> {
    pub fn new(value: &'a str) -> Self {
        Self(value.into())
    }
}
impl<'a> From<&'a str> for AllowGroups<'a> {
    fn from(value: &'a str) -> Self {
        AllowGroups(value.into())
    }
}

impl<'a> crate::Parse<'a> for AllowGroups<'a> {
    type Output = Vec<AllowGroups<'a>>;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("AllowGroups"),
            preceded(
                space1,
                separated_list1(
                    tag(" "),
                    map(
                        preceded(space0, take_while1(|c: char| !c.is_whitespace())),
                        AllowGroups::from,
                    ),
                ),
            ),
        )(input)
    }
}

impl<'a> From<Vec<AllowGroups<'a>>> for crate::Directive<'a> {
    fn from(directive: Vec<AllowGroups<'a>>) -> Self {
        crate::directive::Directive::AllowGroups(directive)
    }
}
