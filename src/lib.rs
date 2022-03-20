use nom::IResult;

mod directive;

pub use directive::*;

pub trait Parse<'a>: Sized {
    type Output;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output>;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Modifier<T> {
    Set(T),
    Add(T),
    Remove(T),
}

impl<T> From<(Option<char>, T)> for Modifier<T> {
    fn from((sign, inner): (Option<char>, T)) -> Self {
        match sign {
            Some('+') => Modifier::Add(inner),
            Some('-') => Modifier::Remove(inner),
            None => Modifier::Set(inner),
            Some(c) => panic!("unexpected sign {c} ahead of modifier"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Ciphers, Directive, KexAlgorithms, Modifier, Parse};

    #[test]
    fn test_comma_separated() {
        assert_eq!(
            Directive::parse("Ciphers 3des-cbc,aes128-gcm@openssh.com")
                .unwrap()
                .1,
            Directive::Ciphers(vec![Ciphers::X3DesCbc, Ciphers::Aes128GcmOpensshCom])
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
                "KexAlgorithms +diffie-hellman-group14-sha12,diffie-hellman-group14-sha256"
            )
            .unwrap()
            .1,
            Directive::KexAlgorithms(Modifier::Add(vec![
                KexAlgorithms::DiffieHellmanGroup14Sha12,
                KexAlgorithms::DiffieHellmanGroup14Sha256
            ]))
        );

        assert_eq!(
            Directive::parse(
                "KexAlgorithms -diffie-hellman-group14-sha12,diffie-hellman-group14-sha256"
            )
            .unwrap()
            .1,
            Directive::KexAlgorithms(Modifier::Remove(vec![
                KexAlgorithms::DiffieHellmanGroup14Sha12,
                KexAlgorithms::DiffieHellmanGroup14Sha256
            ]))
        );

        assert_eq!(
            Directive::parse(
                "KexAlgorithms diffie-hellman-group14-sha12,diffie-hellman-group14-sha256"
            )
            .unwrap()
            .1,
            Directive::KexAlgorithms(Modifier::Set(vec![
                KexAlgorithms::DiffieHellmanGroup14Sha12,
                KexAlgorithms::DiffieHellmanGroup14Sha256
            ]))
        );
    }
    /*
       #[test]
       fn test_simple_config() {
           let config = Config::parse(indoc! {"
               AllowTcpForwarding remote
               AllowAgentForwarding no
           "});

           println!("{:#?}", config);
       }

       #[test]
       fn test_acceptenv() {
           let config = Config::parse(indoc! {"
               AcceptEnv LC_LANG LC_MONEY
               AcceptEnv Second Line
           "});

           println!("{:#?}", config);
       }
    */
}
