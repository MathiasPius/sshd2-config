//! Generated file, do not edit by hand

use crate::Directive;
use nom::{
    branch::alt, bytes::complete::tag_no_case, character::complete::space1, combinator::value,
    sequence::preceded, IResult,
};
#[doc = "Specifies the available KEX (Key Exchange) algorithms."]
#[doc = "Multiple algorithms must be comma-separated.  Alternately"]
#[doc = "if the specified list begins with a ‘+’ character, then the"]
#[doc = "specified methods will be appended to the default set"]
#[doc = "instead of replacing them.  If the specified list begins"]
#[doc = "with a ‘-’ character, then the specified methods (including"]
#[doc = "wildcards) will be removed from the default set instead of"]
#[doc = "replacing them.  If the specified list begins with a ‘^’"]
#[doc = "character, then the specified methods will be placed at the"]
#[doc = "head of the default set."]
#[doc = "See also: [sshd_config(5)](https://man7.org/linux/man-pages/man5/sshd_config.5.html)"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum KexAlgorithms {
    Curve25519Sha256,
    Curve25519Sha256LibsshOrg,
    DiffieHellmanGroup1Sha1,
    DiffieHellmanGroup14Sha12,
    DiffieHellmanGroup14Sha256,
    DiffieHellmanGroup16Sha512,
    DiffieHellmanGroup18Sha512,
    DiffieHellmanGroupExchangeSha1,
    DiffieHellmanGroupExchangeSha256,
    EcdhSha2Nistp256,
    EcdhSha2Nistp384,
    EcdhSha2Nistp521,
    Sntrup761X25519Sha512OpensshCom,
}
impl crate::Parse for KexAlgorithms {
    type Output = Self;
    fn parse(input: &str) -> IResult<&str, Self> {
        preceded(
            tag_no_case("KexAlgorithms"),
            preceded(
                space1,
                alt((
                    value(
                        KexAlgorithms::Curve25519Sha256,
                        tag_no_case("curve25519-sha256"),
                    ),
                    value(
                        KexAlgorithms::Curve25519Sha256LibsshOrg,
                        tag_no_case("curve25519-sha256@libssh.org"),
                    ),
                    value(
                        KexAlgorithms::DiffieHellmanGroup1Sha1,
                        tag_no_case("diffie-hellman-group1-sha1"),
                    ),
                    value(
                        KexAlgorithms::DiffieHellmanGroup14Sha12,
                        tag_no_case("diffie-hellman-group14-sha12"),
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
                        KexAlgorithms::Sntrup761X25519Sha512OpensshCom,
                        tag_no_case("sntrup761x25519-sha512@openssh.com"),
                    ),
                )),
            ),
        )(input)
    }
}
impl Into<Directive> for KexAlgorithms {
    fn into(self) -> Directive {
        Directive::KexAlgorithms(self)
    }
}
