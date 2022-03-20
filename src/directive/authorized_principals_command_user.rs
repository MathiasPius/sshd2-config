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

#[doc = "Specifies the user under whose account the [`AuthorizedPrincipalsCommand`] is run. It is recommended to use a dedicated user that has no other role on the host than running authorized principals commands. If [`AuthorizedPrincipalsCommand`] is specified but [`AuthorizedPrincipalsCommandUser`] is not, then [sshd(8)](https://man.openbsd.org/sshd.8) will refuse to start."]
#[doc = "See also: [AuthorizedPrincipalsCommandUser](https://man.openbsd.org/sshd_config#AuthorizedPrincipalsCommandUser)"]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AuthorizedPrincipalsCommandUser<'a>(Cow<'a, str>);
impl<'a> AuthorizedPrincipalsCommandUser<'a> {
    pub fn new(value: &'a str) -> Self {
        Self(value.into())
    }
}
impl<'a> From<&'a str> for AuthorizedPrincipalsCommandUser<'a> {
    fn from(value: &'a str) -> Self {
        AuthorizedPrincipalsCommandUser(value.into())
    }
}

impl<'a> crate::Parse<'a> for AuthorizedPrincipalsCommandUser<'a> {
    type Output = AuthorizedPrincipalsCommandUser<'a>;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("AuthorizedPrincipalsCommandUser"),
            preceded(
                space1,
                map(
                    preceded(space0, take_while1(|c: char| !c.is_whitespace())),
                    AuthorizedPrincipalsCommandUser::from,
                ),
            ),
        )(input)
    }
}

impl<'a> From<AuthorizedPrincipalsCommandUser<'a>> for crate::Directive<'a> {
    fn from(directive: AuthorizedPrincipalsCommandUser<'a>) -> Self {
        crate::directive::Directive::AuthorizedPrincipalsCommandUser(directive)
    }
}
