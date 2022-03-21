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

#[doc = "Specifies the local addresses [sshd(8)](https://man.openbsd.org/sshd.8) should listen on. The following forms may be used:"]
#[doc = ""]
#[doc = "* **ListenAddress** hostname|address [**rdomain** domain]"]
#[doc = "* **ListenAddress** hostname:port [**rdomain** domain]"]
#[doc = "* **ListenAddress** IPv4_address:port [**rdomain** domain]"]
#[doc = "* **ListenAddress** [hostname|address]:port [**rdomain** domain]"]
#[doc = ""]
#[doc = "The optional **rdomain** qualifier requests [sshd(8)](https://man.openbsd.org/sshd.8) listen in an explicit routing domain. If port is not specified, sshd will listen on the address and all **Port** options specified. The default is to listen on all local addresses on the current default routing domain. Multiple **ListenAddress** options are permitted. For more information on routing domains, see [rdomain(4)](https://man.openbsd.org/rdomain.4)."]
#[doc = "See also: [ListenAddress](https://man.openbsd.org/sshd_config#ListenAddress)"]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ListenAddress<'a>(Cow<'a, str>);
impl<'a> ListenAddress<'a> {
    pub fn new(value: &'a str) -> Self {
        Self(value.into())
    }
}
impl<'a> From<&'a str> for ListenAddress<'a> {
    fn from(value: &'a str) -> Self {
        ListenAddress(value.into())
    }
}

impl<'a> crate::Parse<'a> for ListenAddress<'a> {
    type Output = Vec<ListenAddress<'a>>;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("ListenAddress"),
            preceded(
                space1,
                separated_list1(
                    tag(" "),
                    map(
                        preceded(space0, take_while1(|c: char| !c.is_whitespace())),
                        ListenAddress::from,
                    ),
                ),
            ),
        )(input)
    }
}

impl<'a> From<Vec<ListenAddress<'a>>> for crate::Directive<'a> {
    fn from(directive: Vec<ListenAddress<'a>>) -> Self {
        crate::directive::Directive::ListenAddress(directive)
    }
}
