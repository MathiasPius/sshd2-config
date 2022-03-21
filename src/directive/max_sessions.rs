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

#[doc = "Specifies the maximum number of open shell, login or subsystem (e.g. sftp) sessions permitted per network connection. Multiple sessions may be established by clients that support connection multiplexing. Setting **MaxSessions** to 1 will effectively disable session multiplexing, whereas setting it to 0 will prevent all shell, login and subsystem sessions while still permitting forwarding. The default is 10."]
#[doc = "See also: [MaxSessions](https://man.openbsd.org/sshd_config#MaxSessions)"]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MaxSessions(u64);
impl MaxSessions {
    pub fn new(value: u64) -> Self {
        Self(value)
    }
}
impl From<u64> for MaxSessions {
    fn from(value: u64) -> Self {
        MaxSessions(value)
    }
}

impl<'a> crate::Parse<'a> for MaxSessions {
    type Output = MaxSessions;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("MaxSessions"),
            preceded(
                space1,
                map(
                    preceded(space0, nom::character::complete::u64),
                    MaxSessions::from,
                ),
            ),
        )(input)
    }
}

impl<'a> From<MaxSessions> for crate::Directive<'a> {
    fn from(directive: MaxSessions) -> Self {
        crate::directive::Directive::MaxSessions(directive)
    }
}
