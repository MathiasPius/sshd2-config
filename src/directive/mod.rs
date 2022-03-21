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
mod host_certificate;
mod host_key;
mod host_key_agent;
mod host_key_algorithms;
mod hostbased_accepted_algorithms;
mod hostbased_authentication;
mod hostbased_uses_name_from_packet_only;
mod ignore_rhosts;
mod ignore_user_known_hosts;
mod include;
mod ip_qo_s;
mod kbd_interactive_authentication;
mod kerberos_authentication;
mod kerberos_get_afs_token;
mod kerberos_or_local_passwd;
mod kerberos_ticket_cleanup;
mod kex_algorithms;
mod listen_address;
mod log_level;
mod log_verbose;
mod login_grace_time;
mod ma_cs;
mod match_block;
mod max_auth_tries;
mod max_sessions;
mod max_startups;
mod moduli_file;
mod password_authentication;
mod per_source_max_startups;
mod per_source_net_block_size;
mod permit_empty_passwords;
mod permit_listen;
mod permit_open;
mod permit_root_login;
mod permit_tty;
mod permit_tunnel;
mod permit_user_environment;
mod permit_user_rc;
mod pid_file;
mod port;
mod print_last_log;
mod print_motd;
mod pubkey_accepted_algorithms;
mod pubkey_auth_options;
mod pubkey_authentication;
mod r_domain;
mod rekey_limit;
mod revoked_keys;
mod security_key_provider;
mod set_env;
mod stream_local_bind_mask;
mod stream_local_bind_unlink;
mod strict_modes;
mod subsystem;
mod syslog_facility;
mod tcp_keep_alive;
mod trusted_user_ca_keys;
mod use_dns;
mod version_addendum;
mod x_11_display_offset;
mod x_11_forwarding;
mod x_11_use_localhost;
mod x_auth_location;

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
pub use host_certificate::*;
pub use host_key::*;
pub use host_key_agent::*;
pub use host_key_algorithms::*;
pub use hostbased_accepted_algorithms::*;
pub use hostbased_authentication::*;
pub use hostbased_uses_name_from_packet_only::*;
pub use ignore_rhosts::*;
pub use ignore_user_known_hosts::*;
pub use include::*;
pub use ip_qo_s::*;
pub use kbd_interactive_authentication::*;
pub use kerberos_authentication::*;
pub use kerberos_get_afs_token::*;
pub use kerberos_or_local_passwd::*;
pub use kerberos_ticket_cleanup::*;
pub use kex_algorithms::*;
pub use listen_address::*;
pub use log_level::*;
pub use log_verbose::*;
pub use login_grace_time::*;
pub use ma_cs::*;
pub use match_block::*;
pub use max_auth_tries::*;
pub use max_sessions::*;
pub use max_startups::*;
pub use moduli_file::*;
use nom::IResult;
use nom::{
    branch::alt,
    character::complete::line_ending,
    combinator::{eof, into},
    sequence::terminated,
};
pub use password_authentication::*;
pub use per_source_max_startups::*;
pub use per_source_net_block_size::*;
pub use permit_empty_passwords::*;
pub use permit_listen::*;
pub use permit_open::*;
pub use permit_root_login::*;
pub use permit_tty::*;
pub use permit_tunnel::*;
pub use permit_user_environment::*;
pub use permit_user_rc::*;
pub use pid_file::*;
pub use port::*;
pub use print_last_log::*;
pub use print_motd::*;
pub use pubkey_accepted_algorithms::*;
pub use pubkey_auth_options::*;
pub use pubkey_authentication::*;
pub use r_domain::*;
pub use rekey_limit::*;
pub use revoked_keys::*;
pub use security_key_provider::*;
pub use set_env::*;
pub use stream_local_bind_mask::*;
pub use stream_local_bind_unlink::*;
pub use strict_modes::*;
pub use subsystem::*;
pub use syslog_facility::*;
pub use tcp_keep_alive::*;
pub use trusted_user_ca_keys::*;
pub use use_dns::*;
pub use version_addendum::*;
pub use x_11_display_offset::*;
pub use x_11_forwarding::*;
pub use x_11_use_localhost::*;
pub use x_auth_location::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Directive<'a> {
    AcceptEnv(Vec<AcceptEnv<'a>>),
    AddressFamily(AddressFamily),
    AllowAgentForwarding(AllowAgentForwarding),
    AllowGroups(Vec<AllowGroups<'a>>),
    AllowStreamLocalForwarding(AllowStreamLocalForwarding),
    AllowTcpForwarding(AllowTcpForwarding),
    AllowUsers(Vec<AllowUsers<'a>>),
    AuthenticationMethods(Vec<Vec<AuthenticationMethods>>),
    AuthorizedKeysCommand(AuthorizedKeysCommand<'a>),
    AuthorizedKeysCommandUser(AuthorizedKeysCommandUser<'a>),
    AuthorizedKeysFile(AuthorizedKeysFile<'a>),
    AuthorizedPrincipalsCommand(AuthorizedPrincipalsCommand<'a>),
    AuthorizedPrincipalsCommandUser(AuthorizedPrincipalsCommandUser<'a>),
    AuthorizedPrincipalsFile(AuthorizedPrincipalsFile<'a>),
    Banner(Banner<'a>),
    CASignatureAlgorithms(Modifier<Vec<CASignatureAlgorithms>>),
    ChrootDirectory(ChrootDirectory<'a>),
    Ciphers(Modifier<Vec<Ciphers>>),
    ClientAliveCountMax(ClientAliveCountMax),
    ClientAliveInterval(ClientAliveInterval),
    Compression(Compression),
    DenyGroups(Vec<DenyGroups<'a>>),
    DenyUsers(Vec<DenyUsers<'a>>),
    DisableForwarding(DisableForwarding),
    ExposeAuthInfo(ExposeAuthInfo),
    FingerprintHash(FingerprintHash),
    ForceCommand(ForceCommand<'a>),
    GSSAPIAuthentication(GSSAPIAuthentication),
    GSSAPICleanupCredentials(GSSAPICleanupCredentials),
    GSSAPIStrictAcceptorCheck(GSSAPIStrictAcceptorCheck),
    GatewayPorts(GatewayPorts),
    HostCertificate(HostCertificate<'a>),
    HostKey(HostKey<'a>),
    HostKeyAgent(HostKeyAgent<'a>),
    HostKeyAlgorithms(Modifier<Vec<HostKeyAlgorithms>>),
    HostbasedAcceptedAlgorithms(Modifier<Vec<HostbasedAcceptedAlgorithms>>),
    HostbasedAuthentication(HostbasedAuthentication),
    HostbasedUsesNameFromPacketOnly(HostbasedUsesNameFromPacketOnly),
    IPQoS(Vec<IPQoS>),
    IgnoreRhosts(IgnoreRhosts),
    IgnoreUserKnownHosts(IgnoreUserKnownHosts),
    Include(Vec<Include<'a>>),
    KbdInteractiveAuthentication(KbdInteractiveAuthentication),
    KerberosAuthentication(KerberosAuthentication),
    KerberosGetAFSToken(KerberosGetAFSToken),
    KerberosOrLocalPasswd(KerberosOrLocalPasswd),
    KerberosTicketCleanup(KerberosTicketCleanup),
    KexAlgorithms(Modifier<Vec<KexAlgorithms>>),
    ListenAddress(Vec<ListenAddress<'a>>),
    LogLevel(LogLevel),
    LogVerbose(LogVerbose<'a>),
    LoginGraceTime(LoginGraceTime),
    MACs(Modifier<Vec<MACs>>),
    MatchBlock(MatchBlock<'a>),
    MaxAuthTries(MaxAuthTries),
    MaxSessions(MaxSessions),
    MaxStartups(MaxStartups<'a>),
    ModuliFile(ModuliFile<'a>),
    PasswordAuthentication(PasswordAuthentication),
    PerSourceMaxStartups(PerSourceMaxStartups<'a>),
    PerSourceNetBlockSize(PerSourceNetBlockSize<'a>),
    PermitEmptyPasswords(PermitEmptyPasswords),
    PermitListen(PermitListen<'a>),
    PermitOpen(PermitOpen<'a>),
    PermitRootLogin(PermitRootLogin),
    PermitTTY(PermitTTY),
    PermitTunnel(PermitTunnel),
    PermitUserEnvironment(PermitUserEnvironment<'a>),
    PermitUserRC(PermitUserRC),
    PidFile(PidFile<'a>),
    Port(Port),
    PrintLastLog(PrintLastLog),
    PrintMotd(PrintMotd),
    PubkeyAcceptedAlgorithms(Modifier<Vec<PubkeyAcceptedAlgorithms>>),
    PubkeyAuthOptions(Vec<PubkeyAuthOptions>),
    PubkeyAuthentication(PubkeyAuthentication),
    RDomain(RDomain<'a>),
    RekeyLimit(RekeyLimit<'a>),
    RevokedKeys(RevokedKeys<'a>),
    SecurityKeyProvider(SecurityKeyProvider<'a>),
    SetEnv(SetEnv<'a>),
    StreamLocalBindMask(StreamLocalBindMask<'a>),
    StreamLocalBindUnlink(StreamLocalBindUnlink),
    StrictModes(StrictModes),
    Subsystem(Subsystem<'a>),
    SyslogFacility(SyslogFacility),
    TCPKeepAlive(TCPKeepAlive),
    TrustedUserCAKeys(TrustedUserCAKeys<'a>),
    UseDNS(UseDNS),
    VersionAddendum(VersionAddendum<'a>),
    X11DisplayOffset(X11DisplayOffset),
    X11Forwarding(X11Forwarding),
    X11UseLocalhost(X11UseLocalhost),
    XAuthLocation(XAuthLocation<'a>),
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
                directive::<AddressFamily>,
                directive::<AllowAgentForwarding>,
                directive::<AllowGroups>,
                directive::<AllowStreamLocalForwarding>,
                directive::<AllowTcpForwarding>,
                directive::<AllowUsers>,
                directive::<AuthenticationMethods>,
                directive::<AuthorizedKeysCommand>,
                directive::<AuthorizedKeysCommandUser>,
                directive::<AuthorizedKeysFile>,
                directive::<AuthorizedPrincipalsCommand>,
                directive::<AuthorizedPrincipalsCommandUser>,
                directive::<AuthorizedPrincipalsFile>,
                directive::<Banner>,
                directive::<CASignatureAlgorithms>,
                directive::<ChrootDirectory>,
                directive::<Ciphers>,
                directive::<ClientAliveCountMax>,
                directive::<ClientAliveInterval>,
            )),
            alt((
                directive::<Compression>,
                directive::<DenyGroups>,
                directive::<DenyUsers>,
                directive::<DisableForwarding>,
                directive::<ExposeAuthInfo>,
                directive::<FingerprintHash>,
                directive::<ForceCommand>,
                directive::<GSSAPIAuthentication>,
                directive::<GSSAPICleanupCredentials>,
                directive::<GSSAPIStrictAcceptorCheck>,
                directive::<GatewayPorts>,
                directive::<HostCertificate>,
                directive::<HostKey>,
                directive::<HostKeyAgent>,
                directive::<HostKeyAlgorithms>,
                directive::<HostbasedAcceptedAlgorithms>,
                directive::<HostbasedAuthentication>,
                directive::<HostbasedUsesNameFromPacketOnly>,
                directive::<IPQoS>,
                directive::<IgnoreRhosts>,
            )),
            alt((
                directive::<IgnoreUserKnownHosts>,
                directive::<Include>,
                directive::<KbdInteractiveAuthentication>,
                directive::<KerberosAuthentication>,
                directive::<KerberosGetAFSToken>,
                directive::<KerberosOrLocalPasswd>,
                directive::<KerberosTicketCleanup>,
                directive::<KexAlgorithms>,
                directive::<ListenAddress>,
                directive::<LogLevel>,
                directive::<LogVerbose>,
                directive::<LoginGraceTime>,
                directive::<MACs>,
                directive::<MatchBlock>,
                directive::<MaxAuthTries>,
                directive::<MaxSessions>,
                directive::<MaxStartups>,
                directive::<ModuliFile>,
                directive::<PasswordAuthentication>,
                directive::<PerSourceMaxStartups>,
            )),
            alt((
                directive::<PerSourceNetBlockSize>,
                directive::<PermitEmptyPasswords>,
                directive::<PermitListen>,
                directive::<PermitOpen>,
                directive::<PermitRootLogin>,
                directive::<PermitTTY>,
                directive::<PermitTunnel>,
                directive::<PermitUserEnvironment>,
                directive::<PermitUserRC>,
                directive::<PidFile>,
                directive::<Port>,
                directive::<PrintLastLog>,
                directive::<PrintMotd>,
                directive::<PubkeyAcceptedAlgorithms>,
                directive::<PubkeyAuthOptions>,
                directive::<PubkeyAuthentication>,
                directive::<RDomain>,
                directive::<RekeyLimit>,
                directive::<RevokedKeys>,
                directive::<SecurityKeyProvider>,
            )),
            alt((
                directive::<SetEnv>,
                directive::<StreamLocalBindMask>,
                directive::<StreamLocalBindUnlink>,
                directive::<StrictModes>,
                directive::<Subsystem>,
                directive::<SyslogFacility>,
                directive::<TCPKeepAlive>,
                directive::<TrustedUserCAKeys>,
                directive::<UseDNS>,
                directive::<VersionAddendum>,
                directive::<X11DisplayOffset>,
                directive::<X11Forwarding>,
                directive::<X11UseLocalhost>,
                directive::<XAuthLocation>,
            )),
        ))(input)
    }
}
