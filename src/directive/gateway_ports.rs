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

#[doc = "Specifies whether remote hosts are allowed to connect to ports forwarded for the client. By default, [sshd(8)](https://man.openbsd.org/sshd.8) binds remote port forwardings to the loopback address. This prevents other remote hosts from connecting to forwarded ports. **GatewayPorts** can be used to specify that sshd should allow remote port forwardings to bind to non-loopback addresses, thus allowing other hosts to connect. The argument may be **no** to force remote port forwardings to be available to the local host only, **yes** to force remote port forwardings to bind to the wildcard address, or **clientspecified** to allow the client to select the address to which the forwarding is bound. The default is **no**."]
#[doc = "See also: [GatewayPorts](https://man.openbsd.org/sshd_config#GatewayPorts)"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum GatewayPorts {
    #[doc = "no"]
    No,
    #[doc = "yes"]
    Yes,
    #[doc = "clientspecified"]
    Clientspecified,
}

impl<'a> crate::Parse<'a> for GatewayPorts {
    type Output = GatewayPorts;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("GatewayPorts"),
            preceded(
                space1,
                preceded(
                    space0,
                    alt((
                        value(GatewayPorts::No, tag_no_case("no")),
                        value(GatewayPorts::Yes, tag_no_case("yes")),
                        value(
                            GatewayPorts::Clientspecified,
                            tag_no_case("clientspecified"),
                        ),
                    )),
                ),
            ),
        )(input)
    }
}

impl<'a> From<GatewayPorts> for crate::Directive<'a> {
    fn from(directive: GatewayPorts) -> Self {
        crate::directive::Directive::GatewayPorts(directive)
    }
}
