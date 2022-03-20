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
mod authorized_keys_command_user;
mod authorized_keys_file;
mod authorized_principals_command;
mod authorized_principals_command_user;
mod authorized_principals_file;
mod banner;
mod ca_signature_algorithms;
mod chroot_directory;
mod ciphers;
mod client_alive_count_max;
mod client_alive_interval;
mod compression;
mod deny_groups;
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
pub use authorized_keys_command_user::*;
pub use authorized_keys_file::*;
pub use authorized_principals_command::*;
pub use authorized_principals_command_user::*;
pub use authorized_principals_file::*;
pub use banner::*;
pub use ca_signature_algorithms::*;
pub use chroot_directory::*;
pub use ciphers::*;
pub use client_alive_count_max::*;
pub use client_alive_interval::*;
pub use compression::*;
pub use deny_groups::*;
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
    ClientAliveCountMax(ClientAliveCountMax),
    AllowUsers(Vec<AllowUsers<'a>>),
    AllowGroups(Vec<AllowGroups<'a>>),
    AuthorizedPrincipalsFile(AuthorizedPrincipalsFile<'a>),
    AcceptEnv(Vec<AcceptEnv<'a>>),
    AllowAgentForwarding(AllowAgentForwarding),
    AuthenticationMethods(Vec<Vec<AuthenticationMethods>>),
    AddressFamily(AddressFamily),
    Banner(Banner<'a>),
    KexAlgorithms(Modifier<Vec<KexAlgorithms>>),
    AuthorizedKeysCommandUser(AuthorizedKeysCommandUser<'a>),
    Compression(Compression),
    AuthorizedKeysFile(AuthorizedKeysFile<'a>),
    AuthorizedPrincipalsCommandUser(AuthorizedPrincipalsCommandUser<'a>),
    Ciphers(Modifier<Vec<Ciphers>>),
    ChrootDirectory(ChrootDirectory<'a>),
    ClientAliveInterval(ClientAliveInterval),
    AuthorizedKeysCommand(AuthorizedKeysCommand<'a>),
    CASignatureAlgorithms(Modifier<Vec<CASignatureAlgorithms>>),
    AuthorizedPrincipalsCommand(AuthorizedPrincipalsCommand<'a>),
    DenyGroups(DenyGroups<'a>),
    AllowTcpForwarding(AllowTcpForwarding),
    AllowStreamLocalForwarding(AllowStreamLocalForwarding),
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
            alt((
                directive::<ClientAliveCountMax>,
                directive::<AllowUsers>,
                directive::<AllowGroups>,
                directive::<AuthorizedPrincipalsFile>,
                directive::<AcceptEnv>,
                directive::<AllowAgentForwarding>,
                directive::<AuthenticationMethods>,
                directive::<AddressFamily>,
                directive::<Banner>,
                directive::<KexAlgorithms>,
                directive::<AuthorizedKeysCommandUser>,
                directive::<Compression>,
                directive::<AuthorizedKeysFile>,
                directive::<AuthorizedPrincipalsCommandUser>,
                directive::<Ciphers>,
                directive::<ChrootDirectory>,
                directive::<ClientAliveInterval>,
                directive::<AuthorizedKeysCommand>,
                directive::<CASignatureAlgorithms>,
                directive::<AuthorizedPrincipalsCommand>,
            )),
            alt((
                directive::<DenyGroups>,
                directive::<AllowTcpForwarding>,
                directive::<AllowStreamLocalForwarding>,
            )),
        ))(input)
    }
}
