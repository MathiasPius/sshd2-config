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

/// Specifies whether public key authentication is allowed.
///
/// The default is **yes**.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PubkeyAuthentication {
    #[doc = "yes"]
    Yes,
    #[doc = "no"]
    No,
}

impl<'a> crate::Parse<'a> for PubkeyAuthentication {
    type Output = PubkeyAuthentication;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("PubkeyAuthentication"),
            preceded(
                space1,
                preceded(
                    space0,
                    alt((
                        value(PubkeyAuthentication::Yes, tag_no_case("yes")),
                        value(PubkeyAuthentication::No, tag_no_case("no")),
                    )),
                ),
            ),
        )(input)
    }
}

impl<'a> From<PubkeyAuthentication> for crate::Directive<'a> {
    fn from(directive: PubkeyAuthentication) -> Self {
        crate::directive::Directive::PubkeyAuthentication(directive)
    }
}

