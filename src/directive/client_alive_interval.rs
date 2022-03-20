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

#[doc = "Sets a timeout interval in seconds after which if no data has been received from the client, [sshd(8)](https://man.openbsd.org/sshd.8) will send a message through the encrypted channel to request a response from the client. The default is 0, indicating that these messages will not be sent to the client."]
#[doc = "See also: [ClientAliveInterval](https://man.openbsd.org/sshd_config#ClientAliveInterval)"]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ClientAliveInterval(u64);
impl ClientAliveInterval {
    pub fn new(value: u64) -> Self {
        Self(value)
    }
}
impl From<u64> for ClientAliveInterval {
    fn from(value: u64) -> Self {
        ClientAliveInterval(value)
    }
}

impl<'a> crate::Parse<'a> for ClientAliveInterval {
    type Output = ClientAliveInterval;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("ClientAliveInterval"),
            preceded(
                space1,
                map(
                    preceded(space0, nom::character::complete::u64),
                    ClientAliveInterval::from,
                ),
            ),
        )(input)
    }
}

impl<'a> From<ClientAliveInterval> for crate::Directive<'a> {
    fn from(directive: ClientAliveInterval) -> Self {
        crate::directive::Directive::ClientAliveInterval(directive)
    }
}
