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

/// Specifies the full pathname of the [xauth(1)](https://man.openbsd.org/xauth.1) program, or **none** to not use one.
///
/// The default is /usr/X11R6/bin/xauth.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct XAuthLocation<'a>(Cow<'a, str>);
impl<'a> XAuthLocation<'a> {
    pub fn new(value: &'a str) -> Self {
        Self(value.into())
    }
}
impl<'a> From<&'a str> for XAuthLocation<'a> {
    fn from(value: &'a str) -> Self {
        XAuthLocation(value.into())
    }
}

impl<'a> crate::ParseDirective<'a> for XAuthLocation<'a> {
    type Output = XAuthLocation<'a>;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("XAuthLocation"),
            preceded(
                space1,
                map(
                    preceded(space0, take_while1(|c: char| !c.is_whitespace())),
                    XAuthLocation::from,
                ),
            ),
        )(input)
    }
}

impl<'a> From<XAuthLocation<'a>> for crate::Directive<'a> {
    fn from(directive: XAuthLocation<'a>) -> Self {
        crate::directive::Directive::XAuthLocation(directive)
    }
}
