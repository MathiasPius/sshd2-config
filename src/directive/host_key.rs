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

/// Specifies a file containing a private host key used by SSH.
///
/// The defaults are /etc/ssh/ssh_host_ecdsa_key, /etc/ssh/ssh_host_ed25519_key and /etc/ssh/ssh_host_rsa_key.
/// Note that [sshd(8)](https://man.openbsd.org/sshd.8) will refuse to use a file if it is group/world-accessible and that the **HostKeyAlgorithms** option restricts which of the keys are actually used by [sshd(8)](https://man.openbsd.org/sshd.8).
///
/// It is possible to have multiple host key files.
/// It is also possible to specify public host key files instead.
/// In this case operations on the private key will be delegated to an [ssh-agent(1)](https://man.openbsd.org/ssh-agent.1).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct HostKey<'a>(Cow<'a, str>);
impl<'a> HostKey<'a> {
    pub fn new(value: &'a str) -> Self {
        Self(value.into())
    }
}
impl<'a> From<&'a str> for HostKey<'a> {
    fn from(value: &'a str) -> Self {
        HostKey(value.into())
    }
}
impl<'a> AsRef<str> for HostKey<'a> {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl<'a> crate::ParseDirective<'a> for HostKey<'a> {
    type Output = HostKey<'a>;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("HostKey"),
            preceded(
                space1,
                map(
                    preceded(
                        space0,
                        take_while1(|c: char| !c.is_whitespace() && c != '#'),
                    ),
                    HostKey::from,
                ),
            ),
        )(input)
    }
}

impl<'a> From<HostKey<'a>> for crate::Directive<'a> {
    fn from(directive: HostKey<'a>) -> Self {
        crate::directive::Directive::HostKey(directive)
    }
}
