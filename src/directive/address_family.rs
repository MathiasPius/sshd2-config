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

#[doc = "Specifies which address family should be used by [sshd(8)](https://man.openbsd.org/sshd.8). Valid arguments are **any** (the default), **inet** (use IPv4 only), or **inet6** (use IPv6 only)."]
#[doc = "See also: [AddressFamily](https://man.openbsd.org/sshd_config#AddressFamily)"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AddressFamily {
    #[doc = "any"]
    Any,
    #[doc = "inet"]
    Inet,
    #[doc = "inet6"]
    Inet6,
}

impl<'a> crate::Parse<'a> for AddressFamily {
    type Output = AddressFamily;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("AddressFamily"),
            preceded(
                space1,
                preceded(
                    space0,
                    alt((
                        value(AddressFamily::Any, tag_no_case("any")),
                        value(AddressFamily::Inet, tag_no_case("inet")),
                        value(AddressFamily::Inet6, tag_no_case("inet6")),
                    )),
                ),
            ),
        )(input)
    }
}

impl<'a> From<AddressFamily> for crate::Directive<'a> {
    fn from(directive: AddressFamily) -> Self {
        crate::directive::Directive::AddressFamily(directive)
    }
}
