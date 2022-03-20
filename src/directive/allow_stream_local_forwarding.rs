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

#[doc = "Specifies whether StreamLocal (Unix-domain socket) forwarding is permitted. The available options are **yes** (the default) or **all** to allow StreamLocal forwarding, **no** to prevent all StreamLocal forwarding, **local** to allow local (from the perspective of [ssh(1)](https://man.openbsd.org/ssh.1)) forwarding only or **remote** to allow remote forwarding only. Note that disabling StreamLocal forwarding does not improve security unless users are also denied shell access, as they can always install their own forwarders."]
#[doc = "See also: [AllowStreamLocalForwarding](https://man.openbsd.org/sshd_config#AllowStreamLocalForwarding)"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AllowStreamLocalForwarding {
    #[doc = "yes"]
    Yes,
    #[doc = "no"]
    No,
    #[doc = "all"]
    All,
    #[doc = "local"]
    Local,
    #[doc = "remote"]
    Remote,
}

impl<'a> crate::Parse<'a> for AllowStreamLocalForwarding {
    type Output = AllowStreamLocalForwarding;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("AllowStreamLocalForwarding"),
            preceded(
                space1,
                preceded(
                    space0,
                    alt((
                        value(AllowStreamLocalForwarding::Yes, tag_no_case("yes")),
                        value(AllowStreamLocalForwarding::No, tag_no_case("no")),
                        value(AllowStreamLocalForwarding::All, tag_no_case("all")),
                        value(AllowStreamLocalForwarding::Local, tag_no_case("local")),
                        value(AllowStreamLocalForwarding::Remote, tag_no_case("remote")),
                    )),
                ),
            ),
        )(input)
    }
}

impl<'a> From<AllowStreamLocalForwarding> for crate::Directive<'a> {
    fn from(directive: AllowStreamLocalForwarding) -> Self {
        crate::directive::Directive::AllowStreamLocalForwarding(directive)
    }
}
