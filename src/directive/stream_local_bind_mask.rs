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

/// Sets the octal file creation mode mask (umask) used when creating a Unix-domain socket file for local or remote port forwarding.
///
/// This option is only used for port forwarding to a Unix-domain socket file.
/// The default value is 0177, which creates a Unix-domain socket file that is readable and writable only by the owner.
/// Note that not all operating systems honor the file mode on Unix-domain socket files.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StreamLocalBindMask<'a>(Cow<'a, str>);
impl<'a> StreamLocalBindMask<'a> {
    pub fn new(value: &'a str) -> Self {
        Self(value.into())
    }
}
impl<'a> From<&'a str> for StreamLocalBindMask<'a> {
    fn from(value: &'a str) -> Self {
        StreamLocalBindMask(value.into())
    }
}

impl<'a> crate::ParseDirective<'a> for StreamLocalBindMask<'a> {
    type Output = StreamLocalBindMask<'a>;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("StreamLocalBindMask"),
            preceded(
                space1,
                map(
                    preceded(
                        space0,
                        take_while1(|c: char| !c.is_whitespace() && c != '#'),
                    ),
                    StreamLocalBindMask::from,
                ),
            ),
        )(input)
    }
}

impl<'a> From<StreamLocalBindMask<'a>> for crate::Directive<'a> {
    fn from(directive: StreamLocalBindMask<'a>) -> Self {
        crate::directive::Directive::StreamLocalBindMask(directive)
    }
}
