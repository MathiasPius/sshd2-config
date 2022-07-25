use std::borrow::Cow;

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
pub struct Comment<'a>(Cow<'a, str>);

impl<'a> From<&'a str> for Comment<'a> {
    fn from(comment: &'a str) -> Self {
        Comment(comment.into())
    }
}

impl<'a> AsRef<str> for Comment<'a> {
    fn as_ref(&self) -> &str {
        self.0
            .as_ref()
            .trim_start_matches(|c: char| c.is_whitespace() || c == '#')
            .trim_end()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ConfigLine<'a> {
    directive: Option<Directive<'a>>,
    comment: Option<Comment<'a>>,
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
            Ok(ConfigLine {
                directive,
                comment: comment.map(Comment::from),
            })
        }
    }

    pub fn directive(&'a self) -> Option<&Directive<'a>> {
        self.directive.as_ref()
    }

    pub fn comment(&'a self) -> Option<&Comment<'a>> {
        self.comment.as_ref()
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

    pub fn lines(&self) -> &[ConfigLine<'a>] {
        self.lines.as_ref()
    }

    pub fn directives(&'a self) -> impl Iterator<Item = &Directive<'a>> {
        self.lines.iter().filter_map(ConfigLine::directive)
    }
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
        assert_eq!(
            "Comment",
            ConfigLine::try_parse("ListenAddress 127.0.0.1#Comment")
                .unwrap()
                .comment()
                .unwrap()
                .as_ref()
        );
        assert_eq!(
            "Comment",
            ConfigLine::try_parse("ListenAddress 127.0.0.1   #Comment")
                .unwrap()
                .comment()
                .unwrap()
                .as_ref()
        );
        assert_eq!(
            "Comment",
            ConfigLine::try_parse("ListenAddress 127.0.0.1#   Comment")
                .unwrap()
                .comment()
                .unwrap()
                .as_ref()
        );
        assert_eq!(
            "Comment",
            ConfigLine::try_parse("ListenAddress 127.0.0.1   #   Comment")
                .unwrap()
                .comment()
                .unwrap()
                .as_ref()
        );
    }

    #[test]
    fn test_parse_config_1() {
        let config = Config::try_parse(include_str!("../test-data/1-sshd-config"));

        println!("{:#?}", config.unwrap());
    }
}
