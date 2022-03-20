//! Generated file, do not edit by hand

#[allow(unused_imports)]
use crate::Directive;
#[allow(unused_imports)]
use nom::{
    branch::alt,
    bytes::complete::{tag_no_case, take_until, take_while1},
    character::complete::{alphanumeric1, space0, space1},
    combinator::{map, not, value},
    multi::many1,
    sequence::preceded,
    IResult,
};
#[allow(unused_imports)]
use std::borrow::Cow;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AddressFamilyDirective(AddressFamily);
impl<'a> crate::Parse<'a> for AddressFamilyDirective {
    type Output = AddressFamilyDirective;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        map(
            preceded(tag_no_case("AddressFamily"), AddressFamily::parse),
            |value| AddressFamilyDirective(value),
        )(input)
    }
}
impl<'a> From<AddressFamilyDirective> for Directive<'a> {
    fn from(directive: AddressFamilyDirective) -> Self {
        Directive::AddressFamily(directive)
    }
}
#[doc = "Specifies which address family should be used by sshd(8)."]
#[doc = "Valid arguments are any (the default), inet (use IPv4"]
#[doc = "only), or inet6 (use IPv6 only)."]
#[doc = "See also: [sshd_config(5)](https://man7.org/linux/man-pages/man5/sshd_config.5.html)"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AddressFamily {
    Any,
    Inet,
    Inet6,
}
impl<'a> crate::Parse<'a> for AddressFamily {
    type Output = Self;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            space1,
            alt((
                value(AddressFamily::Any, tag_no_case("any")),
                value(AddressFamily::Inet, tag_no_case("inet")),
                value(AddressFamily::Inet6, tag_no_case("inet6")),
            )),
        )(input)
    }
}
