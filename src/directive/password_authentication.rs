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

/// Specifies whether password authentication is allowed.
///
/// The default is **yes**.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PasswordAuthentication {
    #[doc = "yes"]
    Yes,
    #[doc = "no"]
    No,
}

impl<'a> crate::ParseDirective<'a> for PasswordAuthentication {
    type Output = PasswordAuthentication;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("PasswordAuthentication"),
            preceded(
                space1,
                preceded(
                    space0,
                    alt((
                        value(PasswordAuthentication::Yes, tag_no_case("yes")),
                        value(PasswordAuthentication::No, tag_no_case("no")),
                    )),
                ),
            ),
        )(input)
    }
}

impl<'a> From<PasswordAuthentication> for crate::Directive<'a> {
    fn from(directive: PasswordAuthentication) -> Self {
        crate::directive::Directive::PasswordAuthentication(directive)
    }
}

