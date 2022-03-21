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

/// Specifies whether to remove an existing Unix-domain socket file for local or remote port forwarding before creating a new one.
///
/// If the socket file already exists and **StreamLocalBindUnlink** is not enabled, sshd will be unable to forward the port to the Unix-domain socket file.
/// This option is only used for port forwarding to a Unix-domain socket file.
/// The argument must be **yes** or **no**.
/// The default is **no**.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum StreamLocalBindUnlink {
    #[doc = "yes"]
    Yes,
    #[doc = "no"]
    No,
}

impl<'a> crate::ParseDirective<'a> for StreamLocalBindUnlink {
    type Output = StreamLocalBindUnlink;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("StreamLocalBindUnlink"),
            preceded(
                space1,
                preceded(
                    space0,
                    alt((
                        value(StreamLocalBindUnlink::Yes, tag_no_case("yes")),
                        value(StreamLocalBindUnlink::No, tag_no_case("no")),
                    )),
                ),
            ),
        )(input)
    }
}

impl<'a> From<StreamLocalBindUnlink> for crate::Directive<'a> {
    fn from(directive: StreamLocalBindUnlink) -> Self {
        crate::directive::Directive::StreamLocalBindUnlink(directive)
    }
}
