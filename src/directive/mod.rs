//! Generated file, do not edit by hand

mod accept_env;
mod address_family;
mod allow_agent_forwarding;
mod allow_groups;
mod allow_stream_local_forwarding;
mod allow_tcp_forwarding;
mod allow_users;
mod authentication_methods;
mod authorized_keys_command;
mod ciphers;
mod kex_algorithms;

use crate::{Modifier, Parse};
pub use accept_env::*;
pub use address_family::*;
pub use allow_agent_forwarding::*;
pub use allow_groups::*;
pub use allow_stream_local_forwarding::*;
pub use allow_tcp_forwarding::*;
pub use allow_users::*;
pub use authentication_methods::*;
pub use authorized_keys_command::*;
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
    AllowTcpForwarding(AllowTcpForwarding),
    AddressFamily(AddressFamily),
    AcceptEnv(Vec<AcceptEnv<'a>>),
    AllowStreamLocalForwarding(AllowStreamLocalForwarding),
    AuthenticationMethods(Vec<Vec<AuthenticationMethods>>),
    AllowUsers(Vec<AllowUsers<'a>>),
    AuthorizedKeysCommand(AuthorizedKeysCommand<'a>),
    KexAlgorithms(Modifier<Vec<KexAlgorithms>>),
    AllowGroups(Vec<AllowGroups<'a>>),
    Ciphers(Modifier<Vec<Ciphers>>),
    AllowAgentForwarding(AllowAgentForwarding),
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
            directive::<AllowTcpForwarding>,
            directive::<AddressFamily>,
            directive::<AcceptEnv>,
            directive::<AllowStreamLocalForwarding>,
            directive::<AuthenticationMethods>,
            directive::<AllowUsers>,
            directive::<AuthorizedKeysCommand>,
            directive::<KexAlgorithms>,
            directive::<AllowGroups>,
            directive::<Ciphers>,
            directive::<AllowAgentForwarding>,
        ))(input)
    }
}
