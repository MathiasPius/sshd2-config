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

/// Specifies the hash algorithm used when logging key fingerprints.
///
/// Valid options are: **md5** and **sha256**.
/// The default is **sha256**.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FingerprintHash {
    #[doc = "md5"]
    Md5,
    #[doc = "sha256"]
    Sha256,
}

impl<'a> crate::ParseDirective<'a> for FingerprintHash {
    type Output = FingerprintHash;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("FingerprintHash"),
            preceded(
                space1,
                preceded(
                    space0,
                    alt((
                        value(FingerprintHash::Md5, tag_no_case("md5")),
                        value(FingerprintHash::Sha256, tag_no_case("sha256")),
                    )),
                ),
            ),
        )(input)
    }
}

impl<'a> From<FingerprintHash> for crate::Directive<'a> {
    fn from(directive: FingerprintHash) -> Self {
        crate::directive::Directive::FingerprintHash(directive)
    }
}
