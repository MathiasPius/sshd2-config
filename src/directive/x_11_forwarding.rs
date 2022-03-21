//! Generated file, do not edit by hand

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

#[doc = "Specifies whether X11 forwarding is permitted. The argument must be **yes** or **no**. The default is **no**. When X11 forwarding is enabled, there may be additional exposure to the server and to client displays if the [sshd(8)](https://man.openbsd.org/sshd.8) proxy display is configured to listen on the wildcard address (see **X11UseLocalhost**), though this is not the default. Additionally, the authentication spoofing and authentication data verification and substitution occur on the client side. The security risk of using X11 forwarding is that the client's X11 display server may be exposed to attack when the SSH client requests forwarding (see the warnings for **ForwardX11** in [ssh_config(5)](https://man.openbsd.org/ssh_config.5)). A system administrator may have a stance in which they want to protect clients that may expose themselves to attack by unwittingly requesting X11 forwarding, which can warrant a **no** setting. Note that disabling X11 forwarding does not prevent users from forwarding X11 traffic, as users can always install their own forwarders."]
#[doc = "See also: [X11Forwarding](https://man.openbsd.org/sshd_config#X11Forwarding)"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum X11Forwarding {
    #[doc = "yes"]
    Yes,
    #[doc = "no"]
    No,
}

impl<'a> crate::Parse<'a> for X11Forwarding {
    type Output = X11Forwarding;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("X11Forwarding"),
            preceded(
                space1,
                preceded(
                    space0,
                    alt((
                        value(X11Forwarding::Yes, tag_no_case("yes")),
                        value(X11Forwarding::No, tag_no_case("no")),
                    )),
                ),
            ),
        )(input)
    }
}

impl<'a> From<X11Forwarding> for crate::Directive<'a> {
    fn from(directive: X11Forwarding) -> Self {
        crate::directive::Directive::X11Forwarding(directive)
    }
}
