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

/// Specifies whether to ignore per-user .rhosts and .shosts files during **HostbasedAuthentication**.
///
/// The system-wide /etc/hosts.equiv and /etc/shosts.equiv are still used regardless of this setting.
/// Accepted values are **yes** (the default) to ignore all per-user files, **shosts-only** to allow the use of .shosts but to ignore .rhosts or **no** to allow both .shosts and rhosts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum IgnoreRhosts {
    #[doc = "yes"]
    Yes,
    #[doc = "no"]
    No,
    #[doc = "shosts-only"]
    ShostsOnly,
}

impl<'a> crate::Parse<'a> for IgnoreRhosts {
    type Output = IgnoreRhosts;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("IgnoreRhosts"),
            preceded(
                space1,
                preceded(
                    space0,
                    alt((
                        value(IgnoreRhosts::Yes, tag_no_case("yes")),
                        value(IgnoreRhosts::No, tag_no_case("no")),
                        value(IgnoreRhosts::ShostsOnly, tag_no_case("shosts-only")),
                    )),
                ),
            ),
        )(input)
    }
}

impl<'a> From<IgnoreRhosts> for crate::Directive<'a> {
    fn from(directive: IgnoreRhosts) -> Self {
        crate::directive::Directive::IgnoreRhosts(directive)
    }
}

