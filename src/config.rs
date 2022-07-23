use nom::bytes::complete::tag;
use nom::character::complete::not_line_ending;
use nom::character::complete::space0;
use nom::combinator::all_consuming;
use nom::combinator::opt;

use nom::combinator::recognize;
use nom::multi::many0;

use nom::sequence::tuple;

use crate::Directive;
use crate::ParseDirective;

#[derive(Debug)]
struct Line<'a> {
    directive: Option<Directive<'a>>,
    comment: Option<&'a str>,
}

#[derive(Debug)]
pub struct Config<'a> {
    lines: Vec<Line<'a>>,
}

impl<'a> Config<'a> {
    pub fn from_str(config: &'a str) -> Result<Config<'a>, nom::Err<nom::error::Error<&str>>> {
        let mut pattern = all_consuming(tuple((
            opt(Directive::parse),
            opt(recognize(tuple((space0, tag("#"), many0(not_line_ending))))),
        )));

        let mut lines = Vec::new();
        for line in config.lines() {
            let (leftovers, (directive, comment)) = pattern(line)?;

            if leftovers != "" {
                panic!("leftovrs: {leftovers}");
            } else {
                lines.push(Line { directive, comment });
            }
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
    use super::Config;

    #[test]
    fn test_parse_config_1() {
        let config = Config::from_str(include_str!("../test-data/1-sshd-config"));

        println!("{:#?}", config);
    }
}
