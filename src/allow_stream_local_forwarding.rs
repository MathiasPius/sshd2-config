//! Generated file, do not edit by hand

#[allow(unused_imports)]
use crate::Directive;
#[allow(unused_imports)]
use nom::{
    branch::alt,
    bytes::complete::{tag_no_case, take_until, take_while1},
    character::complete::{alphanumeric1, space0, space1},
    combinator::{map, not, value},
    multi::many1,
    sequence::preceded,
    IResult,
};
#[allow(unused_imports)]
use std::borrow::Cow;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AllowStreamLocalForwardingDirective(AllowStreamLocalForwarding);
impl<'a> crate::Parse<'a> for AllowStreamLocalForwardingDirective {
    type Output = AllowStreamLocalForwardingDirective;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        map(
            preceded(
                tag_no_case("AllowStreamLocalForwarding"),
                AllowStreamLocalForwarding::parse,
            ),
            |value| AllowStreamLocalForwardingDirective(value),
        )(input)
    }
}
impl<'a> From<AllowStreamLocalForwardingDirective> for Directive<'a> {
    fn from(directive: AllowStreamLocalForwardingDirective) -> Self {
        Directive::AllowStreamLocalForwarding(directive)
    }
}
#[doc = "Specifies whether StreamLocal (Unix-domain socket)"]
#[doc = "forwarding is permitted.  The available options are yes"]
#[doc = "(the default) or all to allow StreamLocal forwarding, no to"]
#[doc = "prevent all StreamLocal forwarding, local to allow local"]
#[doc = "(from the perspective of ssh(1)) forwarding only or remote"]
#[doc = "to allow remote forwarding only.  Note that disabling"]
#[doc = "StreamLocal forwarding does not improve security unless"]
#[doc = "users are also denied shell access, as they can always"]
#[doc = "install their own forwarders."]
#[doc = "See also: [sshd_config(5)](https://man7.org/linux/man-pages/man5/sshd_config.5.html)"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AllowStreamLocalForwarding {
    Yes,
    No,
    All,
    Local,
    Remote,
}
impl<'a> crate::Parse<'a> for AllowStreamLocalForwarding {
    type Output = Self;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            space1,
            alt((
                value(AllowStreamLocalForwarding::Yes, tag_no_case("yes")),
                value(AllowStreamLocalForwarding::No, tag_no_case("no")),
                value(AllowStreamLocalForwarding::All, tag_no_case("all")),
                value(AllowStreamLocalForwarding::Local, tag_no_case("local")),
                value(AllowStreamLocalForwarding::Remote, tag_no_case("remote")),
            )),
        )(input)
    }
}
