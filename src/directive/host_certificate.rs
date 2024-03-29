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

/// Specifies a file containing a public host certificate.
///
/// The certificate's public key must match a private host key already specified by **HostKey**.
/// The default behaviour of [sshd(8)](https://man.openbsd.org/sshd.8) is not to load any certificates.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct HostCertificate<'a>(Cow<'a, str>);
impl<'a> HostCertificate<'a> {
    pub fn new(value: &'a str) -> Self {
        Self(value.into())
    }
}
impl<'a> From<&'a str> for HostCertificate<'a> {
    fn from(value: &'a str) -> Self {
        HostCertificate(value.into())
    }
}
impl<'a> AsRef<str> for HostCertificate<'a> {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl<'a> crate::ParseDirective<'a> for HostCertificate<'a> {
    type Output = HostCertificate<'a>;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("HostCertificate"),
            preceded(
                space1,
                map(
                    preceded(
                        space0,
                        take_while1(|c: char| !c.is_whitespace() && c != '#'),
                    ),
                    HostCertificate::from,
                ),
            ),
        )(input)
    }
}

impl<'a> From<HostCertificate<'a>> for crate::Directive<'a> {
    fn from(directive: HostCertificate<'a>) -> Self {
        crate::directive::Directive::HostCertificate(directive)
    }
}
