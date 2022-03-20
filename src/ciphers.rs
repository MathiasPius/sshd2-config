//! Generated file, do not edit by hand

use crate::Directive;
use nom::{branch::alt, bytes::complete::tag_no_case, combinator::value, IResult};
#[doc = "Specifies the ciphers allowed.  Multiple ciphers must be"]
#[doc = "comma-separated.  If the specified list begins with a ‘+’"]
#[doc = "character, then the specified ciphers will be appended to"]
#[doc = "the default set instead of replacing them.  If the"]
#[doc = "specified list begins with a ‘-’ character, then the"]
#[doc = "specified ciphers (including wildcards) will be removed"]
#[doc = "from the default set instead of replacing them.  If the"]
#[doc = "specified list begins with a ‘^’ character, then the"]
#[doc = "specified ciphers will be placed at the head of the default"]
#[doc = "set."]
#[doc = "See also: [sshd_config(5)](https://man7.org/linux/man-pages/man5/sshd_config.5.html)"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Ciphers {
    X3DesCbc,
    Aes128Cbc,
    Aes192Cbc,
    Aes256Cbc,
    Aes128Ctr,
    Aes192Ctr,
    Aes256Ctr,
    Aes128GcmOpensshCom,
    Aes256GcmOpensshCom,
    Chacha20Poly1305OpensshCom,
}
impl crate::Parse for Ciphers {
    fn parse(input: &str) -> IResult<&str, Self> {
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
        ))(input)
    }
}
impl Into<Directive> for Ciphers {
    fn into(self) -> Directive {
        Directive::Ciphers(self)
    }
}
impl crate::Named for Ciphers {
    const OPTION_NAME: &'static str = "Ciphers";
}
