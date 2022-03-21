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

/// Specifies the user under whose account the **AuthorizedKeysCommand** is run.
///
/// It is recommended to use a dedicated user that has no other role on the host than running authorized keys commands.
/// If **AuthorizedKeysCommand** is specified but **AuthorizedKeysCommandUser** is not, then [sshd(8)](https://man.openbsd.org/sshd.8) will refuse to start.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AuthorizedKeysCommandUser<'a>(Cow<'a, str>);
impl<'a> AuthorizedKeysCommandUser<'a> {
    pub fn new(value: &'a str) -> Self {
        Self(value.into())
    }
}
impl<'a> From<&'a str> for AuthorizedKeysCommandUser<'a> {
    fn from(value: &'a str) -> Self {
        AuthorizedKeysCommandUser(value.into())
    }
}

impl<'a> crate::ParseDirective<'a> for AuthorizedKeysCommandUser<'a> {
    type Output = AuthorizedKeysCommandUser<'a>;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("AuthorizedKeysCommandUser"),
            preceded(
                space1,
                map(
                    preceded(space0, take_while1(|c: char| !c.is_whitespace())),
                    AuthorizedKeysCommandUser::from,
                ),
            ),
        )(input)
    }
}

impl<'a> From<AuthorizedKeysCommandUser<'a>> for crate::Directive<'a> {
    fn from(directive: AuthorizedKeysCommandUser<'a>) -> Self {
        crate::directive::Directive::AuthorizedKeysCommandUser(directive)
    }
}
