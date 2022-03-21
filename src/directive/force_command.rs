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

/// Forces the execution of the command specified by **ForceCommand**, ignoring any command supplied by the client and ~/.ssh/rc if present.
///
/// The command is invoked by using the user's login shell with the -c option.
/// This applies to shell, command, or subsystem execution.
/// It is most useful inside a **Match** block.
/// The command originally supplied by the client is available in the `SSH_ORIGINAL_COMMAND` environment variable.
/// Specifying a command of **internal-sftp** will force the use of an in-process SFTP server that requires no support files when used with **ChrootDirectory**.
/// The default is **none**.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ForceCommand<'a>(Cow<'a, str>);
impl<'a> ForceCommand<'a> {
    pub fn new(value: &'a str) -> Self {
        Self(value.into())
    }
}
impl<'a> From<&'a str> for ForceCommand<'a> {
    fn from(value: &'a str) -> Self {
        ForceCommand(value.into())
    }
}

impl<'a> crate::ParseDirective<'a> for ForceCommand<'a> {
    type Output = ForceCommand<'a>;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("ForceCommand"),
            preceded(
                space1,
                map(
                    preceded(space0, take_while1(|c: char| !c.is_whitespace())),
                    ForceCommand::from,
                ),
            ),
        )(input)
    }
}

impl<'a> From<ForceCommand<'a>> for crate::Directive<'a> {
    fn from(directive: ForceCommand<'a>) -> Self {
        crate::directive::Directive::ForceCommand(directive)
    }
}
