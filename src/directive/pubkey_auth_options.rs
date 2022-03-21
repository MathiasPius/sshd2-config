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

/// Sets one or more public key authentication options.
///
/// The supported keywords are: **none** (the default; indicating no additional options are enabled), **touch-required** and **verify-required**.
/// The **touch-required** option causes public key authentication using a FIDO authenticator algorithm (i.e.
/// **ecdsa-sk** or **ed25519-sk**) to always require the signature to attest that a physically present user explicitly confirmed the authentication (usually by touching the authenticator).
/// By default, [sshd(8)](https://man.openbsd.org/sshd.8) requires user presence unless overridden with an authorized_keys option.
/// The **touch-required** flag disables this override.
///
/// The **verify-required** option requires a FIDO key signature attest that the user was verified, e.g.
/// via a PIN.
///
/// Neither the **touch-required** or **verify-required** options have any effect for other, non-FIDO, public key types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PubkeyAuthOptions {
    #[doc = "none"]
    None,
    #[doc = "touch-required"]
    TouchRequired,
    #[doc = "verify-required"]
    VerifyRequired,
}

impl<'a> crate::Parse<'a> for PubkeyAuthOptions {
    type Output = Vec<PubkeyAuthOptions>;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("PubkeyAuthOptions"),
            preceded(
                space1,
                separated_list1(
                    tag(" "),
                    preceded(
                        space0,
                        alt((
                            value(PubkeyAuthOptions::None, tag_no_case("none")),
                            value(
                                PubkeyAuthOptions::TouchRequired,
                                tag_no_case("touch-required"),
                            ),
                            value(
                                PubkeyAuthOptions::VerifyRequired,
                                tag_no_case("verify-required"),
                            ),
                        )),
                    ),
                ),
            ),
        )(input)
    }
}

impl<'a> From<Vec<PubkeyAuthOptions>> for crate::Directive<'a> {
    fn from(directive: Vec<PubkeyAuthOptions>) -> Self {
        crate::directive::Directive::PubkeyAuthOptions(directive)
    }
}

