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

/// Specifies the signature algorithms that will be accepted for hostbased authentication as a list of comma-separated patterns.
///
/// Alternately if the specified list begins with a ‘+’ character, then the specified signature algorithms will be appended to the default set instead of replacing them.
/// If the specified list begins with a ‘-’ character, then the specified signature algorithms (including wildcards) will be removed from the default set instead of replacing them.
/// If the specified list begins with a ‘^’ character, then the specified signature algorithms will be placed at the head of the default set.
/// The default for this option is:
/// > ssh-ed25519-cert-v01@openssh.com, ecdsa-sha2-nistp256-cert-v01@openssh.com, ecdsa-sha2-nistp384-cert-v01@openssh.com, ecdsa-sha2-nistp521-cert-v01@openssh.com, sk-ssh-ed25519-cert-v01@openssh.com, sk-ecdsa-sha2-nistp256-cert-v01@openssh.com, rsa-sha2-512-cert-v01@openssh.com, rsa-sha2-256-cert-v01@openssh.com, ssh-ed25519, ecdsa-sha2-nistp256,ecdsa-sha2-nistp384,ecdsa-sha2-nistp521, sk-ssh-ed25519@openssh.com, sk-ecdsa-sha2-nistp256@openssh.com, rsa-sha2-512,rsa-sha2-256
///
///
/// The list of available signature algorithms may also be obtained using 'ssh -Q HostbasedAcceptedAlgorithms'.
/// This was formerly named HostbasedAcceptedKeyTypes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HostbasedAcceptedAlgorithms {
    #[doc = "ssh-ed25519"]
    SshEd25519,
    #[doc = "ssh-ed25519-cert-v01@openssh.com"]
    SshEd25519CertV01OpensshCom,
    #[doc = "sk-ssh-ed25519@openssh.com"]
    SkSshEd25519OpensshCom,
    #[doc = "sk-ssh-ed25519-cert-v01@openssh.com"]
    SkSshEd25519CertV01OpensshCom,
    #[doc = "ssh-rsa"]
    SshRsa,
    #[doc = "rsa-sha2-256"]
    RsaSha2256,
    #[doc = "rsa-sha2-512"]
    RsaSha2512,
    #[doc = "ssh-dss"]
    SshDss,
    #[doc = "ecdsa-sha2-nistp256"]
    EcdsaSha2Nistp256,
    #[doc = "ecdsa-sha2-nistp384"]
    EcdsaSha2Nistp384,
    #[doc = "ecdsa-sha2-nistp521"]
    EcdsaSha2Nistp521,
    #[doc = "sk-ecdsa-sha2-nistp256@openssh.com"]
    SkEcdsaSha2Nistp256OpensshCom,
    #[doc = "webauthn-sk-ecdsa-sha2-nistp256@openssh.com"]
    WebauthnSkEcdsaSha2Nistp256OpensshCom,
    #[doc = "ssh-rsa-cert-v01@openssh.com"]
    SshRsaCertV01OpensshCom,
    #[doc = "rsa-sha2-256-cert-v01@openssh.com"]
    RsaSha2256CertV01OpensshCom,
    #[doc = "rsa-sha2-512-cert-v01@openssh.com"]
    RsaSha2512CertV01OpensshCom,
    #[doc = "ssh-dss-cert-v01@openssh.com"]
    SshDssCertV01OpensshCom,
    #[doc = "ecdsa-sha2-nistp256-cert-v01@openssh.com"]
    EcdsaSha2Nistp256CertV01OpensshCom,
    #[doc = "ecdsa-sha2-nistp384-cert-v01@openssh.com"]
    EcdsaSha2Nistp384CertV01OpensshCom,
    #[doc = "ecdsa-sha2-nistp521-cert-v01@openssh.com"]
    EcdsaSha2Nistp521CertV01OpensshCom,
    #[doc = "sk-ecdsa-sha2-nistp256-cert-v01@openssh.com"]
    SkEcdsaSha2Nistp256CertV01OpensshCom,
}

