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

/// Introduces a conditional block.
///
/// If all of the criteria on the **Match** line are satisfied, the keywords on the following lines override those set in the global section of the config file, until either another **Match** line or the end of the file.
/// If a keyword appears in multiple **Match** blocks that are satisfied, only the first instance of the keyword is applied.
/// The arguments to **Match** are one or more criteria-pattern pairs or the single token **All** which matches all criteria.
/// The available criteria are **User**, **Group**, **Host**, **LocalAddress**, **LocalPort**, **RDomain**, and **Address** (with **RDomain** representing the [rdomain(4)](https://man.openbsd.org/rdomain.4) on which the connection was received).
///
/// The match patterns may consist of single entries or comma-separated lists and may use the wildcard and negation operators described in the PATTERNS section of [ssh_config(5)](https://man.openbsd.org/ssh_config.5).
///
/// The patterns in an **Address** criteria may additionally contain addresses to match in CIDR address/masklen format, such as 192.0.2.0/24 or 2001:db8::/32.
/// Note that the mask length provided must be consistent with the address - it is an error to specify a mask length that is too long for the address or one with bits set in this host portion of the address.
/// For example, 192.0.2.0/33 and 192.0.2.0/8, respectively.
///
/// Only a subset of keywords may be used on the lines following a **Match** keyword.
/// Available keywords are **AcceptEnv**, **AllowAgentForwarding**, **AllowGroups**, **AllowStreamLocalForwarding**, **AllowTcpForwarding**, **AllowUsers**, **AuthenticationMethods**, **AuthorizedKeysCommand**, **AuthorizedKeysCommandUser**, **AuthorizedKeysFile**, **AuthorizedPrincipalsCommand**, **AuthorizedPrincipalsCommandUser**, **AuthorizedPrincipalsFile**, **Banner**, **CASignatureAlgorithms**, **ChrootDirectory**, **ClientAliveCountMax**, **ClientAliveInterval**, **DenyGroups**, **DenyUsers**, **DisableForwarding**, **ExposeAuthInfo**, **ForceCommand**, **GatewayPorts**, **GSSAPIAuthentication**, **HostbasedAcceptedAlgorithms**, **HostbasedAuthentication**, **HostbasedUsesNameFromPacketOnly**, **IgnoreRhosts**, **Include**, **IPQoS**, **KbdInteractiveAuthentication**, **KerberosAuthentication**, **LogLevel**, **MaxAuthTries**, **MaxSessions**, **PasswordAuthentication**, **PermitEmptyPasswords**, **PermitListen**, **PermitOpen**, **PermitRootLogin**, **PermitTTY**, **PermitTunnel**, **PermitUserRC**, **PubkeyAcceptedAlgorithms**, **PubkeyAuthentication**, **PubkeyAuthOptions**, **RekeyLimit**, **RevokedKeys**, **RDomain**, **SetEnv**, **StreamLocalBindMask**, **StreamLocalBindUnlink**, **TrustedUserCAKeys**, **X11DisplayOffset**, **X11Forwarding** and **X11UseLocalhost**.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MatchBlock<'a>(Cow<'a, str>);
impl<'a> MatchBlock<'a> {
    pub fn new(value: &'a str) -> Self {
        Self(value.into())
    }
}
impl<'a> From<&'a str> for MatchBlock<'a> {
    fn from(value: &'a str) -> Self {
        MatchBlock(value.into())
    }
}

impl<'a> crate::Parse<'a> for MatchBlock<'a> {
    type Output = MatchBlock<'a>;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("Match"),
            preceded(
                space1,
                map(
                    preceded(space0, take_while1(|c: char| !c.is_whitespace())),
                    MatchBlock::from,
                ),
            ),
        )(input)
    }
}

impl<'a> From<MatchBlock<'a>> for crate::Directive<'a> {
    fn from(directive: MatchBlock<'a>) -> Self {
        crate::directive::Directive::MatchBlock(directive)
    }
}

