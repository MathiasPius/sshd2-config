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

#[doc = "Specifies whether [tun(4)](https://man.openbsd.org/tun.4) device forwarding is allowed. The argument must be **yes**, **point-to-point** (layer 3), **ethernet** (layer 2), or **no**. Specifying **yes** permits both **point-to-point** and **ethernet**. The default is **no**. Independent of this setting, the permissions of the selected [tun(4)](https://man.openbsd.org/tun.4) device must allow access to the user."]
#[doc = "See also: [PermitTunnel](https://man.openbsd.org/sshd_config#PermitTunnel)"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PermitTunnel {
    #[doc = "yes"]
    Yes,
    #[doc = "no"]
    No,
    #[doc = "point-to-point"]
    PointToPoint,
    #[doc = "ethernet"]
    Ethernet,
}

impl<'a> crate::Parse<'a> for PermitTunnel {
    type Output = PermitTunnel;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("PermitTunnel"),
            preceded(
                space1,
                preceded(
                    space0,
                    alt((
                        value(PermitTunnel::Yes, tag_no_case("yes")),
                        value(PermitTunnel::No, tag_no_case("no")),
                        value(PermitTunnel::PointToPoint, tag_no_case("point-to-point")),
                        value(PermitTunnel::Ethernet, tag_no_case("ethernet")),
                    )),
                ),
            ),
        )(input)
    }
}

impl<'a> From<PermitTunnel> for crate::Directive<'a> {
    fn from(directive: PermitTunnel) -> Self {
        crate::directive::Directive::PermitTunnel(directive)
    }
}
