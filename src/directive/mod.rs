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
mod deny_users;
mod disable_forwarding;
mod expose_auth_info;
mod fingerprint_hash;
mod force_command;
mod gateway_ports;
mod gssapi_authentication;
mod gssapi_cleanup_credentials;
mod gssapi_strict_acceptor_check;
mod hostbased_accepted_algorithms;
mod hostbased_authentication;
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
pub use deny_users::*;
pub use disable_forwarding::*;
pub use expose_auth_info::*;
pub use fingerprint_hash::*;
pub use force_command::*;
pub use gateway_ports::*;
pub use gssapi_authentication::*;
pub use gssapi_cleanup_credentials::*;
pub use gssapi_strict_acceptor_check::*;
pub use hostbased_accepted_algorithms::*;
pub use hostbased_authentication::*;
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
    AcceptEnv(Vec<AcceptEnv<'a>>),
    Banner(Banner<'a>),
    AllowTcpForwarding(AllowTcpForwarding),
    GatewayPorts(GatewayPorts),
    ChrootDirectory(ChrootDirectory<'a>),
    Ciphers(Modifier<Vec<Ciphers>>),
    DisableForwarding(DisableForwarding),
    KexAlgorithms(Modifier<Vec<KexAlgorithms>>),
    CASignatureAlgorithms(Modifier<Vec<CASignatureAlgorithms>>),
    AuthorizedKeysFile(AuthorizedKeysFile<'a>),
    ClientAliveInterval(ClientAliveInterval),
    Compression(Compression),
    ExposeAuthInfo(ExposeAuthInfo),
    HostbasedAuthentication(HostbasedAuthentication),
    GSSAPIAuthentication(GSSAPIAuthentication),
    AddressFamily(AddressFamily),
    AuthorizedPrincipalsCommandUser(AuthorizedPrincipalsCommandUser<'a>),
    ForceCommand(ForceCommand<'a>),
    ClientAliveCountMax(ClientAliveCountMax),
    AuthorizedPrincipalsCommand(AuthorizedPrincipalsCommand<'a>),
    AllowAgentForwarding(AllowAgentForwarding),
    DenyGroups(DenyGroups<'a>),
    AuthorizedKeysCommandUser(AuthorizedKeysCommandUser<'a>),
    AllowStreamLocalForwarding(AllowStreamLocalForwarding),
    AuthorizedKeysCommand(AuthorizedKeysCommand<'a>),
    AuthorizedPrincipalsFile(AuthorizedPrincipalsFile<'a>),
    DenyUsers(DenyUsers<'a>),
    AuthenticationMethods(Vec<Vec<AuthenticationMethods>>),
    FingerprintHash(FingerprintHash),
    GSSAPIStrictAcceptorCheck(GSSAPIStrictAcceptorCheck),
    AllowGroups(Vec<AllowGroups<'a>>),
    AllowUsers(Vec<AllowUsers<'a>>),
    HostbasedAcceptedAlgorithms(Modifier<Vec<HostbasedAcceptedAlgorithms>>),
    GSSAPICleanupCredentials(GSSAPICleanupCredentials),
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
                directive::<AcceptEnv>,
                directive::<Banner>,
                directive::<AllowTcpForwarding>,
                directive::<GatewayPorts>,
                directive::<ChrootDirectory>,
                directive::<Ciphers>,
                directive::<DisableForwarding>,
                directive::<KexAlgorithms>,
                directive::<CASignatureAlgorithms>,
                directive::<AuthorizedKeysFile>,
                directive::<ClientAliveInterval>,
                directive::<Compression>,
                directive::<ExposeAuthInfo>,
                directive::<HostbasedAuthentication>,
                directive::<GSSAPIAuthentication>,
                directive::<AddressFamily>,
                directive::<AuthorizedPrincipalsCommandUser>,
                directive::<ForceCommand>,
                directive::<ClientAliveCountMax>,
                directive::<AuthorizedPrincipalsCommand>,
            )),
            alt((
                directive::<AllowAgentForwarding>,
                directive::<DenyGroups>,
                directive::<AuthorizedKeysCommandUser>,
                directive::<AllowStreamLocalForwarding>,
                directive::<AuthorizedKeysCommand>,
                directive::<AuthorizedPrincipalsFile>,
                directive::<DenyUsers>,
                directive::<AuthenticationMethods>,
                directive::<FingerprintHash>,
                directive::<GSSAPIStrictAcceptorCheck>,
                directive::<AllowGroups>,
                directive::<AllowUsers>,
                directive::<HostbasedAcceptedAlgorithms>,
                directive::<GSSAPICleanupCredentials>,
            )),
        ))(input)
    }
}
