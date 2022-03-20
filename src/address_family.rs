//! Generated file, do not edit by hand

use crate::Directive;
use nom::{
    branch::alt, bytes::complete::tag_no_case, character::complete::space1, combinator::value,
    sequence::preceded, IResult,
};
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
impl crate::Parse for AddressFamily {
    type Output = Self;
    fn parse(input: &str) -> IResult<&str, Self> {
        preceded(
            tag_no_case("AddressFamily"),
            preceded(
                space1,
                alt((
                    value(AddressFamily::Any, tag_no_case("any")),
                    value(AddressFamily::Inet, tag_no_case("inet")),
                    value(AddressFamily::Inet6, tag_no_case("inet6")),
                )),
            ),
        )(input)
    }
}
impl Into<Directive> for AddressFamily {
    fn into(self) -> Directive {
        Directive::AddressFamily(self)
    }
}
