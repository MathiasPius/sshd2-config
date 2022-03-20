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

#[doc = "See also: [AuthenticationMethods](https://man.openbsd.org/sshd_config#AuthenticationMethods)"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AuthenticationMethods {
    #[doc = "Public Key Authentication"]
    Pubkey,
    #[doc = "Challenge Response"]
    ChallengeResponse,
    #[doc = "Password Authentication"]
    Password,
    #[doc = "Keyboard Interactive"]
    KeyboardInteractive,
}

impl<'a> crate::Parse<'a> for AuthenticationMethods {
    type Output = Vec<Vec<AuthenticationMethods>>;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("AuthenticationMethods"),
            preceded(
                space1,
                separated_list1(
                    tag(" "),
                    separated_list1(
                        tag(","),
                        preceded(
                            space0,
                            alt((
                                value(AuthenticationMethods::Pubkey, tag_no_case("pubkey")),
                                value(
                                    AuthenticationMethods::ChallengeResponse,
                                    tag_no_case("challenge-response"),
                                ),
                                value(AuthenticationMethods::Password, tag_no_case("password")),
                                value(
                                    AuthenticationMethods::KeyboardInteractive,
                                    tag_no_case("keyboard-interactive"),
                                ),
                            )),
                        ),
                    ),
                ),
            ),
        )(input)
    }
}

impl<'a> From<Vec<Vec<AuthenticationMethods>>> for crate::Directive<'a> {
    fn from(directive: Vec<Vec<AuthenticationMethods>>) -> Self {
        crate::directive::Directive::AuthenticationMethods(directive)
    }
}
