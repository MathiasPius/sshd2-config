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

#[doc = "Gives the verbosity level that is used when logging messages from [sshd(8)](https://man.openbsd.org/sshd.8). The possible values are: QUIET, FATAL, ERROR, INFO, VERBOSE, DEBUG, DEBUG1, DEBUG2, and DEBUG3. The default is INFO. DEBUG and DEBUG1 are equivalent. DEBUG2 and DEBUG3 each specify higher levels of debugging output. Logging with a DEBUG level violates the privacy of users and is not recommended."]
#[doc = "See also: [LogLevel](https://man.openbsd.org/sshd_config#LogLevel)"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    #[doc = "QUIET"]
    Quiet,
    #[doc = "FATAL"]
    Fatal,
    #[doc = "ERROR"]
    Error,
    #[doc = "INFO"]
    Info,
    #[doc = "VERBOSE"]
    Verbose,
    #[doc = "DEBUG"]
    Debug,
    #[doc = "DEBUG1"]
    Debug1,
    #[doc = "DEBUG2"]
    Debug2,
    #[doc = "DEBUG3"]
    Debug3,
}

impl<'a> crate::Parse<'a> for LogLevel {
    type Output = LogLevel;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("LogLevel"),
            preceded(
                space1,
                preceded(
                    space0,
                    alt((
                        value(LogLevel::Quiet, tag_no_case("QUIET")),
                        value(LogLevel::Fatal, tag_no_case("FATAL")),
                        value(LogLevel::Error, tag_no_case("ERROR")),
                        value(LogLevel::Info, tag_no_case("INFO")),
                        value(LogLevel::Verbose, tag_no_case("VERBOSE")),
                        value(LogLevel::Debug, tag_no_case("DEBUG")),
                        value(LogLevel::Debug1, tag_no_case("DEBUG1")),
                        value(LogLevel::Debug2, tag_no_case("DEBUG2")),
                        value(LogLevel::Debug3, tag_no_case("DEBUG3")),
                    )),
                ),
            ),
        )(input)
    }
}

impl<'a> From<LogLevel> for crate::Directive<'a> {
    fn from(directive: LogLevel) -> Self {
        crate::directive::Directive::LogLevel(directive)
    }
}
