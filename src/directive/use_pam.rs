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

/// Enables the Pluggable Authentication Module interface.
///
/// If set to **yes** this will enable PAM authentication using **ChallengeResponseAuthentication** and **PasswordAuthentication** in addition to PAM account and session module processing for all authentication types.
///
/// Because PAM challenge-response authentication usually serves an equivalent role to password authentication, you should disable either **PasswordAuthentication** or **ChallengeResponseAuthentication**.
///
/// If UsePAM is enabled, you will not be able to run [sshd(8)](https://man.openbsd.org/sshd.8) as a non-root user.
/// The default is **no**.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum UsePAM {
    #[doc = "yes"]
    Yes,
    #[doc = "no"]
    No,
}

impl<'a> crate::ParseDirective<'a> for UsePAM {
    type Output = UsePAM;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("UsePAM"),
            preceded(
                space1,
                preceded(
                    space0,
                    alt((
                        value(UsePAM::Yes, tag_no_case("yes")),
                        value(UsePAM::No, tag_no_case("no")),
                    )),
                ),
            ),
        )(input)
    }
}

impl<'a> From<UsePAM> for crate::Directive<'a> {
    fn from(directive: UsePAM) -> Self {
        crate::directive::Directive::UsePAM(directive)
    }
}
