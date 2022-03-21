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

/// Specifies whether the system should send TCP keepalive messages to the other side.
///
/// If they are sent, death of the connection or crash of one of the machines will be properly noticed.
/// However, this means that connections will die if the route is down temporarily, and some people find it annoying.
/// On the other hand, if TCP keepalives are not sent, sessions may hang indefinitely on the server, leaving 'ghost' users and consuming server resources.
/// The default is **yes** (to send TCP keepalive messages), and the server will notice if the network goes down or the client host crashes.
/// This avoids infinitely hanging sessions.
///
/// To disable TCP keepalive messages, the value should be set to **no**.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TCPKeepAlive {
    #[doc = "yes"]
    Yes,
    #[doc = "no"]
    No,
}

impl<'a> crate::Parse<'a> for TCPKeepAlive {
    type Output = TCPKeepAlive;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("TCPKeepAlive"),
            preceded(
                space1,
                preceded(
                    space0,
                    alt((
                        value(TCPKeepAlive::Yes, tag_no_case("yes")),
                        value(TCPKeepAlive::No, tag_no_case("no")),
                    )),
                ),
            ),
        )(input)
    }
}

impl<'a> From<TCPKeepAlive> for crate::Directive<'a> {
    fn from(directive: TCPKeepAlive) -> Self {
        crate::directive::Directive::TCPKeepAlive(directive)
    }
}