impl<'a> crate::ParseDirective<'a> for HostbasedAcceptedAlgorithms {
    type Output = Modifier<Vec<HostbasedAcceptedAlgorithms>>;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded (tag ("HostbasedAcceptedAlgorithms") , preceded (space1 , map (tuple ((opt (one_of ("+-^")) , separated_list1 (tag (",") , preceded (space0 , alt ((alt ((value (HostbasedAcceptedAlgorithms :: SshEd25519 , tag_no_case ("ssh-ed25519")) , value (HostbasedAcceptedAlgorithms :: SshEd25519CertV01OpensshCom , tag_no_case ("ssh-ed25519-cert-v01@openssh.com")) , value (HostbasedAcceptedAlgorithms :: SkSshEd25519OpensshCom , tag_no_case ("sk-ssh-ed25519@openssh.com")) , value (HostbasedAcceptedAlgorithms :: SkSshEd25519CertV01OpensshCom , tag_no_case ("sk-ssh-ed25519-cert-v01@openssh.com")) , value (HostbasedAcceptedAlgorithms :: SshRsa , tag_no_case ("ssh-rsa")) , value (HostbasedAcceptedAlgorithms :: RsaSha2256 , tag_no_case ("rsa-sha2-256")) , value (HostbasedAcceptedAlgorithms :: RsaSha2512 , tag_no_case ("rsa-sha2-512")) , value (HostbasedAcceptedAlgorithms :: SshDss , tag_no_case ("ssh-dss")) , value (HostbasedAcceptedAlgorithms :: EcdsaSha2Nistp256 , tag_no_case ("ecdsa-sha2-nistp256")) , value (HostbasedAcceptedAlgorithms :: EcdsaSha2Nistp384 , tag_no_case ("ecdsa-sha2-nistp384")) , value (HostbasedAcceptedAlgorithms :: EcdsaSha2Nistp521 , tag_no_case ("ecdsa-sha2-nistp521")) , value (HostbasedAcceptedAlgorithms :: SkEcdsaSha2Nistp256OpensshCom , tag_no_case ("sk-ecdsa-sha2-nistp256@openssh.com")) , value (HostbasedAcceptedAlgorithms :: WebauthnSkEcdsaSha2Nistp256OpensshCom , tag_no_case ("webauthn-sk-ecdsa-sha2-nistp256@openssh.com")) , value (HostbasedAcceptedAlgorithms :: SshRsaCertV01OpensshCom , tag_no_case ("ssh-rsa-cert-v01@openssh.com")) , value (HostbasedAcceptedAlgorithms :: RsaSha2256CertV01OpensshCom , tag_no_case ("rsa-sha2-256-cert-v01@openssh.com")) , value (HostbasedAcceptedAlgorithms :: RsaSha2512CertV01OpensshCom , tag_no_case ("rsa-sha2-512-cert-v01@openssh.com")) , value (HostbasedAcceptedAlgorithms :: SshDssCertV01OpensshCom , tag_no_case ("ssh-dss-cert-v01@openssh.com")) , value (HostbasedAcceptedAlgorithms :: EcdsaSha2Nistp256CertV01OpensshCom , tag_no_case ("ecdsa-sha2-nistp256-cert-v01@openssh.com")) , value (HostbasedAcceptedAlgorithms :: EcdsaSha2Nistp384CertV01OpensshCom , tag_no_case ("ecdsa-sha2-nistp384-cert-v01@openssh.com")) , value (HostbasedAcceptedAlgorithms :: EcdsaSha2Nistp521CertV01OpensshCom , tag_no_case ("ecdsa-sha2-nistp521-cert-v01@openssh.com")) ,)) , alt ((value (HostbasedAcceptedAlgorithms :: SkEcdsaSha2Nistp256CertV01OpensshCom , tag_no_case ("sk-ecdsa-sha2-nistp256-cert-v01@openssh.com")) ,)))))))) , Modifier :: from))) (input)
    }
}

impl<'a> From<Modifier<Vec<HostbasedAcceptedAlgorithms>>> for crate::Directive<'a> {
    fn from(directive: Modifier<Vec<HostbasedAcceptedAlgorithms>>) -> Self {
        crate::directive::Directive::HostbasedAcceptedAlgorithms(directive)
    }
}
