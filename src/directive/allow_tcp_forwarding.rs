//! Generated file, do not edit by hand


#[allow(unused_imports)]
use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_until, take_while1},
    character::complete::{alphanumeric1, space0, space1},
    combinator::{map, not, value},
    multi::{many1, separated_list1},
    sequence::preceded,
    IResult,
};
#[allow(unused_imports)]
use std::borrow::Cow;

#[doc = "Specifies whether TCP forwarding is permitted.  The"]
#[doc = "available options are yes (the default) or all to allow TCP"]
#[doc = "forwarding, no to prevent all TCP forwarding, local to"]
#[doc = "allow local (from the perspective of ssh(1)) forwarding"]
#[doc = "only or remote to allow remote forwarding only.  Note that"]
#[doc = "disabling TCP forwarding does not improve security unless"]
#[doc = "users are also denied shell access, as they can always"]
#[doc = "install their own forwarders."]
#[doc = "See also: [AllowTcpForwarding](https://man.openbsd.org/sshd_config#AllowTcpForwarding)"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AllowTcpForwarding {
    Yes,
    No,
    All,
    Local,
    Remote,
}

impl<'a> crate::Parse<'a> for AllowTcpForwarding {
    type Output = AllowTcpForwarding;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("AllowTcpForwarding"),
            preceded(
                space1,
                preceded(
                    space0,
                    alt((
                        value(AllowTcpForwarding::Yes, tag_no_case("yes")),
                        value(AllowTcpForwarding::No, tag_no_case("no")),
                        value(AllowTcpForwarding::All, tag_no_case("all")),
                        value(AllowTcpForwarding::Local, tag_no_case("local")),
                        value(AllowTcpForwarding::Remote, tag_no_case("remote")),
                    )),
                ),
            ),
        )(input)
    }
}

impl<'a> From<AllowTcpForwarding> for crate::Directive<'a> {
    fn from(directive: AllowTcpForwarding) -> Self {
        crate::Directive::AllowTcpForwarding(directive)
    }
}