//! Generated file, do not edit by hand

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

#[doc = "Specifies whether [sshd(8)](https://man.openbsd.org/sshd.8) should check file modes and ownership of the user's files and home directory before accepting login. This is normally desirable because novices sometimes accidentally leave their directory or files world-writable. The default is **yes**. Note that this does not apply to **ChrootDirectory**, whose permissions and ownership are checked unconditionally."]
#[doc = "See also: [StrictModes](https://man.openbsd.org/sshd_config#StrictModes)"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum StrictModes {
    #[doc = "yes"]
    Yes,
    #[doc = "no"]
    No,
}

impl<'a> crate::Parse<'a> for StrictModes {
    type Output = StrictModes;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("StrictModes"),
            preceded(
                space1,
                preceded(
                    space0,
                    alt((
                        value(StrictModes::Yes, tag_no_case("yes")),
                        value(StrictModes::No, tag_no_case("no")),
                    )),
                ),
            ),
        )(input)
    }
}

impl<'a> From<StrictModes> for crate::Directive<'a> {
    fn from(directive: StrictModes) -> Self {
        crate::directive::Directive::StrictModes(directive)
    }
}
