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

#[doc = "If AFS is active and the user has a Kerberos 5 TGT, attempt to acquire an AFS token before accessing the user's home directory. The default is **no**."]
#[doc = "See also: [KerberosGetAFSToken](https://man.openbsd.org/sshd_config#KerberosGetAFSToken)"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum KerberosGetAFSToken {
    #[doc = "yes"]
    Yes,
    #[doc = "no"]
    No,
}

impl<'a> crate::Parse<'a> for KerberosGetAFSToken {
    type Output = KerberosGetAFSToken;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("KerberosGetAFSToken"),
            preceded(
                space1,
                preceded(
                    space0,
                    alt((
                        value(KerberosGetAFSToken::Yes, tag_no_case("yes")),
                        value(KerberosGetAFSToken::No, tag_no_case("no")),
                    )),
                ),
            ),
        )(input)
    }
}

impl<'a> From<KerberosGetAFSToken> for crate::Directive<'a> {
    fn from(directive: KerberosGetAFSToken) -> Self {
        crate::directive::Directive::KerberosGetAFSToken(directive)
    }
}
