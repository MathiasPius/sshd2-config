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

#[doc = "Specifies whether user authentication based on GSSAPI is allowed. The default is **no**."]
#[doc = "See also: [GSSAPIAuthentication](https://man.openbsd.org/sshd_config#GSSAPIAuthentication)"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum GSSAPIAuthentication {
    #[doc = "yes"]
    Yes,
    #[doc = "no"]
    No,
}

impl<'a> crate::Parse<'a> for GSSAPIAuthentication {
    type Output = GSSAPIAuthentication;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("GSSAPIAuthentication"),
            preceded(
                space1,
                preceded(
                    space0,
                    alt((
                        value(GSSAPIAuthentication::Yes, tag_no_case("yes")),
                        value(GSSAPIAuthentication::No, tag_no_case("no")),
                    )),
                ),
            ),
        )(input)
    }
}

impl<'a> From<GSSAPIAuthentication> for crate::Directive<'a> {
    fn from(directive: GSSAPIAuthentication) -> Self {
        crate::directive::Directive::GSSAPIAuthentication(directive)
    }
}
