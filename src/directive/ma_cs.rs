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

/// Specifies the available MAC (message authentication code) algorithms.
///
/// The MAC algorithm is used for data integrity protection.
/// Multiple algorithms must be comma-separated.
/// If the specified list begins with a ‘+’ character, then the specified algorithms will be appended to the default set instead of replacing them.
/// If the specified list begins with a ‘-’ character, then the specified algorithms (including wildcards) will be removed from the default set instead of replacing them.
/// If the specified list begins with a ‘^’ character, then the specified algorithms will be placed at the head of the default set.
/// The algorithms that contain '-etm' calculate the MAC after encryption (encrypt-then-mac).
/// These are considered safer and their use recommended.
/// The supported MACs are:
///
///
///
///
/// * hmac-md5
/// * hmac-md5-96
/// * hmac-sha1
/// * hmac-sha1-96
/// * hmac-sha2-256
/// * hmac-sha2-512
/// * umac-64@openssh.com
/// * umac-128@openssh.com
/// * hmac-md5-etm@openssh.com
/// * hmac-md5-96-etm@openssh.com
/// * hmac-sha1-etm@openssh.com
/// * hmac-sha1-96-etm@openssh.com
/// * hmac-sha2-256-etm@openssh.com
/// * hmac-sha2-512-etm@openssh.com
/// * umac-64-etm@openssh.com
/// * umac-128-etm@openssh.com
///
///
/// The default is:
///
/// > umac-64-etm@openssh.com,umac-128-etm@openssh.com, hmac-sha2-256-etm@openssh.com,hmac-sha2-512-etm@openssh.com, hmac-sha1-etm@openssh.com, umac-64@openssh.com,umac-128@openssh.com, hmac-sha2-256,hmac-sha2-512,hmac-sha1
///
///
/// The list of available MAC algorithms may also be obtained using 'ssh -Q mac'.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MACs {
    #[doc = "hmac-md5"]
    HmacMd5,
    #[doc = "hmac-md5-96"]
    HmacMd596,
    #[doc = "hmac-sha1"]
    HmacSha1,
    #[doc = "hmac-sha1-96"]
    HmacSha196,
    #[doc = "hmac-sha2-256"]
    HmacSha2256,
    #[doc = "hmac-sha2-512"]
    HmacSha2512,
    #[doc = "umac-64@openssh.com"]
    Umac64OpensshCom,
    #[doc = "umac-128@openssh.com"]
    Umac128OpensshCom,
    #[doc = "hmac-md5-etm@openssh.com"]
    HmacMd5EtmOpensshCom,
    #[doc = "hmac-md5-96-etm@openssh.com"]
    HmacMd596EtmOpensshCom,
    #[doc = "hmac-sha1-etm@openssh.com"]
    HmacSha1EtmOpensshCom,
    #[doc = "hmac-sha1-96-etm@openssh.com"]
    HmacSha196EtmOpensshCom,
    #[doc = "hmac-sha2-256-etm@openssh.com"]
    HmacSha2256EtmOpensshCom,
    #[doc = "hmac-sha2-512-etm@openssh.com"]
    HmacSha2512EtmOpensshCom,
    #[doc = "umac-64-etm@openssh.com"]
    Umac64EtmOpensshCom,
    #[doc = "umac-128-etm@openssh.com"]
    Umac128EtmOpensshCom,
}

impl<'a> crate::Parse<'a> for MACs {
    type Output = Modifier<Vec<MACs>>;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("MACs"),
            preceded(
                space1,
                map(
                    tuple((
                        opt(one_of("+-^")),
                        separated_list1(
                            tag(" "),
                            preceded(
                                space0,
                                alt((
                                    value(MACs::HmacMd5, tag_no_case("hmac-md5")),
                                    value(MACs::HmacMd596, tag_no_case("hmac-md5-96")),
                                    value(MACs::HmacSha1, tag_no_case("hmac-sha1")),
                                    value(MACs::HmacSha196, tag_no_case("hmac-sha1-96")),
                                    value(MACs::HmacSha2256, tag_no_case("hmac-sha2-256")),
                                    value(MACs::HmacSha2512, tag_no_case("hmac-sha2-512")),
                                    value(
                                        MACs::Umac64OpensshCom,
                                        tag_no_case("umac-64@openssh.com"),
                                    ),
                                    value(
                                        MACs::Umac128OpensshCom,
                                        tag_no_case("umac-128@openssh.com"),
                                    ),
                                    value(
                                        MACs::HmacMd5EtmOpensshCom,
                                        tag_no_case("hmac-md5-etm@openssh.com"),
                                    ),
                                    value(
                                        MACs::HmacMd596EtmOpensshCom,
                                        tag_no_case("hmac-md5-96-etm@openssh.com"),
                                    ),
                                    value(
                                        MACs::HmacSha1EtmOpensshCom,
                                        tag_no_case("hmac-sha1-etm@openssh.com"),
                                    ),
                                    value(
                                        MACs::HmacSha196EtmOpensshCom,
                                        tag_no_case("hmac-sha1-96-etm@openssh.com"),
                                    ),
                                    value(
                                        MACs::HmacSha2256EtmOpensshCom,
                                        tag_no_case("hmac-sha2-256-etm@openssh.com"),
                                    ),
                                    value(
                                        MACs::HmacSha2512EtmOpensshCom,
                                        tag_no_case("hmac-sha2-512-etm@openssh.com"),
                                    ),
                                    value(
                                        MACs::Umac64EtmOpensshCom,
                                        tag_no_case("umac-64-etm@openssh.com"),
                                    ),
                                    value(
                                        MACs::Umac128EtmOpensshCom,
                                        tag_no_case("umac-128-etm@openssh.com"),
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

impl<'a> From<Modifier<Vec<MACs>>> for crate::Directive<'a> {
    fn from(directive: Modifier<Vec<MACs>>) -> Self {
        crate::directive::Directive::MACs(directive)
    }
}

