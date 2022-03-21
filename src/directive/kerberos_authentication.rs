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

#[doc = "Specifies whether the password provided by the user for **PasswordAuthentication** will be validated through the Kerberos KDC. To use this option, the server needs a Kerberos servtab which allows the verification of the KDC's identity. The default is **no**."]
#[doc = "See also: [KerberosAuthentication](https://man.openbsd.org/sshd_config#KerberosAuthentication)"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum KerberosAuthentication {
    #[doc = "yes"]
    Yes,
    #[doc = "no"]
    No,
}

impl<'a> crate::Parse<'a> for KerberosAuthentication {
    type Output = KerberosAuthentication;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("KerberosAuthentication"),
            preceded(
                space1,
                preceded(
                    space0,
                    alt((
                        value(KerberosAuthentication::Yes, tag_no_case("yes")),
                        value(KerberosAuthentication::No, tag_no_case("no")),
                    )),
                ),
            ),
        )(input)
    }
}

impl<'a> From<KerberosAuthentication> for crate::Directive<'a> {
    fn from(directive: KerberosAuthentication) -> Self {
        crate::directive::Directive::KerberosAuthentication(directive)
    }
}
