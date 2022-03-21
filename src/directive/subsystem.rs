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

#[doc = "Configures an external subsystem (e.g. file transfer daemon). Arguments should be a subsystem name and a command (with optional arguments) to execute upon subsystem request. The command **sftp-server** implements the SFTP file transfer subsystem. Alternately the name **internal-sftp** implements an in-process SFTP server. This may simplify configurations using **ChrootDirectory** to force a different filesystem root on clients. By default no subsystems are defined."]
#[doc = "See also: [Subsystem](https://man.openbsd.org/sshd_config#Subsystem)"]
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

impl<'a> crate::Parse<'a> for Subsystem<'a> {
    type Output = Subsystem<'a>;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("Subsystem"),
            preceded(
                space1,
                map(
                    preceded(space0, take_while1(|c: char| !c.is_whitespace())),
                    Subsystem::from,
                ),
            ),
        )(input)
    }
}

impl<'a> From<Subsystem<'a>> for crate::Directive<'a> {
    fn from(directive: Subsystem<'a>) -> Self {
        crate::directive::Directive::Subsystem(directive)
    }
}
