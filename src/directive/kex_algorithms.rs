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

#[doc = "Specifies the available KEX (Key Exchange) algorithms. Multiple algorithms must be comma-separated. Alternately if the specified list begins with a ‘+’ character, then the specified algorithms will be appended to the default set instead of replacing them. If the specified list begins with a ‘-’ character, then the specified algorithms (including wildcards) will be removed from the default set instead of replacing them. If the specified list begins with a ‘^’ character, then the specified algorithms will be placed at the head of the default set. The supported algorithms are:"]
#[doc = ""]
#[doc = "* curve25519-sha256"]
#[doc = "* curve25519-sha256@libssh.org"]
#[doc = "* diffie-hellman-group1-sha1"]
#[doc = "* diffie-hellman-group14-sha1"]
#[doc = "* diffie-hellman-group14-sha256"]
#[doc = "* diffie-hellman-group16-sha512"]
#[doc = "* diffie-hellman-group18-sha512"]
#[doc = "* diffie-hellman-group-exchange-sha1"]
#[doc = "* diffie-hellman-group-exchange-sha256"]
#[doc = "* ecdh-sha2-nistp256"]
#[doc = "* ecdh-sha2-nistp384"]
#[doc = "* ecdh-sha2-nistp521"]
#[doc = "* sntrup761x25519-sha512@openssh.com"]
#[doc = ""]
#[doc = "The default is:"]
#[doc = "> curve25519-sha256,curve25519-sha256@libssh.org, ecdh-sha2-nistp256,ecdh-sha2-nistp384,ecdh-sha2-nistp521, sntrup761x25519-sha512@openssh.com, diffie-hellman-group-exchange-sha256, diffie-hellman-group16-sha512,diffie-hellman-group18-sha512, diffie-hellman-group14-sha256"]
#[doc = ""]
#[doc = "The list of available key exchange algorithms may also be obtained using 'ssh -Q KexAlgorithms'."]
#[doc = "See also: [KexAlgorithms](https://man.openbsd.org/sshd_config#KexAlgorithms)"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum KexAlgorithms {
    #[doc = "diffie-hellman-group1-sha1"]
    DiffieHellmanGroup1Sha1,
    #[doc = "diffie-hellman-group14-sha1"]
    DiffieHellmanGroup14Sha1,
    #[doc = "diffie-hellman-group14-sha256"]
    DiffieHellmanGroup14Sha256,
    #[doc = "diffie-hellman-group16-sha512"]
    DiffieHellmanGroup16Sha512,
    #[doc = "diffie-hellman-group18-sha512"]
    DiffieHellmanGroup18Sha512,
    #[doc = "diffie-hellman-group-exchange-sha1"]
    DiffieHellmanGroupExchangeSha1,
    #[doc = "diffie-hellman-group-exchange-sha256"]
    DiffieHellmanGroupExchangeSha256,
    #[doc = "ecdh-sha2-nistp256"]
    EcdhSha2Nistp256,
    #[doc = "ecdh-sha2-nistp384"]
    EcdhSha2Nistp384,
    #[doc = "ecdh-sha2-nistp521"]
    EcdhSha2Nistp521,
    #[doc = "curve25519-sha256"]
    Curve25519Sha256,
    #[doc = "curve25519-sha256@libssh.org"]
    Curve25519Sha256LibsshOrg,
    #[doc = "sntrup761x25519-sha512@openssh.com"]
    Sntrup761X25519Sha512OpensshCom,
}

impl<'a> crate::Parse<'a> for KexAlgorithms {
    type Output = Modifier<Vec<KexAlgorithms>>;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("KexAlgorithms"),
            preceded(
                space1,
                map(
                    tuple((
                        opt(one_of("+-")),
                        separated_list1(
                            tag(","),
                            preceded(
                                space0,
                                alt((
                                    value(
                                        KexAlgorithms::DiffieHellmanGroup1Sha1,
                                        tag_no_case("diffie-hellman-group1-sha1"),
                                    ),
                                    value(
                                        KexAlgorithms::DiffieHellmanGroup14Sha1,
                                        tag_no_case("diffie-hellman-group14-sha1"),
                                    ),
                                    value(
                                        KexAlgorithms::DiffieHellmanGroup14Sha256,
                                        tag_no_case("diffie-hellman-group14-sha256"),
                                    ),
                                    value(
                                        KexAlgorithms::DiffieHellmanGroup16Sha512,
                                        tag_no_case("diffie-hellman-group16-sha512"),
                                    ),
                                    value(
                                        KexAlgorithms::DiffieHellmanGroup18Sha512,
                                        tag_no_case("diffie-hellman-group18-sha512"),
                                    ),
                                    value(
                                        KexAlgorithms::DiffieHellmanGroupExchangeSha1,
                                        tag_no_case("diffie-hellman-group-exchange-sha1"),
                                    ),
                                    value(
                                        KexAlgorithms::DiffieHellmanGroupExchangeSha256,
                                        tag_no_case("diffie-hellman-group-exchange-sha256"),
                                    ),
                                    value(
                                        KexAlgorithms::EcdhSha2Nistp256,
                                        tag_no_case("ecdh-sha2-nistp256"),
                                    ),
                                    value(
                                        KexAlgorithms::EcdhSha2Nistp384,
                                        tag_no_case("ecdh-sha2-nistp384"),
                                    ),
                                    value(
                                        KexAlgorithms::EcdhSha2Nistp521,
                                        tag_no_case("ecdh-sha2-nistp521"),
                                    ),
                                    value(
                                        KexAlgorithms::Curve25519Sha256,
                                        tag_no_case("curve25519-sha256"),
                                    ),
                                    value(
                                        KexAlgorithms::Curve25519Sha256LibsshOrg,
                                        tag_no_case("curve25519-sha256@libssh.org"),
                                    ),
                                    value(
                                        KexAlgorithms::Sntrup761X25519Sha512OpensshCom,
                                        tag_no_case("sntrup761x25519-sha512@openssh.com"),
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

impl<'a> From<Modifier<Vec<KexAlgorithms>>> for crate::Directive<'a> {
    fn from(directive: Modifier<Vec<KexAlgorithms>>) -> Self {
        crate::directive::Directive::KexAlgorithms(directive)
    }
}
