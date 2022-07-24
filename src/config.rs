use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, not_line_ending, space0},
    combinator::{all_consuming, eof, opt, recognize},
    multi::many_till,
    sequence::{terminated, tuple},
};

use crate::Directive;
use crate::ParseDirective;

type Error<'a> = nom::Err<nom::error::Error<&'a str>>;

#[derive(Debug, PartialEq, Eq)]
struct ConfigLine<'a> {
    directive: Option<Directive<'a>>,
    comment: Option<&'a str>,
}

impl<'a> ConfigLine<'a> {
    pub fn try_parse(line: &'a str) -> Result<ConfigLine<'a>, Error<'a>> {
        let (rest, (directive, comment)) = all_consuming(terminated(
            tuple((
                opt(Directive::parse),
                opt(recognize(tuple((
                    space0,
                    tag("#"),
                    many_till(not_line_ending, alt((line_ending, eof))),
                )))),
            )),
            opt(line_ending),
        ))(line)?;

        if !rest.is_empty() {
            panic!("rest: {rest}");
        } else {
            Ok(ConfigLine { directive, comment })
        }
    }
}

#[derive(Debug)]
pub struct Config<'a> {
    lines: Vec<ConfigLine<'a>>,
}

impl<'a> Config<'a> {
    pub fn try_parse(config: &'a str) -> Result<Config<'a>, Error> {
        let mut lines = Vec::new();
        for line in config.lines() {
            lines.push(ConfigLine::try_parse(line).unwrap());
        }

        Ok(Config { lines })
    }
    /*
    pub fn serialize(&self) -> String {
        let mut output = String::new();
        for line in &self.lines {

        }
    }
     */
}

#[cfg(test)]
mod tests {
    use super::{Config, ConfigLine};
    use crate::{Directive, ListenAddress};

    #[test]
    fn parse_single_config_line() {
        let no_comment = ConfigLine {
            directive: Some(Directive::ListenAddress(vec![ListenAddress::new(
                "127.0.0.1",
            )])),
            comment: None,
        };

        assert_eq!(
            no_comment,
            ConfigLine::try_parse("ListenAddress 127.0.0.1").unwrap()
        );
        assert_eq!(
            no_comment,
            ConfigLine::try_parse("ListenAddress 127.0.0.1\n").unwrap()
        );
        println!(
            "{:#?}",
            ConfigLine::try_parse("ListenAddress 127.0.0.1#Comment").unwrap()
        );
        println!(
            "{:#?}",
            ConfigLine::try_parse("ListenAddress 127.0.0.1 #Comment").unwrap()
        );
        println!(
            "{:#?}",
            ConfigLine::try_parse("ListenAddress 127.0.0.1 #   Comment").unwrap()
        );
    }

    #[test]
    fn test_parse_config_1() {
        let config = Config::try_parse(include_str!("../test-data/1-sshd-config"));

        println!("{:#?}", config);
    }
}
