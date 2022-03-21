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

#[doc = "The server disconnects after this time if the user has not successfully logged in. If the value is 0, there is no time limit. The default is 120 seconds."]
#[doc = "See also: [LoginGraceTime](https://man.openbsd.org/sshd_config#LoginGraceTime)"]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LoginGraceTime(u64);
impl LoginGraceTime {
    pub fn new(value: u64) -> Self {
        Self(value)
    }
}
impl From<u64> for LoginGraceTime {
    fn from(value: u64) -> Self {
        LoginGraceTime(value)
    }
}

impl<'a> crate::Parse<'a> for LoginGraceTime {
    type Output = LoginGraceTime;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("LoginGraceTime"),
            preceded(
                space1,
                map(
                    preceded(space0, nom::character::complete::u64),
                    LoginGraceTime::from,
                ),
            ),
        )(input)
    }
}

impl<'a> From<LoginGraceTime> for crate::Directive<'a> {
    fn from(directive: LoginGraceTime) -> Self {
        crate::directive::Directive::LoginGraceTime(directive)
    }
}
