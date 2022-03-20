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
pub struct AllowTcpForwardingDirective(AllowTcpForwarding);
impl<'a> crate::Parse<'a> for AllowTcpForwardingDirective {
    type Output = AllowTcpForwardingDirective;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        map(
            preceded(tag_no_case("AllowTcpForwarding"), AllowTcpForwarding::parse),
            |value| AllowTcpForwardingDirective(value),
        )(input)
    }
}
impl<'a> From<AllowTcpForwardingDirective> for Directive<'a> {
    fn from(directive: AllowTcpForwardingDirective) -> Self {
        Directive::AllowTcpForwarding(directive)
    }
}
#[doc = "Specifies whether TCP forwarding is permitted.  The"]
#[doc = "available options are yes (the default) or all to allow TCP"]
#[doc = "forwarding, no to prevent all TCP forwarding, local to"]
#[doc = "allow local (from the perspective of ssh(1)) forwarding"]
#[doc = "only or remote to allow remote forwarding only.  Note that"]
#[doc = "disabling TCP forwarding does not improve security unless"]
#[doc = "users are also denied shell access, as they can always"]
#[doc = "install their own forwarders."]
#[doc = "See also: [sshd_config(5)](https://man7.org/linux/man-pages/man5/sshd_config.5.html)"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AllowTcpForwarding {
    Yes,
    No,
    All,
    Local,
    Remote,
}
impl<'a> crate::Parse<'a> for AllowTcpForwarding {
    type Output = Self;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            space1,
            alt((
                value(AllowTcpForwarding::Yes, tag_no_case("yes")),
                value(AllowTcpForwarding::No, tag_no_case("no")),
                value(AllowTcpForwarding::All, tag_no_case("all")),
                value(AllowTcpForwarding::Local, tag_no_case("local")),
                value(AllowTcpForwarding::Remote, tag_no_case("remote")),
            )),
        )(input)
    }
}
