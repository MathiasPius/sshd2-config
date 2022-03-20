//! Generated file, do not edit by hand

mod accept_env;
mod address_family;
mod allow_agent_forwarding;
mod allow_groups;
mod allow_tcp_forwarding;
mod authentication_methods;
mod ciphers;
mod kex_algorithms;

use crate::{Modifier, Parse};
pub use accept_env::*;
pub use address_family::*;
pub use allow_agent_forwarding::*;
pub use allow_groups::*;
pub use allow_tcp_forwarding::*;
pub use authentication_methods::*;
pub use ciphers::*;
pub use kex_algorithms::*;
use nom::IResult;
use nom::{
    branch::alt,
    character::complete::line_ending,
    combinator::{eof, into},
    sequence::terminated,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Directive<'a> {
    Ciphers(Modifier<Vec<Ciphers>>),
    AllowGroups(Vec<AllowGroups<'a>>),
    KexAlgorithms(Modifier<Vec<KexAlgorithms>>),
    AddressFamily(AddressFamily),
    AllowAgentForwarding(AllowAgentForwarding),
    AllowTcpForwarding(AllowTcpForwarding),
    AuthenticationMethods(Vec<Vec<AuthenticationMethods>>),
    AcceptEnv(Vec<AcceptEnv<'a>>),
}

fn directive<'a, T: Parse<'a>>(input: &'a str) -> IResult<&'a str, Directive>
where
    <T as Parse<'a>>::Output: Into<Directive<'a>>,
{
    terminated(into(<T as Parse<'a>>::parse), alt((line_ending, eof)))(input)
}
impl<'a> Parse<'a> for Directive<'a> {
    type Output = Self;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        alt((
            directive::<Ciphers>,
            directive::<AllowGroups>,
            directive::<KexAlgorithms>,
            directive::<AddressFamily>,
            directive::<AllowAgentForwarding>,
            directive::<AllowTcpForwarding>,
            directive::<AuthenticationMethods>,
            directive::<AcceptEnv>,
        ))(input)
    }
}
