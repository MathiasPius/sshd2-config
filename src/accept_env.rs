//! Generated file, do not edit by hand

#[allow(unused_imports)]
use crate::Directive;
#[allow(unused_imports)]
use nom::{
    branch::alt,
    bytes::complete::tag_no_case,
    character::complete::{alphanumeric1, space0, space1},
    combinator::{map, value},
    multi::many1,
    sequence::preceded,
    IResult,
};
#[allow(unused_imports)]
use std::borrow::Cow;
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
        map(preceded(space1, alphanumeric1), |value: &'a str| {
            AcceptEnv(value.into())
        })(input)
    }
}
