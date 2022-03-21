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

/// The contents of the specified file are sent to the remote user before authentication is allowed.
///
/// If the argument is **none** then no banner is displayed.
/// By default, no banner is displayed.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Banner<'a>(Cow<'a, str>);
impl<'a> Banner<'a> {
    pub fn new(value: &'a str) -> Self {
        Self(value.into())
    }
}
impl<'a> From<&'a str> for Banner<'a> {
    fn from(value: &'a str) -> Self {
        Banner(value.into())
    }
}

impl<'a> crate::Parse<'a> for Banner<'a> {
    type Output = Banner<'a>;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("Banner"),
            preceded(
                space1,
                map(
                    preceded(space0, take_while1(|c: char| !c.is_whitespace())),
                    Banner::from,
                ),
            ),
        )(input)
    }
}

impl<'a> From<Banner<'a>> for crate::Directive<'a> {
    fn from(directive: Banner<'a>) -> Self {
        crate::directive::Directive::Banner(directive)
    }
}

