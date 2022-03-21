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

#[doc = "Specifies an explicit routing domain that is applied after authentication has completed. The user session, as well as any forwarded or listening IP sockets, will be bound to this [rdomain(4)](https://man.openbsd.org/rdomain.4). If the routing domain is set to **%D**, then the domain in which the incoming connection was received will be applied."]
#[doc = "See also: [RDomain](https://man.openbsd.org/sshd_config#RDomain)"]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RDomain<'a>(Cow<'a, str>);
impl<'a> RDomain<'a> {
    pub fn new(value: &'a str) -> Self {
        Self(value.into())
    }
}
impl<'a> From<&'a str> for RDomain<'a> {
    fn from(value: &'a str) -> Self {
        RDomain(value.into())
    }
}

impl<'a> crate::Parse<'a> for RDomain<'a> {
    type Output = RDomain<'a>;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("RDomain"),
            preceded(
                space1,
                map(
                    preceded(space0, take_while1(|c: char| !c.is_whitespace())),
                    RDomain::from,
                ),
            ),
        )(input)
    }
}

impl<'a> From<RDomain<'a>> for crate::Directive<'a> {
    fn from(directive: RDomain<'a>) -> Self {
        crate::directive::Directive::RDomain(directive)
    }
}
