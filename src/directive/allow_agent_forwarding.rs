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

#[doc = "Specifies whether ssh-agent(1) forwarding is permitted."]
#[doc = "The default is yes.  Note that disabling agent forwarding"]
#[doc = "does not improve security unless users are also denied"]
#[doc = "shell access, as they can always install their own"]
#[doc = "forwarders."]
#[doc = "See also: [AllowAgentForwarding](https://man.openbsd.org/sshd_config#AllowAgentForwarding)"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AllowAgentForwarding {
    Yes,
    No,
}

impl<'a> crate::Parse<'a> for AllowAgentForwarding {
    type Output = AllowAgentForwarding;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("AllowAgentForwarding"),
            preceded(
                space1,
                preceded(
                    space0,
                    alt((
                        value(AllowAgentForwarding::Yes, tag_no_case("yes")),
                        value(AllowAgentForwarding::No, tag_no_case("no")),
                    )),
                ),
            ),
        )(input)
    }
}

impl<'a> From<AllowAgentForwarding> for crate::Directive<'a> {
    fn from(directive: AllowAgentForwarding) -> Self {
        crate::Directive::AllowAgentForwarding(directive)
    }
}