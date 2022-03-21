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

#[doc = "Specifies whether any ~/.ssh/rc file is executed. The default is **yes**."]
#[doc = "See also: [PermitUserRC](https://man.openbsd.org/sshd_config#PermitUserRC)"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PermitUserRC {
    #[doc = "yes"]
    Yes,
    #[doc = "no"]
    No,
}

impl<'a> crate::Parse<'a> for PermitUserRC {
    type Output = PermitUserRC;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("PermitUserRC"),
            preceded(
                space1,
                preceded(
                    space0,
                    alt((
                        value(PermitUserRC::Yes, tag_no_case("yes")),
                        value(PermitUserRC::No, tag_no_case("no")),
                    )),
                ),
            ),
        )(input)
    }
}

impl<'a> From<PermitUserRC> for crate::Directive<'a> {
    fn from(directive: PermitUserRC) -> Self {
        crate::directive::Directive::PermitUserRC(directive)
    }
}
