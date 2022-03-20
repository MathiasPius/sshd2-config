//! Generated file, do not edit by hand

#[allow(unused_imports)]
use crate::Directive;
#[allow(unused_imports)]
use nom::{
    branch::alt,
    bytes::complete::tag_no_case,
    character::complete::{alphanumeric1, space0, space1},
    combinator::{map, value},
    multi::many1,
    sequence::preceded,
    IResult,
};
#[allow(unused_imports)]
use std::borrow::Cow;
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
