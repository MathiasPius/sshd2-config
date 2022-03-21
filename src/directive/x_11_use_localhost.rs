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

/// Specifies whether [sshd(8)](https://man.openbsd.org/sshd.8) should bind the X11 forwarding server to the loopback address or to the wildcard address.
///
/// By default, sshd binds the forwarding server to the loopback address and sets the hostname part of the `DISPLAY` environment variable to **localhost**.
/// This prevents remote hosts from connecting to the proxy display.
/// However, some older X11 clients may not function with this configuration.
/// **X11UseLocalhost** may be set to **no** to specify that the forwarding server should be bound to the wildcard address.
/// The argument must be **yes** or **no**.
/// The default is **yes**.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum X11UseLocalhost {
    #[doc = "yes"]
    Yes,
    #[doc = "no"]
    No,
}

impl<'a> crate::ParseDirective<'a> for X11UseLocalhost {
    type Output = X11UseLocalhost;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("X11UseLocalhost"),
            preceded(
                space1,
                preceded(
                    space0,
                    alt((
                        value(X11UseLocalhost::Yes, tag_no_case("yes")),
                        value(X11UseLocalhost::No, tag_no_case("no")),
                    )),
                ),
            ),
        )(input)
    }
}

impl<'a> From<X11UseLocalhost> for crate::Directive<'a> {
    fn from(directive: X11UseLocalhost) -> Self {
        crate::directive::Directive::X11UseLocalhost(directive)
    }
}
