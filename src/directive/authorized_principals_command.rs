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

#[doc = "Specifies a program to be used to generate the list of allowed certificate principals as per **AuthorizedPrincipalsFile**. The program must be owned by root, not writable by group or others and specified by an absolute path. Arguments to **AuthorizedPrincipalsCommand** accept the tokens described in the TOKENS section. If no arguments are specified then the username of the target user is used. The program should produce on standard output zero or more lines of **AuthorizedPrincipalsFile** output. If either **AuthorizedPrincipalsCommand** or **AuthorizedPrincipalsFile** is specified, then certificates offered by the client for authentication must contain a principal that is listed. By default, no **AuthorizedPrincipalsCommand** is run."]
#[doc = "See also: [AuthorizedPrincipalsCommand](https://man.openbsd.org/sshd_config#AuthorizedPrincipalsCommand)"]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AuthorizedPrincipalsCommand<'a>(Cow<'a, str>);
impl<'a> AuthorizedPrincipalsCommand<'a> {
    pub fn new(value: &'a str) -> Self {
        Self(value.into())
    }
}
impl<'a> From<&'a str> for AuthorizedPrincipalsCommand<'a> {
    fn from(value: &'a str) -> Self {
        AuthorizedPrincipalsCommand(value.into())
    }
}

impl<'a> crate::Parse<'a> for AuthorizedPrincipalsCommand<'a> {
    type Output = AuthorizedPrincipalsCommand<'a>;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("AuthorizedPrincipalsCommand"),
            preceded(
                space1,
                map(
                    preceded(space0, take_while1(|c: char| !c.is_whitespace())),
                    AuthorizedPrincipalsCommand::from,
                ),
            ),
        )(input)
    }
}

impl<'a> From<AuthorizedPrincipalsCommand<'a>> for crate::Directive<'a> {
    fn from(directive: AuthorizedPrincipalsCommand<'a>) -> Self {
        crate::directive::Directive::AuthorizedPrincipalsCommand(directive)
    }
}
