//! Generated file, do not edit by hand


#[allow(unused_imports)]
use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_until, take_while1},
    character::complete::{alphanumeric1, space0, space1},
    combinator::{map, not, value},
    multi::{many1, separated_list1},
    sequence::preceded,
    IResult,
};
#[allow(unused_imports)]
use std::borrow::Cow;

#[doc = "Specifies what environment variables sent by the client"]
#[doc = ""]
#[doc = ""]
#[doc = "will be copied into the session's environ(7).  See SendEnv"]
#[doc = ""]
#[doc = ""]
#[doc = "and SetEnv in ssh_config(5) for how to configure the"]
#[doc = ""]
#[doc = ""]
#[doc = "client.  The TERM environment variable is always accepted"]
#[doc = ""]
#[doc = ""]
#[doc = "whenever the client requests a pseudo-terminal as it is"]
#[doc = ""]
#[doc = ""]
#[doc = "required by the protocol.  Variables are specified by name,"]
#[doc = ""]
#[doc = ""]
#[doc = "which may contain the wildcard characters ‘*’ and ‘?’."]
#[doc = ""]
#[doc = ""]
#[doc = "Multiple environment variables may be separated by"]
#[doc = ""]
#[doc = ""]
#[doc = "whitespace or spread across multiple AcceptEnv directives."]
#[doc = ""]
#[doc = ""]
#[doc = "Be warned that some environment variables could be used to"]
#[doc = ""]
#[doc = ""]
#[doc = "bypass restricted user environments.  For this reason, care"]
#[doc = ""]
#[doc = ""]
#[doc = "should be taken in the use of this directive.  The default"]
#[doc = ""]
#[doc = ""]
#[doc = "is not to accept any environment variables."]
#[doc = ""]
#[doc = ""]
#[doc = "See also: [AcceptEnv](https://man.openbsd.org/sshd_config#AcceptEnv)"]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AcceptEnv<'a>(Cow<'a, str>);
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
        crate::Directive::AcceptEnv(directive)
    }
}
