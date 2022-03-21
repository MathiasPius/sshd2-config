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

/// Writes a temporary file containing a list of authentication methods and public credentials (e.g.
///
/// keys) used to authenticate the user.
/// The location of the file is exposed to the user session through the `SSH_USER_AUTH` environment variable.
/// The default is **no**.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ExposeAuthInfo {
    #[doc = "yes"]
    Yes,
    #[doc = "no"]
    No,
}

impl<'a> crate::ParseDirective<'a> for ExposeAuthInfo {
    type Output = ExposeAuthInfo;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("ExposeAuthInfo"),
            preceded(
                space1,
                preceded(
                    space0,
                    alt((
                        value(ExposeAuthInfo::Yes, tag_no_case("yes")),
                        value(ExposeAuthInfo::No, tag_no_case("no")),
                    )),
                ),
            ),
        )(input)
    }
}

impl<'a> From<ExposeAuthInfo> for crate::Directive<'a> {
    fn from(directive: ExposeAuthInfo) -> Self {
        crate::directive::Directive::ExposeAuthInfo(directive)
    }
}
