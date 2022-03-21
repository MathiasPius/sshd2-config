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

/// Specifies whether root can log in using [ssh(1)](https://man.openbsd.org/ssh.1).
///
/// The argument must be **yes**, **prohibit-password**, **forced-commands-only**, or **no**.
/// The default is **prohibit-password**.
/// If this option is set to **prohibit-password** (or its deprecated alias, **without-password**), password and keyboard-interactive authentication are disabled for root.
///
/// If this option is set to **forced-commands-only**, root login with public key authentication will be allowed, but only if the command option has been specified (which may be useful for taking remote backups even if root login is normally not allowed).
/// All other authentication methods are disabled for root.
///
/// If this option is set to **no**, root is not allowed to log in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PermitRootLogin {
    #[doc = "yes"]
    Yes,
    #[doc = "no"]
    No,
    #[doc = "prohibit-password"]
    ProhibitPassword,
    #[doc = "forced-commands-only"]
    ForcedCommandsOnly,
}

impl<'a> crate::Parse<'a> for PermitRootLogin {
    type Output = PermitRootLogin;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("PermitRootLogin"),
            preceded(
                space1,
                preceded(
                    space0,
                    alt((
                        value(PermitRootLogin::Yes, tag_no_case("yes")),
                        value(PermitRootLogin::No, tag_no_case("no")),
                        value(
                            PermitRootLogin::ProhibitPassword,
                            tag_no_case("prohibit-password"),
                        ),
                        value(
                            PermitRootLogin::ForcedCommandsOnly,
                            tag_no_case("forced-commands-only"),
                        ),
                    )),
                ),
            ),
        )(input)
    }
}

impl<'a> From<PermitRootLogin> for crate::Directive<'a> {
    fn from(directive: PermitRootLogin) -> Self {
        crate::directive::Directive::PermitRootLogin(directive)
    }
}

