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

/// This keyword can be followed by a list of user name patterns, separated by spaces.
///
/// If specified, login is allowed only for user names that match one of the patterns.
/// Only user names are valid; a numerical user ID is not recognized.
/// By default, login is allowed for all users.
/// If the pattern takes the form USER@HOST then USER and HOST are separately checked, restricting logins to particular users from particular hosts.
/// HOST criteria may additionally contain addresses to match in CIDR address/masklen format.
/// The allow/deny users directives are processed in the following order: **DenyUsers**, **AllowUsers**.
/// See PATTERNS in [ssh_config(5)](https://man.openbsd.org/ssh_config.5) for more information on patterns.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AllowUsers<'a>(Cow<'a, str>);
impl<'a> AllowUsers<'a> {
    pub fn new(value: &'a str) -> Self {
        Self(value.into())
    }
}
impl<'a> From<&'a str> for AllowUsers<'a> {
    fn from(value: &'a str) -> Self {
        AllowUsers(value.into())
    }
}

impl<'a> crate::Parse<'a> for AllowUsers<'a> {
    type Output = Vec<AllowUsers<'a>>;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("AllowUsers"),
            preceded(
                space1,
                separated_list1(
                    tag(" "),
                    map(
                        preceded(space0, take_while1(|c: char| !c.is_whitespace())),
                        AllowUsers::from,
                    ),
                ),
            ),
        )(input)
    }
}

impl<'a> From<Vec<AllowUsers<'a>>> for crate::Directive<'a> {
    fn from(directive: Vec<AllowUsers<'a>>) -> Self {
        crate::directive::Directive::AllowUsers(directive)
    }
}

