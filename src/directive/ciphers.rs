//! Generated file, do not edit by hand


#[allow(unused_imports)]
use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_until, take_while1},
    character::complete::{alphanumeric1, space0, space1},
    combinator::{map, not, value},
    multi::{many1, separated_list1},
    sequence::preceded,
    IResult,
};
#[allow(unused_imports)]
use std::borrow::Cow;

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
#[doc = "See also: [Ciphers](https://man.openbsd.org/sshd_config#Ciphers)"]
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

impl<'a> crate::Parse<'a> for Ciphers {
    type Output = Vec<Ciphers>;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("Ciphers"),
            preceded(
                space1,
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
            ),
        )(input)
    }
}

impl<'a> From<Vec<Ciphers>> for crate::Directive<'a> {
    fn from(directive: Vec<Ciphers>) -> Self {
        crate::Directive::Ciphers(directive)
    }
}
