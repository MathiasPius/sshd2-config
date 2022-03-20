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

#[doc = "Specifies whether or not the server will attempt to perform a reverse name lookup when matching the name in the `~/.shosts`, `~/.rhosts`, and `/etc/hosts.equiv` files during [`HostbasedAuthentication`]. A setting of **yes** means that sshd(8) uses the name supplied by the client rather than attempting to resolve the name from the TCP connection itself. The default is **no**."]
#[doc = "See also: [HostbasedUsesNameFromPacketOnly](https://man.openbsd.org/sshd_config#HostbasedUsesNameFromPacketOnly)"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HostbasedUsesNameFromPacketOnly {
    #[doc = "yes"]
    Yes,
    #[doc = "no"]
    No,
}

impl<'a> crate::Parse<'a> for HostbasedUsesNameFromPacketOnly {
    type Output = HostbasedUsesNameFromPacketOnly;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("HostbasedUsesNameFromPacketOnly"),
            preceded(
                space1,
                preceded(
                    space0,
                    alt((
                        value(HostbasedUsesNameFromPacketOnly::Yes, tag_no_case("yes")),
                        value(HostbasedUsesNameFromPacketOnly::No, tag_no_case("no")),
                    )),
                ),
            ),
        )(input)
    }
}

impl<'a> From<HostbasedUsesNameFromPacketOnly> for crate::Directive<'a> {
    fn from(directive: HostbasedUsesNameFromPacketOnly) -> Self {
        crate::directive::Directive::HostbasedUsesNameFromPacketOnly(directive)
    }
}
