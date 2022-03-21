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

/// Specifies the addresses/ports on which a remote TCP port forwarding may listen.
///
/// The listen specification must be one of the following forms:
///
///
///
/// * **PermitListen** port
/// * **PermitListen** host:port
///
///
/// Multiple permissions may be specified by separating them with whitespace.
/// An argument of **any** can be used to remove all restrictions and permit any listen requests.
/// An argument of **none** can be used to prohibit all listen requests.
/// The host name may contain wildcards as described in the PATTERNS section in [ssh_config(5)](https://man.openbsd.org/ssh_config.5).
/// The wildcard ‘*’ can also be used in place of a port number to allow all ports.
/// By default all port forwarding listen requests are permitted.
/// Note that the **GatewayPorts** option may further restrict which addresses may be listened on.
/// Note also that [ssh(1)](https://man.openbsd.org/ssh.1) will request a listen host of “localhost” if no listen host was specifically requested, and this name is treated differently to explicit localhost addresses of “127.0.0.1” and “::1”.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PermitListen<'a>(Cow<'a, str>);
impl<'a> PermitListen<'a> {
    pub fn new(value: &'a str) -> Self {
        Self(value.into())
    }
}
impl<'a> From<&'a str> for PermitListen<'a> {
    fn from(value: &'a str) -> Self {
        PermitListen(value.into())
    }
}

impl<'a> crate::ParseDirective<'a> for PermitListen<'a> {
    type Output = PermitListen<'a>;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("PermitListen"),
            preceded(
                space1,
                map(
                    preceded(space0, take_while1(|c: char| !c.is_whitespace())),
                    PermitListen::from,
                ),
            ),
        )(input)
    }
}

impl<'a> From<PermitListen<'a>> for crate::Directive<'a> {
    fn from(directive: PermitListen<'a>) -> Self {
        crate::directive::Directive::PermitListen(directive)
    }
}
