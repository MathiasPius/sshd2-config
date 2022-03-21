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

/// Gives the facility code that is used when logging messages from [sshd(8)](https://man.openbsd.org/sshd.8).
///
/// The possible values are: DAEMON, USER, AUTH, LOCAL0, LOCAL1, LOCAL2, LOCAL3, LOCAL4, LOCAL5, LOCAL6, LOCAL7.
/// The default is AUTH.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SyslogFacility {
    #[doc = "DAEMON"]
    Daemon,
    #[doc = "USER"]
    User,
    #[doc = "AUTH"]
    Auth,
    #[doc = "LOCAL0"]
    Local0,
    #[doc = "LOCAL1"]
    Local1,
    #[doc = "LOCAL2"]
    Local2,
    #[doc = "LOCAL3"]
    Local3,
    #[doc = "LOCAL4"]
    Local4,
    #[doc = "LOCAL5"]
    Local5,
    #[doc = "LOCAL6"]
    Local6,
    #[doc = "LOCAL7"]
    Local7,
}

impl<'a> crate::ParseDirective<'a> for SyslogFacility {
    type Output = SyslogFacility;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("SyslogFacility"),
            preceded(
                space1,
                preceded(
                    space0,
                    alt((
                        value(SyslogFacility::Daemon, tag_no_case("DAEMON")),
                        value(SyslogFacility::User, tag_no_case("USER")),
                        value(SyslogFacility::Auth, tag_no_case("AUTH")),
                        value(SyslogFacility::Local0, tag_no_case("LOCAL0")),
                        value(SyslogFacility::Local1, tag_no_case("LOCAL1")),
                        value(SyslogFacility::Local2, tag_no_case("LOCAL2")),
                        value(SyslogFacility::Local3, tag_no_case("LOCAL3")),
                        value(SyslogFacility::Local4, tag_no_case("LOCAL4")),
                        value(SyslogFacility::Local5, tag_no_case("LOCAL5")),
                        value(SyslogFacility::Local6, tag_no_case("LOCAL6")),
                        value(SyslogFacility::Local7, tag_no_case("LOCAL7")),
                    )),
                ),
            ),
        )(input)
    }
}

impl<'a> From<SyslogFacility> for crate::Directive<'a> {
    fn from(directive: SyslogFacility) -> Self {
        crate::directive::Directive::SyslogFacility(directive)
    }
}
