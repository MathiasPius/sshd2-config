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

/// Specifies whether compression is enabled after the user has authenticated successfully.
///
/// The argument must be **yes**, **delayed** (a legacy synonym for **yes**) or **no**.
/// The default is **yes**.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Compression {
    #[doc = "yes"]
    Yes,
    #[doc = "delayed"]
    Delayed,
    #[doc = "no"]
    No,
}

impl<'a> crate::ParseDirective<'a> for Compression {
    type Output = Compression;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("Compression"),
            preceded(
                space1,
                preceded(
                    space0,
                    alt((
                        value(Compression::Yes, tag_no_case("yes")),
                        value(Compression::Delayed, tag_no_case("delayed")),
                        value(Compression::No, tag_no_case("no")),
                    )),
                ),
            ),
        )(input)
    }
}

impl<'a> From<Compression> for crate::Directive<'a> {
    fn from(directive: Compression) -> Self {
        crate::directive::Directive::Compression(directive)
    }
}
