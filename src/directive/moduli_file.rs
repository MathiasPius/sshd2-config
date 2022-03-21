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

/// Specifies the [moduli(5)](https://man.openbsd.org/moduli.5) file that contains the Diffie-Hellman groups used for the “diffie-hellman-group-exchange-sha1” and “diffie-hellman-group-exchange-sha256” key exchange methods.
///
/// The default is /etc/moduli.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ModuliFile<'a>(Cow<'a, str>);
impl<'a> ModuliFile<'a> {
    pub fn new(value: &'a str) -> Self {
        Self(value.into())
    }
}
impl<'a> From<&'a str> for ModuliFile<'a> {
    fn from(value: &'a str) -> Self {
        ModuliFile(value.into())
    }
}

impl<'a> crate::ParseDirective<'a> for ModuliFile<'a> {
    type Output = ModuliFile<'a>;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("ModuliFile"),
            preceded(
                space1,
                map(
                    preceded(space0, take_while1(|c: char| !c.is_whitespace())),
                    ModuliFile::from,
                ),
            ),
        )(input)
    }
}

impl<'a> From<ModuliFile<'a>> for crate::Directive<'a> {
    fn from(directive: ModuliFile<'a>) -> Self {
        crate::directive::Directive::ModuliFile(directive)
    }
}
