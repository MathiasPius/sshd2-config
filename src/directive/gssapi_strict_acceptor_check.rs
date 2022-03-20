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

#[doc = "Determines whether to be strict about the identity of the GSSAPI acceptor a client authenticates against. If set to **yes** then the client must authenticate against the host service on the current hostname. If set to **no** then the client may authenticate against any service key stored in the machine's default store. This facility is provided to assist with operation on multi homed machines. The default is **yes**."]
#[doc = "See also: [GSSAPIStrictAcceptorCheck](https://man.openbsd.org/sshd_config#GSSAPIStrictAcceptorCheck)"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum GSSAPIStrictAcceptorCheck {
    #[doc = "yes"]
    Yes,
    #[doc = "no"]
    No,
}

impl<'a> crate::Parse<'a> for GSSAPIStrictAcceptorCheck {
    type Output = GSSAPIStrictAcceptorCheck;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("GSSAPIStrictAcceptorCheck"),
            preceded(
                space1,
                preceded(
                    space0,
                    alt((
                        value(GSSAPIStrictAcceptorCheck::Yes, tag_no_case("yes")),
                        value(GSSAPIStrictAcceptorCheck::No, tag_no_case("no")),
                    )),
                ),
            ),
        )(input)
    }
}

impl<'a> From<GSSAPIStrictAcceptorCheck> for crate::Directive<'a> {
    fn from(directive: GSSAPIStrictAcceptorCheck) -> Self {
        crate::directive::Directive::GSSAPIStrictAcceptorCheck(directive)
    }
}
