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

/// Specifies a file containing public keys of certificate authorities that are trusted to sign user certificates for authentication, or **none** to not use one.
///
/// Keys are listed one per line; empty lines and comments starting with ‘#’ are allowed.
/// If a certificate is presented for authentication and has its signing CA key listed in this file, then it may be used for authentication for any user listed in the certificate's principals list.
/// Note that certificates that lack a list of principals will not be permitted for authentication using **TrustedUserCAKeys**.
/// For more details on certificates, see the CERTIFICATES section in [ssh-keygen(1)](https://man.openbsd.org/ssh-keygen.1).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TrustedUserCAKeys<'a>(Cow<'a, str>);
impl<'a> TrustedUserCAKeys<'a> {
    pub fn new(value: &'a str) -> Self {
        Self(value.into())
    }
}
impl<'a> From<&'a str> for TrustedUserCAKeys<'a> {
    fn from(value: &'a str) -> Self {
        TrustedUserCAKeys(value.into())
    }
}
impl<'a> AsRef<str> for TrustedUserCAKeys<'a> {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl<'a> crate::ParseDirective<'a> for TrustedUserCAKeys<'a> {
    type Output = TrustedUserCAKeys<'a>;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("TrustedUserCAKeys"),
            preceded(
                space1,
                map(
                    preceded(
                        space0,
                        take_while1(|c: char| !c.is_whitespace() && c != '#'),
                    ),
                    TrustedUserCAKeys::from,
                ),
            ),
        )(input)
    }
}

impl<'a> From<TrustedUserCAKeys<'a>> for crate::Directive<'a> {
    fn from(directive: TrustedUserCAKeys<'a>) -> Self {
        crate::directive::Directive::TrustedUserCAKeys(directive)
    }
}
