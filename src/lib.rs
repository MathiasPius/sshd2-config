//! This crate is not yet ready to be used.
//!
//! Long-term the plan for this library is to be a one-stop-shop for
//! parsing and generating configuration files to be consumed by `sshd_config`.
//!

use nom::IResult;

mod directive;
pub use directive::*;

pub trait ParseDirective<'a>: Sized {
    type Output;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output>;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Modifier<T> {
    Replace(T),
    Append(T),
    Prepend(T),
    Remove(T),
}

impl<T> From<(Option<char>, T)> for Modifier<T> {
    fn from((sign, inner): (Option<char>, T)) -> Self {
        match sign {
            Some('+') => Modifier::Append(inner),
            Some('-') => Modifier::Remove(inner),
            Some('^') => Modifier::Prepend(inner),
            None => Modifier::Replace(inner),
            Some(c) => panic!("unexpected sign {c} ahead of modifier"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_comma_separated() {
        assert_eq!(
            Directive::parse("Ciphers 3des-cbc,aes128-gcm@openssh.com")
                .unwrap()
                .1,
            Directive::Ciphers(Modifier::Replace(vec![
                Ciphers::X3DesCbc,
                Ciphers::Aes128GcmOpensshCom
            ]))
        );
    }

    #[test]
    fn test_space_separator_in_comma_separated() {
        assert!(Directive::parse("Ciphers 3des-cbc aes128-gcm@openssh.com").is_err());
    }

    #[test]
    fn test_modifier_comma_separated() {
        assert_eq!(
            Directive::parse(
                "KexAlgorithms +diffie-hellman-group14-sha1,diffie-hellman-group14-sha256"
            )
            .unwrap()
            .1,
            Directive::KexAlgorithms(Modifier::Append(vec![
                KexAlgorithms::DiffieHellmanGroup14Sha1,
                KexAlgorithms::DiffieHellmanGroup14Sha256
            ]))
        );

        assert_eq!(
            Directive::parse(
                "KexAlgorithms -diffie-hellman-group14-sha1,diffie-hellman-group14-sha256"
            )
            .unwrap()
            .1,
            Directive::KexAlgorithms(Modifier::Remove(vec![
                KexAlgorithms::DiffieHellmanGroup14Sha1,
                KexAlgorithms::DiffieHellmanGroup14Sha256
            ]))
        );

        assert_eq!(
            Directive::parse(
                "KexAlgorithms diffie-hellman-group14-sha1,diffie-hellman-group14-sha256"
            )
            .unwrap()
            .1,
            Directive::KexAlgorithms(Modifier::Replace(vec![
                KexAlgorithms::DiffieHellmanGroup14Sha1,
                KexAlgorithms::DiffieHellmanGroup14Sha256
            ]))
        );

        assert_eq!(
            Directive::parse(
                "KexAlgorithms ^diffie-hellman-group14-sha1,diffie-hellman-group14-sha256"
            )
            .unwrap()
            .1,
            Directive::KexAlgorithms(Modifier::Prepend(vec![
                KexAlgorithms::DiffieHellmanGroup14Sha1,
                KexAlgorithms::DiffieHellmanGroup14Sha256
            ]))
        );
    }

    #[test]
    fn test_allow_groups() {
        assert_eq!(
            Directive::parse("AllowGroups root wheel lol").unwrap().1,
            Directive::AllowGroups(vec![
                AllowGroups::new("root"),
                AllowGroups::new("wheel"),
                AllowGroups::new("lol"),
            ])
        );
    }

    #[test]
    fn test_authentication_methods() {
        Directive::parse("AuthenticationMethods none").unwrap();
        Directive::parse("AuthenticationMethods none,pubkey").unwrap();
        Directive::parse("AuthenticationMethods pubkey,pubkey gssapi-with-mic,hostbased").unwrap();
        Directive::parse("AuthenticationMethods keyboard-interactive,pubkey keyboard-interactive:bsdauth,password").unwrap();
    }
}
