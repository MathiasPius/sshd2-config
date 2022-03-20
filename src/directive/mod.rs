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
mod hostbased_uses_name_from_packet_only;
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
pub use hostbased_uses_name_from_packet_only::*;
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
    GatewayPorts(GatewayPorts),
    GSSAPICleanupCredentials(GSSAPICleanupCredentials),
    AuthorizedKeysCommand(AuthorizedKeysCommand<'a>),
    ChrootDirectory(ChrootDirectory<'a>),
    ForceCommand(ForceCommand<'a>),
    AllowStreamLocalForwarding(AllowStreamLocalForwarding),
    DenyUsers(DenyUsers<'a>),
    AllowAgentForwarding(AllowAgentForwarding),
    AllowTcpForwarding(AllowTcpForwarding),
    AddressFamily(AddressFamily),
    AuthenticationMethods(Vec<Vec<AuthenticationMethods>>),
    HostbasedAcceptedAlgorithms(Modifier<Vec<HostbasedAcceptedAlgorithms>>),
    AcceptEnv(Vec<AcceptEnv<'a>>),
    AuthorizedPrincipalsFile(AuthorizedPrincipalsFile<'a>),
    Banner(Banner<'a>),
    HostbasedUsesNameFromPacketOnly(HostbasedUsesNameFromPacketOnly),
    Compression(Compression),
    FingerprintHash(FingerprintHash),
    ExposeAuthInfo(ExposeAuthInfo),
    KexAlgorithms(Modifier<Vec<KexAlgorithms>>),
    HostbasedAuthentication(HostbasedAuthentication),
    AllowUsers(Vec<AllowUsers<'a>>),
    AuthorizedPrincipalsCommandUser(AuthorizedPrincipalsCommandUser<'a>),
    CASignatureAlgorithms(Modifier<Vec<CASignatureAlgorithms>>),
    Ciphers(Modifier<Vec<Ciphers>>),
    AuthorizedKeysFile(AuthorizedKeysFile<'a>),
    ClientAliveInterval(ClientAliveInterval),
    ClientAliveCountMax(ClientAliveCountMax),
    DisableForwarding(DisableForwarding),
    GSSAPIStrictAcceptorCheck(GSSAPIStrictAcceptorCheck),
    AllowGroups(Vec<AllowGroups<'a>>),
    AuthorizedPrincipalsCommand(AuthorizedPrincipalsCommand<'a>),
    AuthorizedKeysCommandUser(AuthorizedKeysCommandUser<'a>),
    GSSAPIAuthentication(GSSAPIAuthentication),
    DenyGroups(DenyGroups<'a>),
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
                directive::<GatewayPorts>,
                directive::<GSSAPICleanupCredentials>,
                directive::<AuthorizedKeysCommand>,
                directive::<ChrootDirectory>,
                directive::<ForceCommand>,
                directive::<AllowStreamLocalForwarding>,
                directive::<DenyUsers>,
                directive::<AllowAgentForwarding>,
                directive::<AllowTcpForwarding>,
                directive::<AddressFamily>,
                directive::<AuthenticationMethods>,
                directive::<HostbasedAcceptedAlgorithms>,
                directive::<AcceptEnv>,
                directive::<AuthorizedPrincipalsFile>,
                directive::<Banner>,
                directive::<HostbasedUsesNameFromPacketOnly>,
                directive::<Compression>,
                directive::<FingerprintHash>,
                directive::<ExposeAuthInfo>,
                directive::<KexAlgorithms>,
            )),
            alt((
                directive::<HostbasedAuthentication>,
                directive::<AllowUsers>,
                directive::<AuthorizedPrincipalsCommandUser>,
                directive::<CASignatureAlgorithms>,
                directive::<Ciphers>,
                directive::<AuthorizedKeysFile>,
                directive::<ClientAliveInterval>,
                directive::<ClientAliveCountMax>,
                directive::<DisableForwarding>,
                directive::<GSSAPIStrictAcceptorCheck>,
                directive::<AllowGroups>,
                directive::<AuthorizedPrincipalsCommand>,
                directive::<AuthorizedKeysCommandUser>,
                directive::<GSSAPIAuthentication>,
                directive::<DenyGroups>,
            )),
        ))(input)
    }
}
