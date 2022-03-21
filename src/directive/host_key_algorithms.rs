//! Generated file, do not edit by hand

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

#[doc = "Specifies the host key signature algorithms that the server offers. The default for this option is:"]
#[doc = "> ssh-ed25519-cert-v01@openssh.com, ecdsa-sha2-nistp256-cert-v01@openssh.com, ecdsa-sha2-nistp384-cert-v01@openssh.com, ecdsa-sha2-nistp521-cert-v01@openssh.com, sk-ssh-ed25519-cert-v01@openssh.com, sk-ecdsa-sha2-nistp256-cert-v01@openssh.com, rsa-sha2-512-cert-v01@openssh.com, rsa-sha2-256-cert-v01@openssh.com, ssh-ed25519, ecdsa-sha2-nistp256,ecdsa-sha2-nistp384,ecdsa-sha2-nistp521, sk-ssh-ed25519@openssh.com, sk-ecdsa-sha2-nistp256@openssh.com, rsa-sha2-512,rsa-sha2-256"]
#[doc = ""]
#[doc = "The list of available signature algorithms may also be obtained using 'ssh -Q HostKeyAlgorithms'."]
#[doc = "See also: [HostKeyAlgorithms](https://man.openbsd.org/sshd_config#HostKeyAlgorithms)"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HostKeyAlgorithms {
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

impl<'a> crate::Parse<'a> for HostKeyAlgorithms {
    type Output = Modifier<Vec<HostKeyAlgorithms>>;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded (tag ("HostKeyAlgorithms") , preceded (space1 , map (tuple ((opt (one_of ("+-")) , separated_list1 (tag (",") , preceded (space0 , alt ((alt ((value (HostKeyAlgorithms :: SshEd25519 , tag_no_case ("ssh-ed25519")) , value (HostKeyAlgorithms :: SshEd25519CertV01OpensshCom , tag_no_case ("ssh-ed25519-cert-v01@openssh.com")) , value (HostKeyAlgorithms :: SkSshEd25519OpensshCom , tag_no_case ("sk-ssh-ed25519@openssh.com")) , value (HostKeyAlgorithms :: SkSshEd25519CertV01OpensshCom , tag_no_case ("sk-ssh-ed25519-cert-v01@openssh.com")) , value (HostKeyAlgorithms :: SshRsa , tag_no_case ("ssh-rsa")) , value (HostKeyAlgorithms :: RsaSha2256 , tag_no_case ("rsa-sha2-256")) , value (HostKeyAlgorithms :: RsaSha2512 , tag_no_case ("rsa-sha2-512")) , value (HostKeyAlgorithms :: SshDss , tag_no_case ("ssh-dss")) , value (HostKeyAlgorithms :: EcdsaSha2Nistp256 , tag_no_case ("ecdsa-sha2-nistp256")) , value (HostKeyAlgorithms :: EcdsaSha2Nistp384 , tag_no_case ("ecdsa-sha2-nistp384")) , value (HostKeyAlgorithms :: EcdsaSha2Nistp521 , tag_no_case ("ecdsa-sha2-nistp521")) , value (HostKeyAlgorithms :: SkEcdsaSha2Nistp256OpensshCom , tag_no_case ("sk-ecdsa-sha2-nistp256@openssh.com")) , value (HostKeyAlgorithms :: WebauthnSkEcdsaSha2Nistp256OpensshCom , tag_no_case ("webauthn-sk-ecdsa-sha2-nistp256@openssh.com")) , value (HostKeyAlgorithms :: SshRsaCertV01OpensshCom , tag_no_case ("ssh-rsa-cert-v01@openssh.com")) , value (HostKeyAlgorithms :: RsaSha2256CertV01OpensshCom , tag_no_case ("rsa-sha2-256-cert-v01@openssh.com")) , value (HostKeyAlgorithms :: RsaSha2512CertV01OpensshCom , tag_no_case ("rsa-sha2-512-cert-v01@openssh.com")) , value (HostKeyAlgorithms :: SshDssCertV01OpensshCom , tag_no_case ("ssh-dss-cert-v01@openssh.com")) , value (HostKeyAlgorithms :: EcdsaSha2Nistp256CertV01OpensshCom , tag_no_case ("ecdsa-sha2-nistp256-cert-v01@openssh.com")) , value (HostKeyAlgorithms :: EcdsaSha2Nistp384CertV01OpensshCom , tag_no_case ("ecdsa-sha2-nistp384-cert-v01@openssh.com")) , value (HostKeyAlgorithms :: EcdsaSha2Nistp521CertV01OpensshCom , tag_no_case ("ecdsa-sha2-nistp521-cert-v01@openssh.com")) ,)) , alt ((value (HostKeyAlgorithms :: SkEcdsaSha2Nistp256CertV01OpensshCom , tag_no_case ("sk-ecdsa-sha2-nistp256-cert-v01@openssh.com")) ,)))))))) , Modifier :: from))) (input)
    }
}

impl<'a> From<Modifier<Vec<HostKeyAlgorithms>>> for crate::Directive<'a> {
    fn from(directive: Modifier<Vec<HostKeyAlgorithms>>) -> Self {
        crate::directive::Directive::HostKeyAlgorithms(directive)
    }
}
