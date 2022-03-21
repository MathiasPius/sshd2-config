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

#[doc = "Specifies whether [sshd(8)](https://man.openbsd.org/sshd.8) should print the date and time of the last user login when a user logs in interactively. The default is **yes**."]
#[doc = "See also: [PrintLastLog](https://man.openbsd.org/sshd_config#PrintLastLog)"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PrintLastLog {
    #[doc = "yes"]
    Yes,
    #[doc = "no"]
    No,
}

impl<'a> crate::Parse<'a> for PrintLastLog {
    type Output = PrintLastLog;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("PrintLastLog"),
            preceded(
                space1,
                preceded(
                    space0,
                    alt((
                        value(PrintLastLog::Yes, tag_no_case("yes")),
                        value(PrintLastLog::No, tag_no_case("no")),
                    )),
                ),
            ),
        )(input)
    }
}

impl<'a> From<PrintLastLog> for crate::Directive<'a> {
    fn from(directive: PrintLastLog) -> Self {
        crate::directive::Directive::PrintLastLog(directive)
    }
}
