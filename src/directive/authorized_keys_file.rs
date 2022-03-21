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

#[doc = "Specifies the file that contains the public keys used for user authentication. The format is described in the AUTHORIZED_KEYS FILE FORMAT section of [sshd(8)](https://man.openbsd.org/sshd.8). Arguments to **AuthorizedKeysFile** accept the tokens described in the TOKENS section. After expansion, **AuthorizedKeysFile** is taken to be an absolute path or one relative to the user's home directory. Multiple files may be listed, separated by whitespace. Alternately this option may be set to **none** to skip checking for user keys in files. The default is '.ssh/authorized_keys .ssh/authorized_keys2'."]
#[doc = "See also: [AuthorizedKeysFile](https://man.openbsd.org/sshd_config#AuthorizedKeysFile)"]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AuthorizedKeysFile<'a>(Cow<'a, str>);
impl<'a> AuthorizedKeysFile<'a> {
    pub fn new(value: &'a str) -> Self {
        Self(value.into())
    }
}
impl<'a> From<&'a str> for AuthorizedKeysFile<'a> {
    fn from(value: &'a str) -> Self {
        AuthorizedKeysFile(value.into())
    }
}

impl<'a> crate::Parse<'a> for AuthorizedKeysFile<'a> {
    type Output = AuthorizedKeysFile<'a>;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("AuthorizedKeysFile"),
            preceded(
                space1,
                map(
                    preceded(space0, take_while1(|c: char| !c.is_whitespace())),
                    AuthorizedKeysFile::from,
                ),
            ),
        )(input)
    }
}

impl<'a> From<AuthorizedKeysFile<'a>> for crate::Directive<'a> {
    fn from(directive: AuthorizedKeysFile<'a>) -> Self {
        crate::directive::Directive::AuthorizedKeysFile(directive)
    }
}
