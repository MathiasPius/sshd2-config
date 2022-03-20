//! Generated file, do not edit by hand

#[allow(unused_imports)]
use crate::Directive;
#[allow(unused_imports)]
use nom::{
    branch::alt,
    bytes::complete::{tag_no_case, take_until, take_while1},
    character::complete::{alphanumeric1, space0, space1},
    combinator::{map, not, value},
    multi::many1,
    sequence::preceded,
    IResult,
};
#[allow(unused_imports)]
use std::borrow::Cow;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AcceptEnvDirective<'a>(Vec<AcceptEnv<'a>>);
impl<'a> crate::Parse<'a> for AcceptEnvDirective<'a> {
    type Output = AcceptEnvDirective<'a>;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        map(
            preceded(tag_no_case("AcceptEnv"), many1(AcceptEnv::parse)),
            |value| AcceptEnvDirective(value),
        )(input)
    }
}
impl<'a> From<AcceptEnvDirective<'a>> for Directive<'a> {
    fn from(directive: AcceptEnvDirective<'a>) -> Self {
        Directive::AcceptEnv(directive)
    }
}
#[doc = "Specifies what environment variables sent by the client"]
#[doc = "will be copied into the session's environ(7).  See SendEnv"]
#[doc = "and SetEnv in ssh_config(5) for how to configure the"]
#[doc = "client.  The TERM environment variable is always accepted"]
#[doc = "whenever the client requests a pseudo-terminal as it is"]
#[doc = "required by the protocol.  Variables are specified by name,"]
#[doc = "which may contain the wildcard characters ‘*’ and ‘?’."]
#[doc = "Multiple environment variables may be separated by"]
#[doc = "whitespace or spread across multiple AcceptEnv directives."]
#[doc = "Be warned that some environment variables could be used to"]
#[doc = "bypass restricted user environments.  For this reason, care"]
#[doc = "should be taken in the use of this directive.  The default"]
#[doc = "is not to accept any environment variables."]
#[doc = "See also: [sshd_config(5)](https://man7.org/linux/man-pages/man5/sshd_config.5.html)"]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AcceptEnv<'a>(Cow<'a, str>);
impl<'a> crate::Parse<'a> for AcceptEnv<'a> {
    type Output = Self;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        map(
            preceded(space1, take_while1(|c: char| !c.is_whitespace())),
            |value: &'a str| AcceptEnv(value.into()),
        )(input)
    }
}
