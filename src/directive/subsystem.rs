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

/// Configures an external subsystem (e.g.
///
/// file transfer daemon).
/// Arguments should be a subsystem name and a command (with optional arguments) to execute upon subsystem request.
/// The command **sftp-server** implements the SFTP file transfer subsystem.
///
/// Alternately the name **internal-sftp** implements an in-process SFTP server.
/// This may simplify configurations using **ChrootDirectory** to force a different filesystem root on clients.
///
/// By default no subsystems are defined.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Subsystem<'a>(Cow<'a, str>);
impl<'a> Subsystem<'a> {
    pub fn new(value: &'a str) -> Self {
        Self(value.into())
    }
}
impl<'a> From<&'a str> for Subsystem<'a> {
    fn from(value: &'a str) -> Self {
        Subsystem(value.into())
    }
}
impl<'a> AsRef<str> for Subsystem<'a> {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl<'a> crate::ParseDirective<'a> for Subsystem<'a> {
    type Output = Vec<Subsystem<'a>>;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("Subsystem"),
            preceded(
                space1,
                separated_list1(
                    tag(" "),
                    map(
                        preceded(
                            space0,
                            take_while1(|c: char| !c.is_whitespace() && c != '#'),
                        ),
                        Subsystem::from,
                    ),
                ),
            ),
        )(input)
    }
}

impl<'a> From<Vec<Subsystem<'a>>> for crate::Directive<'a> {
    fn from(directive: Vec<Subsystem<'a>>) -> Self {
        crate::directive::Directive::Subsystem(directive)
    }
}
