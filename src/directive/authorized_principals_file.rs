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

/// Specifies a file that lists principal names that are accepted for certificate authentication.
///
/// When using certificates signed by a key listed in **TrustedUserCAKeys**, this file lists names, one of which must appear in the certificate for it to be accepted for authentication.
/// Names are listed one per line preceded by key options (as described in AUTHORIZED_KEYS FILE FORMAT in [sshd(8)](https://man.openbsd.org/sshd.8)).
/// Empty lines and comments starting with ‘#’ are ignored.
/// Arguments to **AuthorizedPrincipalsFile** accept the tokens described in the TOKENS section.
/// After expansion, **AuthorizedPrincipalsFile** is taken to be an absolute path or one relative to the user's home directory.
/// The default is **none**, i.e.
/// not to use a principals file – in this case, the username of the user must appear in a certificate's principals list for it to be accepted.
///
/// Note that **AuthorizedPrincipalsFile** is only used when authentication proceeds using a CA listed in **TrustedUserCAKeys** and is not consulted for certification authorities trusted via ~/.ssh/authorized_keys, though the **principals=** key option offers a similar facility (see [sshd(8)](https://man.openbsd.org/sshd.8) for details).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AuthorizedPrincipalsFile<'a>(Cow<'a, str>);
impl<'a> AuthorizedPrincipalsFile<'a> {
    pub fn new(value: &'a str) -> Self {
        Self(value.into())
    }
}
impl<'a> From<&'a str> for AuthorizedPrincipalsFile<'a> {
    fn from(value: &'a str) -> Self {
        AuthorizedPrincipalsFile(value.into())
    }
}

impl<'a> crate::Parse<'a> for AuthorizedPrincipalsFile<'a> {
    type Output = AuthorizedPrincipalsFile<'a>;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("AuthorizedPrincipalsFile"),
            preceded(
                space1,
                map(
                    preceded(space0, take_while1(|c: char| !c.is_whitespace())),
                    AuthorizedPrincipalsFile::from,
                ),
            ),
        )(input)
    }
}

impl<'a> From<AuthorizedPrincipalsFile<'a>> for crate::Directive<'a> {
    fn from(directive: AuthorizedPrincipalsFile<'a>) -> Self {
        crate::directive::Directive::AuthorizedPrincipalsFile(directive)
    }
}

