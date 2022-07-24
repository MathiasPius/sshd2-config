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

/// Specifies the pathname of a directory to [chroot(2)](https://man.openbsd.org/chroot.2) to after authentication.
///
/// At session startup [sshd(8)](https://man.openbsd.org/sshd.8) checks that all components of the pathname are root-owned directories which are not writable by any other user or group.
/// After the chroot, [sshd(8)](https://man.openbsd.org/sshd.8) changes the working directory to the user's home directory.
/// Arguments to **ChrootDirectory** accept the tokens described in the TOKENS section.
/// The **ChrootDirectory** must contain the necessary files and directories to support the user's session.
/// For an interactive session this requires at least a shell, typically [sh(1)](https://man.openbsd.org/sh.1), and basic /dev nodes such as [null(4)](https://man.openbsd.org/null.4), [zero(4)](https://man.openbsd.org/zero.4), [stdin(4)](https://man.openbsd.org/stdin.4), [stdout(4)](https://man.openbsd.org/stdout.4), [stderr(4)](https://man.openbsd.org/stderr.4), and [tty(4)](https://man.openbsd.org/tty.4) devices.
/// For file transfer sessions using SFTP no additional configuration of the environment is necessary if the in-process sftp-server is used, though sessions which use logging may require /dev/log inside the chroot directory on some operating systems (see [sftp-server(8)](https://man.openbsd.org/sftp-server.8) for details).
///
/// For safety, it is very important that the directory hierarchy be prevented from modification by other processes on the system (especially those outside the jail).
/// Misconfiguration can lead to unsafe environments which [sshd(8)](https://man.openbsd.org/sshd.8) cannot detect.
///
/// The default is **none**, indicating not to [chroot(2)](https://man.openbsd.org/chroot.2).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChrootDirectory<'a>(Cow<'a, str>);
impl<'a> ChrootDirectory<'a> {
    pub fn new(value: &'a str) -> Self {
        Self(value.into())
    }
}
impl<'a> From<&'a str> for ChrootDirectory<'a> {
    fn from(value: &'a str) -> Self {
        ChrootDirectory(value.into())
    }
}

impl<'a> crate::ParseDirective<'a> for ChrootDirectory<'a> {
    type Output = ChrootDirectory<'a>;
    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
        preceded(
            tag("ChrootDirectory"),
            preceded(
                space1,
                map(
                    preceded(
                        space0,
                        take_while1(|c: char| !c.is_whitespace() && c != '#'),
                    ),
                    ChrootDirectory::from,
                ),
            ),
        )(input)
    }
}

impl<'a> From<ChrootDirectory<'a>> for crate::Directive<'a> {
    fn from(directive: ChrootDirectory<'a>) -> Self {
        crate::directive::Directive::ChrootDirectory(directive)
    }
}
