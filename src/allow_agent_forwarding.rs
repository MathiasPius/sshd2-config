//! Generated file, do not edit by hand

use crate::Directive;
use nom::{branch::alt, bytes::complete::tag_no_case, combinator::value, IResult};
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
impl crate::Parse for AllowAgentForwarding {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            value(AllowAgentForwarding::Yes, tag_no_case("yes")),
            value(AllowAgentForwarding::No, tag_no_case("no")),
        ))(input)
    }
}
impl Into<Directive> for AllowAgentForwarding {
    fn into(self) -> Directive {
        Directive::AllowAgentForwarding(self)
    }
}
impl crate::Named for AllowAgentForwarding {
    const OPTION_NAME: &'static str = "AllowAgentForwarding";
}
