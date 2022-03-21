//! This file has been automatically generated. Any changes made to it will be overwritten upon subsequent runs!
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

/// Sets the number of client alive messages which may be sent without [sshd(8)](https://man.openbsd.org/sshd.8) receiving any messages back from the client.
///
/// If this threshold is reached while client alive messages are being sent, sshd will disconnect the client, terminating the session.
/// It is important to note that the use of client alive messages is very different from **TCPKeepAlive**.
/// The client alive messages are sent through the encrypted channel and therefore will not be spoofable.
/// The TCP keepalive option enabled by **TCPKeepAlive** is spoofable.
/// The client alive mechanism is valuable when the client or server depend on knowing when a connection has become unresponsive.
/// The default value is 3.
/// If **ClientAliveInterval** is set to 15, and **ClientAliveCountMax** is left at the default, unresponsive SSH clients will be disconnected after approximately 45 seconds.
/// Setting a zero **ClientAliveCountMax** disables connection termination.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ClientAliveCountMax(u64);
impl ClientAliveCountMax {
    pub fn new(value: u64) -> Self {
        Self(value)
    }
}
impl From<u64> for ClientAliveCountMax {
    fn from(value: u64) -> Self {
        ClientAliveCountMax(value)
    }
}

impl<'a> crate::Parse<'a> for ClientAliveCountMax {
    type Output = ClientAliveCountMax;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("ClientAliveCountMax"),
            preceded(
                space1,
                map(
                    preceded(space0, nom::character::complete::u64),
                    ClientAliveCountMax::from,
                ),
            ),
        )(input)
    }
}

impl<'a> From<ClientAliveCountMax> for crate::Directive<'a> {
    fn from(directive: ClientAliveCountMax) -> Self {
        crate::directive::Directive::ClientAliveCountMax(directive)
    }
}

