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

#[doc = "Specifies what environment variables sent by the client"]
#[doc = "will be copied into the session's environ(7). See SendEnv"]
#[doc = "and SetEnv in ssh_config(5) for how to configure the client."]
#[doc = "The TERM environment variable is always accepted whenever"]
#[doc = "the client requests a pseudo-terminal as it is required by"]
#[doc = "the protocol. Variables are specified by name, which may "]
#[doc = "contain the wildcard characters ‘*’ and ‘?’. Multiple"]
#[doc = "environment variables may be separated by whitespace or"]
#[doc = "spread across multiple AcceptEnv directives. Be warned"]
#[doc = "that some environment variables could be used to bypass"]
#[doc = "restricted user environments. For this reason, care should"]
#[doc = "be taken in the use of this directive. The default is not "]
#[doc = "to accept any environment variables."]
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
