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

#[doc = "Specifies what environment variables sent by the client will be copied into the session's environ(7). See **SendEnv** and **SetEnv** in [ssh_config(5)](https://man.openbsd.org/ssh_config.5#SendEnv) for how to configure the client. The TERM environment variable is always accepted whenever the client requests a pseudo-terminal as it is required by the protocol. Variables are specified by name, which may contain the wildcard characters ‘**’ and ‘?’. Multiple environment variables may be separated by whitespace or spread across multiple AcceptEnv directives. Be warned that some environment variables could be used to bypass restricted user environments. For this reason, care should be taken in the use of this directive. The default is not to accept any environment variables."]
#[doc = "See also: [AcceptEnv](https://man.openbsd.org/sshd_config#AcceptEnv)"]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AcceptEnv<'a>(Cow<'a, str>);
impl<'a> AcceptEnv<'a> {
    pub fn new(value: &'a str) -> Self {
        Self(value.into())
    }
}
impl<'a> From<&'a str> for AcceptEnv<'a> {
    fn from(value: &'a str) -> Self {
        AcceptEnv(value.into())
    }
}

impl<'a> crate::Parse<'a> for AcceptEnv<'a> {
    type Output = Vec<AcceptEnv<'a>>;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("AcceptEnv"),
            preceded(
                space1,
                separated_list1(
                    tag(" "),
                    map(
                        preceded(space0, take_while1(|c: char| !c.is_whitespace())),
                        AcceptEnv::from,
                    ),
                ),
            ),
        )(input)
    }
}

impl<'a> From<Vec<AcceptEnv<'a>>> for crate::Directive<'a> {
    fn from(directive: Vec<AcceptEnv<'a>>) -> Self {
        crate::directive::Directive::AcceptEnv(directive)
    }
}
