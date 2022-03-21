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

/// Specifies the ciphers allowed.
///
/// Multiple ciphers must be comma-separated.
/// If the specified list begins with a ‘+’ character, then the specified ciphers will be appended to the default set instead of replacing them.
/// If the specified list begins with a ‘-’ character, then the specified ciphers (including wildcards) will be removed from the default set instead of replacing them.
/// If the specified list begins with a ‘^’ character, then the specified ciphers will be placed at the head of the default set.
/// The supported ciphers are:
///
///
///
///
/// * 3des-cbc
/// * aes128-cbc
/// * aes192-cbc
/// * aes256-cbc
/// * aes128-ctr
/// * aes192-ctr
/// * aes256-ctr
/// * aes128-gcm@openssh.com
/// * aes256-gcm@openssh.com
/// * chacha20-poly1305@openssh.com
///
///
/// The default is:
///
/// > chacha20-poly1305@openssh.com, aes128-ctr,aes192-ctr,aes256-ctr, aes128-gcm@openssh.com,aes256-gcm@openssh.com
///
///
/// The list of available ciphers may also be obtained using 'ssh -Q cipher'.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Ciphers {
    #[doc = "3des-cbc"]
    X3DesCbc,
    #[doc = "aes128-cbc"]
    Aes128Cbc,
    #[doc = "aes192-cbc"]
    Aes192Cbc,
    #[doc = "aes256-cbc"]
    Aes256Cbc,
    #[doc = "aes128-ctr"]
    Aes128Ctr,
    #[doc = "aes192-ctr"]
    Aes192Ctr,
    #[doc = "aes256-ctr"]
    Aes256Ctr,
    #[doc = "aes128-gcm@openssh.com"]
    Aes128GcmOpensshCom,
    #[doc = "aes256-gcm@openssh.com"]
    Aes256GcmOpensshCom,
    #[doc = "chacha20-poly1305@openssh.com"]
    Chacha20Poly1305OpensshCom,
}

impl<'a> crate::ParseDirective<'a> for Ciphers {
    type Output = Modifier<Vec<Ciphers>>;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("Ciphers"),
            preceded(
                space1,
                map(
                    tuple((
                        opt(one_of("+-^")),
                        separated_list1(
                            tag(","),
                            preceded(
                                space0,
                                alt((
                                    value(Ciphers::X3DesCbc, tag_no_case("3des-cbc")),
                                    value(Ciphers::Aes128Cbc, tag_no_case("aes128-cbc")),
                                    value(Ciphers::Aes192Cbc, tag_no_case("aes192-cbc")),
                                    value(Ciphers::Aes256Cbc, tag_no_case("aes256-cbc")),
                                    value(Ciphers::Aes128Ctr, tag_no_case("aes128-ctr")),
                                    value(Ciphers::Aes192Ctr, tag_no_case("aes192-ctr")),
                                    value(Ciphers::Aes256Ctr, tag_no_case("aes256-ctr")),
                                    value(
                                        Ciphers::Aes128GcmOpensshCom,
                                        tag_no_case("aes128-gcm@openssh.com"),
                                    ),
                                    value(
                                        Ciphers::Aes256GcmOpensshCom,
                                        tag_no_case("aes256-gcm@openssh.com"),
                                    ),
                                    value(
                                        Ciphers::Chacha20Poly1305OpensshCom,
                                        tag_no_case("chacha20-poly1305@openssh.com"),
                                    ),
                                )),
                            ),
                        ),
                    )),
                    Modifier::from,
                ),
            ),
        )(input)
    }
}

impl<'a> From<Modifier<Vec<Ciphers>>> for crate::Directive<'a> {
    fn from(directive: Modifier<Vec<Ciphers>>) -> Self {
        crate::directive::Directive::Ciphers(directive)
    }
}
