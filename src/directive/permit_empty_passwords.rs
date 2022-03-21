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

#[doc = "When password authentication is allowed, it specifies whether the server allows login to accounts with empty password strings. The default is **no**."]
#[doc = "See also: [PermitEmptyPasswords](https://man.openbsd.org/sshd_config#PermitEmptyPasswords)"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PermitEmptyPasswords {
    #[doc = "yes"]
    Yes,
    #[doc = "no"]
    No,
}

impl<'a> crate::Parse<'a> for PermitEmptyPasswords {
    type Output = PermitEmptyPasswords;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("PermitEmptyPasswords"),
            preceded(
                space1,
                preceded(
                    space0,
                    alt((
                        value(PermitEmptyPasswords::Yes, tag_no_case("yes")),
                        value(PermitEmptyPasswords::No, tag_no_case("no")),
                    )),
                ),
            ),
        )(input)
    }
}

impl<'a> From<PermitEmptyPasswords> for crate::Directive<'a> {
    fn from(directive: PermitEmptyPasswords) -> Self {
        crate::directive::Directive::PermitEmptyPasswords(directive)
    }
}
