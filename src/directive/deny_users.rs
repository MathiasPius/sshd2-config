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

#[doc = "This keyword can be followed by a list of user name patterns, separated by spaces. Login is disallowed for user names that match one of the patterns. Only user names are valid; a numerical user ID is not recognized. By default, login is allowed for all users. If the pattern takes the form USER@HOST then USER and HOST are separately checked, restricting logins to particular users from particular hosts. HOST criteria may additionally contain addresses to match in CIDR address/masklen format. The allow/deny users directives are processed in the following order: **DenyUsers**, **AllowUsers**. See PATTERNS in [ssh_config(5)](https://man.openbsd.org/ssh_config.5) for more information on patterns."]
#[doc = "See also: [DenyUsers](https://man.openbsd.org/sshd_config#DenyUsers)"]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DenyUsers<'a>(Cow<'a, str>);
impl<'a> DenyUsers<'a> {
    pub fn new(value: &'a str) -> Self {
        Self(value.into())
    }
}
impl<'a> From<&'a str> for DenyUsers<'a> {
    fn from(value: &'a str) -> Self {
        DenyUsers(value.into())
    }
}

impl<'a> crate::Parse<'a> for DenyUsers<'a> {
    type Output = Vec<DenyUsers<'a>>;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("DenyUsers"),
            preceded(
                space1,
                separated_list1(
                    tag(" "),
                    map(
                        preceded(space0, take_while1(|c: char| !c.is_whitespace())),
                        DenyUsers::from,
                    ),
                ),
            ),
        )(input)
    }
}

impl<'a> From<Vec<DenyUsers<'a>>> for crate::Directive<'a> {
    fn from(directive: Vec<DenyUsers<'a>>) -> Self {
        crate::directive::Directive::DenyUsers(directive)
    }
}
