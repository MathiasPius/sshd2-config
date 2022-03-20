pub mod allow_agent_forwarding;
pub mod allow_tcp_forwarding;
pub mod kex_algorithms;

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

trait Parse: Sized {
    type Output;
    fn parse(input: &str) -> IResult<&str, Self::Output>;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Config(Vec<Directive>);

impl Parse for Config {
    type Output = Self;
    fn parse(input: &str) -> IResult<&str, Self::Output> {
        into(many0(Directive::parse))(input)
    }
}

impl Into<Config> for Vec<Directive> {
    fn into(self) -> Config {
        Config(self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Directive {
    AllowAgentForwarding(AllowAgentForwarding),
    AllowTcpForwarding(AllowTcpForwarding),
    KexAlgorithms(KexAlgorithms),
}

/// Matches a single directive "{NAME} {Parse Result of T}"
fn directive<T: Parse>() -> impl FnMut(&str) -> IResult<&str, Directive>
where
    <T as Parse>::Output: Into<Directive>,
{
    move |input| terminated(into(T::parse), alt((line_ending, eof)))(input)
}

impl Parse for Directive {
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
}
