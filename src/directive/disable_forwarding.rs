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

/// Disables all forwarding features, including X11, [ssh-agent(1)](https://man.openbsd.org/ssh-agent.1), TCP and StreamLocal.
///
/// This option overrides all other forwarding-related options and may simplify restricted configurations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DisableForwarding {
    #[doc = "yes"]
    Yes,
    #[doc = "no"]
    No,
}

impl<'a> crate::Parse<'a> for DisableForwarding {
    type Output = DisableForwarding;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("DisableForwarding"),
            preceded(
                space1,
                preceded(
                    space0,
                    alt((
                        value(DisableForwarding::Yes, tag_no_case("yes")),
                        value(DisableForwarding::No, tag_no_case("no")),
                    )),
                ),
            ),
        )(input)
    }
}

impl<'a> From<DisableForwarding> for crate::Directive<'a> {
    fn from(directive: DisableForwarding) -> Self {
        crate::directive::Directive::DisableForwarding(directive)
    }
}

