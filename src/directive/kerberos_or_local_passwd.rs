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

/// If password authentication through Kerberos fails then the password will be validated via any additional local mechanism such as /etc/passwd.
///
/// The default is **yes**.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum KerberosOrLocalPasswd {
    #[doc = "yes"]
    Yes,
    #[doc = "no"]
    No,
}

impl<'a> crate::Parse<'a> for KerberosOrLocalPasswd {
    type Output = KerberosOrLocalPasswd;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("KerberosOrLocalPasswd"),
            preceded(
                space1,
                preceded(
                    space0,
                    alt((
                        value(KerberosOrLocalPasswd::Yes, tag_no_case("yes")),
                        value(KerberosOrLocalPasswd::No, tag_no_case("no")),
                    )),
                ),
            ),
        )(input)
    }
}

impl<'a> From<KerberosOrLocalPasswd> for crate::Directive<'a> {
    fn from(directive: KerberosOrLocalPasswd) -> Self {
        crate::directive::Directive::KerberosOrLocalPasswd(directive)
    }
}

