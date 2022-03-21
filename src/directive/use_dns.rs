//! This file has been automatically generated. Any changes made to it will be overwritten upon subsequent runs!
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

/// Specifies whether [sshd(8)](https://man.openbsd.org/sshd.8) should look up the remote host name, and to check that the resolved host name for the remote IP address maps back to the very same IP address.
///
/// If this option is set to **no** (the default) then only addresses and not host names may be used in ~/.ssh/authorized_keys **from** and sshd_config **Match** **Host** directives.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum UseDNS {
    #[doc = "yes"]
    Yes,
    #[doc = "no"]
    No,
}

impl<'a> crate::ParseDirective<'a> for UseDNS {
    type Output = UseDNS;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("UseDNS"),
            preceded(
                space1,
                preceded(
                    space0,
                    alt((
                        value(UseDNS::Yes, tag_no_case("yes")),
                        value(UseDNS::No, tag_no_case("no")),
                    )),
                ),
            ),
        )(input)
    }
}

impl<'a> From<UseDNS> for crate::Directive<'a> {
    fn from(directive: UseDNS) -> Self {
        crate::directive::Directive::UseDNS(directive)
    }
}
