use nom::IResult;

mod directive;

pub use directive::*;

pub trait Parse<'a>: Sized {
    type Output;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output>;
}

#[cfg(test)]
mod tests {
    use crate::{Ciphers, Directive, Parse};

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
