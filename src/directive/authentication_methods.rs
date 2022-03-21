//! This file has been automatically generated. Any changes made to it will be overwritten upon subsequent runs!
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

/// Specifies the authentication methods that must be successfully completed for a user to be granted access.
///
/// This option must be followed by one or more lists of comma-separated authentication method names, or by the single string **any** to indicate the default behaviour of accepting any single authentication method.
/// If the default is overridden, then successful authentication requires completion of every method in at least one of these lists.
/// For example, 'publickey,password publickey,keyboard-interactive' would require the user to complete public key authentication, followed by either password or keyboard interactive authentication.
/// Only methods that are next in one or more lists are offered at each stage, so for this example it would not be possible to attempt password or keyboard-interactive authentication before public key.
///
/// For keyboard interactive authentication it is also possible to restrict authentication to a specific device by appending a colon followed by the device identifier **bsdauth**, **pam**, or **skey**, depending on the server configuration.
/// For example, 'keyboard-interactive:bsdauth' would restrict keyboard interactive authentication to the **bsdauth** device.
///
/// If the publickey method is listed more than once, [sshd(8)](https://man.openbsd.org/sshd.8) verifies that keys that have been used successfully are not reused for subsequent authentications.
/// For example, 'publickey,publickey' requires successful authentication using two different public keys.
///
/// Note that each authentication method listed should also be explicitly enabled in the configuration.
///
/// The available authentication methods are: 'gssapi-with-mic', 'hostbased', 'keyboard-interactive', 'none' (used for access to password-less accounts when **PermitEmptyPasswords** is enabled), 'password' and 'publickey'.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AuthenticationMethods {
    #[doc = "Allow authentication by any method."]
    Any,
    #[doc = "Public Key Authentication"]
    Pubkey,
    #[doc = "Challenge Response"]
    ChallengeResponse,
    #[doc = "Password Authentication"]
    Password,
    #[doc = "Keyboard Interactive via bsdauth device"]
    KeyboardInteractiveBsdauth,
    #[doc = "Keyboard Interactive via skey device"]
    KeyboardInteractiveSkey,
    #[doc = "Keyboard Interactive via PAM device"]
    KeyboardInteractivePam,
    #[doc = "Keyboard Interactive"]
    KeyboardInteractive,
    #[doc = "gssapi-with-mic"]
    GssapiWithMic,
    #[doc = "hostbased"]
    Hostbased,
    #[doc = "none"]
    None,
}

impl<'a> crate::ParseDirective<'a> for AuthenticationMethods {
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
                                value(AuthenticationMethods::Any, tag_no_case("any")),
                                value(AuthenticationMethods::Pubkey, tag_no_case("pubkey")),
                                value(
                                    AuthenticationMethods::ChallengeResponse,
                                    tag_no_case("challenge-response"),
                                ),
                                value(AuthenticationMethods::Password, tag_no_case("password")),
                                value(
                                    AuthenticationMethods::KeyboardInteractiveBsdauth,
                                    tag_no_case("keyboard-interactive:bsdauth"),
                                ),
                                value(
                                    AuthenticationMethods::KeyboardInteractiveSkey,
                                    tag_no_case("keyboard-interactive:skey"),
                                ),
                                value(
                                    AuthenticationMethods::KeyboardInteractivePam,
                                    tag_no_case("keyboard-interactive:pam"),
                                ),
                                value(
                                    AuthenticationMethods::KeyboardInteractive,
                                    tag_no_case("keyboard-interactive"),
                                ),
                                value(
                                    AuthenticationMethods::GssapiWithMic,
                                    tag_no_case("gssapi-with-mic"),
                                ),
                                value(AuthenticationMethods::Hostbased, tag_no_case("hostbased")),
                                value(AuthenticationMethods::None, tag_no_case("none")),
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

