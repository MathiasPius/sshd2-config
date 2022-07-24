//! This file has been automatically generated. Any changes made to it will be overwritten upon subsequent runs!
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

/// Specifies the IPv4 type-of-service or DSCP class for the connection.
///
/// Accepted values are **af11**, **af12**, **af13**, **af21**, **af22**, **af23**, **af31**, **af32**, **af33**, **af41**, **af42**, **af43**, **cs0**, **cs1**, **cs2**, **cs3**, **cs4**, **cs5**, **cs6**, **cs7**, **ef**, **le**, **lowdelay**, **throughput**, **reliability**, a numeric value, or **none** to use the operating system default.
/// This option may take one or two arguments, separated by whitespace.
/// If one argument is specified, it is used as the packet class unconditionally.
/// If two values are specified, the first is automatically selected for interactive sessions and the second for non-interactive sessions.
/// The default is **af21** (Low-Latency Data) for interactive sessions and **cs1** (Lower Effort) for non-interactive sessions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum IPQoS {
    Integer(u64),
    Predefined(IPQoSPredefined),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum IPQoSPredefined {
    #[doc = "af11"]
    Af11,
    #[doc = "af12"]
    Af12,
    #[doc = "af13"]
    Af13,
    #[doc = "af21"]
    Af21,
    #[doc = "af22"]
    Af22,
    #[doc = "af23"]
    Af23,
    #[doc = "af31"]
    Af31,
    #[doc = "af32"]
    Af32,
    #[doc = "af33"]
    Af33,
    #[doc = "af41"]
    Af41,
    #[doc = "af42"]
    Af42,
    #[doc = "af43"]
    Af43,
    #[doc = "cs0"]
    Cs0,
    #[doc = "cs1"]
    Cs1,
    #[doc = "cs2"]
    Cs2,
    #[doc = "cs3"]
    Cs3,
    #[doc = "cs4"]
    Cs4,
    #[doc = "cs5"]
    Cs5,
    #[doc = "cs6"]
    Cs6,
    #[doc = "cs7"]
    Cs7,
    #[doc = "ef"]
    Ef,
    #[doc = "le"]
    Le,
    #[doc = "lowdelay"]
    Lowdelay,
    #[doc = "throughput"]
    Throughput,
    #[doc = "reliability"]
    Reliability,
    #[doc = "none"]
    None,
}
impl From<u64> for IPQoS {
    fn from(inner: u64) -> IPQoS {
        IPQoS::Integer(inner)
    }
}
impl From<IPQoSPredefined> for IPQoS {
    fn from(inner: IPQoSPredefined) -> IPQoS {
        IPQoS::Predefined(inner)
    }
}

impl<'a> crate::ParseDirective<'a> for IPQoS {
    type Output = Vec<IPQoS>;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("IPQoS"),
            preceded(
                space1,
                separated_list1(
                    tag(" "),
                    alt((
                        map(preceded(space0, nom::character::complete::u64), IPQoS::from),
                        map(
                            preceded(
                                space0,
                                alt((
                                    alt((
                                        value(IPQoSPredefined::Af11, tag_no_case("af11")),
                                        value(IPQoSPredefined::Af12, tag_no_case("af12")),
                                        value(IPQoSPredefined::Af13, tag_no_case("af13")),
                                        value(IPQoSPredefined::Af21, tag_no_case("af21")),
                                        value(IPQoSPredefined::Af22, tag_no_case("af22")),
                                        value(IPQoSPredefined::Af23, tag_no_case("af23")),
                                        value(IPQoSPredefined::Af31, tag_no_case("af31")),
                                        value(IPQoSPredefined::Af32, tag_no_case("af32")),
                                        value(IPQoSPredefined::Af33, tag_no_case("af33")),
                                        value(IPQoSPredefined::Af41, tag_no_case("af41")),
                                        value(IPQoSPredefined::Af42, tag_no_case("af42")),
                                        value(IPQoSPredefined::Af43, tag_no_case("af43")),
                                        value(IPQoSPredefined::Cs0, tag_no_case("cs0")),
                                        value(IPQoSPredefined::Cs1, tag_no_case("cs1")),
                                        value(IPQoSPredefined::Cs2, tag_no_case("cs2")),
                                        value(IPQoSPredefined::Cs3, tag_no_case("cs3")),
                                        value(IPQoSPredefined::Cs4, tag_no_case("cs4")),
                                        value(IPQoSPredefined::Cs5, tag_no_case("cs5")),
                                        value(IPQoSPredefined::Cs6, tag_no_case("cs6")),
                                        value(IPQoSPredefined::Cs7, tag_no_case("cs7")),
                                    )),
                                    alt((
                                        value(IPQoSPredefined::Ef, tag_no_case("ef")),
                                        value(IPQoSPredefined::Le, tag_no_case("le")),
                                        value(IPQoSPredefined::Lowdelay, tag_no_case("lowdelay")),
                                        value(
                                            IPQoSPredefined::Throughput,
                                            tag_no_case("throughput"),
                                        ),
                                        value(
                                            IPQoSPredefined::Reliability,
                                            tag_no_case("reliability"),
                                        ),
                                        value(IPQoSPredefined::None, tag_no_case("none")),
                                    )),
                                )),
                            ),
                            IPQoS::from,
                        ),
                    )),
                ),
            ),
        )(input)
    }
}

impl<'a> From<Vec<IPQoS>> for crate::Directive<'a> {
    fn from(directive: Vec<IPQoS>) -> Self {
        crate::directive::Directive::IPQoS(directive)
    }
}
