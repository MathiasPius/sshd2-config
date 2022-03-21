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

/// Specifies whether [sshd(8)](https://man.openbsd.org/sshd.8) should ignore the user's ~/.ssh/known_hosts during **HostbasedAuthentication** and use only the system-wide known hosts file /etc/ssh/known_hosts.
///
/// The default is “no”.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum IgnoreUserKnownHosts {
    #[doc = "yes"]
    Yes,
    #[doc = "no"]
    No,
}

impl<'a> crate::Parse<'a> for IgnoreUserKnownHosts {
    type Output = IgnoreUserKnownHosts;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("IgnoreUserKnownHosts"),
            preceded(
                space1,
                preceded(
                    space0,
                    alt((
                        value(IgnoreUserKnownHosts::Yes, tag_no_case("yes")),
                        value(IgnoreUserKnownHosts::No, tag_no_case("no")),
                    )),
                ),
            ),
        )(input)
    }
}

impl<'a> From<IgnoreUserKnownHosts> for crate::Directive<'a> {
    fn from(directive: IgnoreUserKnownHosts) -> Self {
        crate::directive::Directive::IgnoreUserKnownHosts(directive)
    }
}

