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

#[doc = "Specifies whether to allow keyboard-interactive authentication. All authentication styles from [login.conf(5)](https://man.openbsd.org/login.conf.5) are supported. The default is **yes**. The argument to this keyword must be **yes** or **no**. **ChallengeResponseAuthentication** is a deprecated alias for this."]
#[doc = "See also: [KbdInteractiveAuthentication](https://man.openbsd.org/sshd_config#KbdInteractiveAuthentication)"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum KbdInteractiveAuthentication {
    #[doc = "yes"]
    Yes,
    #[doc = "no"]
    No,
}

impl<'a> crate::Parse<'a> for KbdInteractiveAuthentication {
    type Output = KbdInteractiveAuthentication;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("KbdInteractiveAuthentication"),
            preceded(
                space1,
                preceded(
                    space0,
                    alt((
                        value(KbdInteractiveAuthentication::Yes, tag_no_case("yes")),
                        value(KbdInteractiveAuthentication::No, tag_no_case("no")),
                    )),
                ),
            ),
        )(input)
    }
}

impl<'a> From<KbdInteractiveAuthentication> for crate::Directive<'a> {
    fn from(directive: KbdInteractiveAuthentication) -> Self {
        crate::directive::Directive::KbdInteractiveAuthentication(directive)
    }
}
