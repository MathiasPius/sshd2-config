//! Generated file, do not edit by hand

use crate::Directive;
use nom::{
    branch::alt, bytes::complete::tag_no_case, character::complete::space1, combinator::value,
    sequence::preceded, IResult,
};
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
impl crate::Parse for AllowTcpForwarding {
    type Output = Self;
    fn parse(input: &str) -> IResult<&str, Self> {
        preceded(
            tag_no_case("AllowTcpForwarding"),
            preceded(
                space1,
                alt((
                    value(AllowTcpForwarding::Yes, tag_no_case("yes")),
                    value(AllowTcpForwarding::No, tag_no_case("no")),
                    value(AllowTcpForwarding::All, tag_no_case("all")),
                    value(AllowTcpForwarding::Local, tag_no_case("local")),
                    value(AllowTcpForwarding::Remote, tag_no_case("remote")),
                )),
            ),
        )(input)
    }
}
impl Into<Directive> for AllowTcpForwarding {
    fn into(self) -> Directive {
        Directive::AllowTcpForwarding(self)
    }
}
