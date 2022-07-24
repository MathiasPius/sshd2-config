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

/// Specifies the number of unauthenticated connections allowed from a given source address, or “none” if there is no limit.
///
/// This limit is applied in addition to **MaxStartups**, whichever is lower.
/// The default is **none**.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PerSourceMaxStartups<'a>(Cow<'a, str>);
impl<'a> PerSourceMaxStartups<'a> {
    pub fn new(value: &'a str) -> Self {
        Self(value.into())
    }
}
impl<'a> From<&'a str> for PerSourceMaxStartups<'a> {
    fn from(value: &'a str) -> Self {
        PerSourceMaxStartups(value.into())
    }
}

impl<'a> crate::ParseDirective<'a> for PerSourceMaxStartups<'a> {
    type Output = PerSourceMaxStartups<'a>;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("PerSourceMaxStartups"),
            preceded(
                space1,
                map(
                    preceded(
                        space0,
                        take_while1(|c: char| !c.is_whitespace() && c != '#'),
                    ),
                    PerSourceMaxStartups::from,
                ),
            ),
        )(input)
    }
}

impl<'a> From<PerSourceMaxStartups<'a>> for crate::Directive<'a> {
    fn from(directive: PerSourceMaxStartups<'a>) -> Self {
        crate::directive::Directive::PerSourceMaxStartups(directive)
    }
}
