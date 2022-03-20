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
pub struct AllowAgentForwardingDirective(AllowAgentForwarding);
impl<'a> crate::Parse<'a> for AllowAgentForwardingDirective {
    type Output = AllowAgentForwardingDirective;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        map(
            preceded(
                tag_no_case("AllowAgentForwarding"),
                AllowAgentForwarding::parse,
            ),
            |value| AllowAgentForwardingDirective(value),
        )(input)
    }
}
impl<'a> From<AllowAgentForwardingDirective> for Directive<'a> {
    fn from(directive: AllowAgentForwardingDirective) -> Self {
        Directive::AllowAgentForwarding(directive)
    }
}
#[doc = "Specifies whether ssh-agent(1) forwarding is permitted."]
#[doc = "The default is yes.  Note that disabling agent forwarding"]
#[doc = "does not improve security unless users are also denied"]
#[doc = "shell access, as they can always install their own"]
#[doc = "forwarders."]
#[doc = "See also: [sshd_config(5)](https://man7.org/linux/man-pages/man5/sshd_config.5.html)"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AllowAgentForwarding {
    Yes,
    No,
}
impl<'a> crate::Parse<'a> for AllowAgentForwarding {
    type Output = Self;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            space1,
            alt((
                value(AllowAgentForwarding::Yes, tag_no_case("yes")),
                value(AllowAgentForwarding::No, tag_no_case("no")),
            )),
        )(input)
    }
}
