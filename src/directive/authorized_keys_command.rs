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

#[doc = "Specifies a program to be used to look up the user's public keys. The program must be owned by root, not writable by group or others and specified by an absolute path. Arguments to **AuthorizedKeysCommand** accept the tokens described in the TOKENS section. If no arguments are specified then the username of the target user is used. The program should produce on standard output zero or more lines of authorized_keys output (see AUTHORIZED_KEYS in [sshd(8)](https://man.openbsd.org/sshd.8)). **AuthorizedKeysCommand** is tried after the usual **AuthorizedKeysFile** files and will not be executed if a matching key is found there. By default, no **AuthorizedKeysCommand** is run."]
#[doc = "See also: [AuthorizedKeysCommand](https://man.openbsd.org/sshd_config#AuthorizedKeysCommand)"]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AuthorizedKeysCommand<'a>(Cow<'a, str>);
impl<'a> AuthorizedKeysCommand<'a> {
    pub fn new(value: &'a str) -> Self {
        Self(value.into())
    }
}
impl<'a> From<&'a str> for AuthorizedKeysCommand<'a> {
    fn from(value: &'a str) -> Self {
        AuthorizedKeysCommand(value.into())
    }
}

impl<'a> crate::Parse<'a> for AuthorizedKeysCommand<'a> {
    type Output = AuthorizedKeysCommand<'a>;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("AuthorizedKeysCommand"),
            preceded(
                space1,
                map(
                    preceded(space0, take_while1(|c: char| !c.is_whitespace())),
                    AuthorizedKeysCommand::from,
                ),
            ),
        )(input)
    }
}

impl<'a> From<AuthorizedKeysCommand<'a>> for crate::Directive<'a> {
    fn from(directive: AuthorizedKeysCommand<'a>) -> Self {
        crate::directive::Directive::AuthorizedKeysCommand(directive)
    }
}
