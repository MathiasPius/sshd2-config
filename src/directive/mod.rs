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
    AuthorizedPrincipalsFile(AuthorizedPrincipalsFile<'a>),
    ChrootDirectory(ChrootDirectory<'a>),
    AuthorizedKeysCommandUser(AuthorizedKeysCommandUser<'a>),
    AllowTcpForwarding(AllowTcpForwarding),
    Ciphers(Modifier<Vec<Ciphers>>),
    CASignatureAlgorithms(Modifier<Vec<CASignatureAlgorithms>>),
    AllowAgentForwarding(AllowAgentForwarding),
    AllowStreamLocalForwarding(AllowStreamLocalForwarding),
    AuthenticationMethods(Vec<Vec<AuthenticationMethods>>),
    AddressFamily(AddressFamily),
    Banner(Banner<'a>),
    KexAlgorithms(Modifier<Vec<KexAlgorithms>>),
    AuthorizedKeysFile(AuthorizedKeysFile<'a>),
    AuthorizedPrincipalsCommandUser(AuthorizedPrincipalsCommandUser<'a>),
    AuthorizedKeysCommand(AuthorizedKeysCommand<'a>),
    AcceptEnv(Vec<AcceptEnv<'a>>),
    AuthorizedPrincipalsCommand(AuthorizedPrincipalsCommand<'a>),
    AllowUsers(Vec<AllowUsers<'a>>),
    AllowGroups(Vec<AllowGroups<'a>>),
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
            directive::<AuthorizedPrincipalsFile>,
            directive::<ChrootDirectory>,
            directive::<AuthorizedKeysCommandUser>,
            directive::<AllowTcpForwarding>,
            directive::<Ciphers>,
            directive::<CASignatureAlgorithms>,
            directive::<AllowAgentForwarding>,
            directive::<AllowStreamLocalForwarding>,
            directive::<AuthenticationMethods>,
            directive::<AddressFamily>,
            directive::<Banner>,
            directive::<KexAlgorithms>,
            directive::<AuthorizedKeysFile>,
            directive::<AuthorizedPrincipalsCommandUser>,
            directive::<AuthorizedKeysCommand>,
            directive::<AcceptEnv>,
            directive::<AuthorizedPrincipalsCommand>,
            directive::<AllowUsers>,
            directive::<AllowGroups>,
        ))(input)
    }
}
