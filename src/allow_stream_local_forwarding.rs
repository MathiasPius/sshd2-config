//! Generated file, do not edit by hand

use crate::Directive;
use nom::{
    branch::alt, bytes::complete::tag_no_case, character::complete::space1, combinator::value,
    sequence::preceded, IResult,
};
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
impl crate::Parse for AllowStreamLocalForwarding {
    type Output = Self;
    fn parse(input: &str) -> IResult<&str, Self> {
        preceded(
            tag_no_case("AllowStreamLocalForwarding"),
            preceded(
                space1,
                alt((
                    value(AllowStreamLocalForwarding::Yes, tag_no_case("yes")),
                    value(AllowStreamLocalForwarding::No, tag_no_case("no")),
                    value(AllowStreamLocalForwarding::All, tag_no_case("all")),
                    value(AllowStreamLocalForwarding::Local, tag_no_case("local")),
                    value(AllowStreamLocalForwarding::Remote, tag_no_case("remote")),
                )),
            ),
        )(input)
    }
}
impl Into<Directive> for AllowStreamLocalForwarding {
    fn into(self) -> Directive {
        Directive::AllowStreamLocalForwarding(self)
    }
}
