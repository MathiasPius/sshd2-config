use nom::{
    branch::alt,
    bytes::complete::tag_no_case,
    character::complete::{line_ending, space1},
    combinator::{eof, into, value},
    multi::many0,
    sequence::{preceded, terminated},
    IResult,
};

pub trait Named {
    const NAME: &'static str;
}

pub trait Parse: Sized {
    fn parse(input: &str) -> IResult<&str, Self>;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Config(Vec<Directive>);

impl Parse for Config {
    fn parse(input: &str) -> IResult<&str, Self> {
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
}

/// Matches a single directive "{NAME} {Parse Result of T}"
fn directive<T: Parse + Named + Into<Directive>>() -> impl FnMut(&str) -> IResult<&str, Directive> {
    move |input| {
        terminated(
            preceded(tag_no_case(T::NAME), preceded(space1, into(T::parse))),
            alt((line_ending, eof)),
        )(input)
    }
}

impl Parse for Directive {
    fn parse(input: &str) -> IResult<&str, Self> {
        let dir = alt((
            directive::<AllowAgentForwarding>(),
            directive::<AllowTcpForwarding>(),
        ))(input)?;

        Ok(dir)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AllowAgentForwarding {
    Yes,
    No,
}

impl Into<Directive> for AllowAgentForwarding {
    fn into(self) -> Directive {
        Directive::AllowAgentForwarding(self)
    }
}

impl Named for AllowAgentForwarding {
    const NAME: &'static str = "AllowAgentForwarding";
}

impl Parse for AllowAgentForwarding {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            value(AllowAgentForwarding::Yes, tag_no_case("yes")),
            value(AllowAgentForwarding::No, tag_no_case("no")),
        ))(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AllowTcpForwarding {
    Yes,
    No,
    All,
    Local,
    Remote,
}

impl Into<Directive> for AllowTcpForwarding {
    fn into(self) -> Directive {
        Directive::AllowTcpForwarding(self)
    }
}

impl Named for AllowTcpForwarding {
    const NAME: &'static str = "AllowTcpForwarding";
}

impl Parse for AllowTcpForwarding {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            value(AllowTcpForwarding::Yes, tag_no_case("yes")),
            value(AllowTcpForwarding::No, tag_no_case("no")),
            value(AllowTcpForwarding::All, tag_no_case("all")),
            value(AllowTcpForwarding::Local, tag_no_case("local")),
            value(AllowTcpForwarding::Remote, tag_no_case("remote")),
        ))(input)
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
