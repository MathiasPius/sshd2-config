pub mod accept_env;
pub mod allow_agent_forwarding;
pub mod allow_tcp_forwarding;
pub mod kex_algorithms;

use accept_env::AcceptEnv;
use allow_agent_forwarding::AllowAgentForwarding;
use allow_tcp_forwarding::AllowTcpForwarding;
use kex_algorithms::KexAlgorithms;
use nom::{
    branch::alt,
    character::complete::line_ending,
    combinator::{eof, into},
    multi::many0,
    sequence::terminated,
    IResult,
};

trait Parse<'a>: Sized {
    type Output;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output>;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Config<'a>(Vec<Directive<'a>>);

impl<'a> Parse<'a> for Config<'a> {
    type Output = Self;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        into(many0(Directive::parse))(input)
    }
}

impl<'a> Into<Config<'a>> for Vec<Directive<'a>> {
    fn into(self) -> Config<'a> {
        Config(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Directive<'a> {
    AllowAgentForwarding(AllowAgentForwarding),
    AllowTcpForwarding(AllowTcpForwarding),
    KexAlgorithms(KexAlgorithms),
    AcceptEnv(AcceptEnv<'a>),
}

/// Matches a single directive "{NAME} {Parse Result of T}"
fn directive<'a, T: Parse<'a>>() -> impl FnMut(&'a str) -> IResult<&'a str, Directive>
where
    <T as Parse<'a>>::Output: Into<Directive<'a>>,
{
    move |input| terminated(into(<T as Parse<'a>>::parse), alt((line_ending, eof)))(input)
}

impl<'a> Parse<'a> for Directive<'a> {
    type Output = Self;
    fn parse(input: &str) -> IResult<&str, Self::Output> {
        let dir = alt((
            directive::<AllowAgentForwarding>(),
            directive::<AllowTcpForwarding>(),
        ))(input)?;

        Ok(dir)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Config, Directive, Parse};
    use indoc::indoc;

    #[test]
    fn test_tcp_forwarding() {
        println!("{:#?}", Directive::parse("AllowTcpForwarding remote"));
    }

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
        "});

        println!("{:#?}", config);
    }
}
