use convert_case::{Case, Casing};
use proc_macro2::{Ident, Span};
use quote::quote;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

#[derive(Debug, Deserialize)]
struct Config {
    #[serde(borrow)]
    pub enums: HashMap<&'static str, ConfigOption>,
}

#[derive(Debug, Deserialize)]
pub enum DirectiveType {
    Single,
    Multiple,
    SingleCommaSeparated,
    MultipleCommaSeparated,
}

impl Default for DirectiveType {
    fn default() -> Self {
        DirectiveType::Single
    }
}

#[derive(Debug, Deserialize)]
struct ConfigOption {
    #[serde(borrow, default)]
    pub values: Vec<&'static str>,
    #[serde(borrow, default)]
    pub comment: Vec<&'static str>,
    #[serde(default)]
    pub directive_type: DirectiveType,
}

/// From rust-analyzer
pub fn project_root() -> PathBuf {
    Path::new(
        &env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_owned()),
    )
    .to_path_buf()
}

fn reformat(text: impl std::fmt::Display) -> std::io::Result<String> {
    let mut rustfmt = Command::new("rustfmt")
        .arg("--config-path")
        .arg(project_root().join("rustfmt.toml"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    write!(rustfmt.stdin.take().unwrap(), "{}", text)?;
    let output = rustfmt.wait_with_output()?;
    let stdout = String::from_utf8(output.stdout).unwrap();
    let preamble = "Generated file, do not edit by hand";
    Ok(format!("//! {}\n\n{}", preamble, stdout))
}

/// Sanitizes possible values so they can be used as enum variants
fn value_to_ident(value: &&'static str) -> Ident {
    let value = value.replace(['@', '.'], "-");

    // If the value starts with a number (such as 3des)
    // preface the ident with an x
    let value = if value.starts_with(|c: char| c.is_digit(10)) {
        format!("x{}", value)
    } else {
        value
    };

    Ident::new(&value.to_case(Case::Pascal), Span::call_site())
}

fn main() {
    let config: Config = toml::from_str(include_str!("sshd.toml")).unwrap();

    let includes = quote! {
        #[allow(unused_imports)]
        use crate::Directive;

        #[allow(unused_imports)]
        use std::borrow::Cow;
        #[allow(unused_imports)]
        use nom::{
            character::complete::{space0, space1, alphanumeric1},
            sequence::preceded,
            multi::many1,
            branch::alt,
            bytes::complete::{take_until, take_while1, tag_no_case},
            combinator::{map, value, not},
            IResult
        };
    };

    for (
        name,
        ConfigOption {
            values,
            directive_type,
            comment,
        },
    ) in config.enums
    {
        let name_ident = Ident::new(name, Span::call_site());

        // Sanitize the possible values so they can be used for Idents
        // i.e: aes128-gcm@openssh.com --> Aes128GcmOpensshCom
        let value_idents: Vec<Ident> = values.iter().map(value_to_ident).collect();

        let selector = if !values.is_empty() {
            // For options with known set of possible values, match strictly on these.
            let mapping = values
                .iter()
                .zip(value_idents.iter())
                .map(|(value, ident)| {
                    quote! {
                        value(#name_ident::#ident, tag_no_case(#value))
                    }
                })
                .collect::<Vec<_>>();

            quote! {
                #(#[doc = #comment])*
                #[doc = "See also: [sshd_config(5)](https://man7.org/linux/man-pages/man5/sshd_config.5.html)"]
                #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
                pub enum #name_ident {
                    #(#value_idents),*
                }

                impl<'a> crate::Parse<'a> for #name_ident {
                    type Output = Self;
                    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
                        preceded(
                            space1,
                            alt((
                                #(#mapping),*
                            ))
                        )(input)
                    }
                }
            }
        } else {
            quote! {
                #(#[doc = #comment])*
                #[doc = "See also: [sshd_config(5)](https://man7.org/linux/man-pages/man5/sshd_config.5.html)"]
                #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
                pub struct #name_ident<'a>(Cow<'a, str>);

                impl<'a> crate::Parse<'a> for #name_ident<'a> {
                    type Output = Self;
                    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
                        map(preceded(
                            space1,
                            take_while1(|c: char| !c.is_whitespace())
                        ), |value: &'a str| #name_ident(value.into()))(input)
                    }
                }
            }
        };

        let directive_name = format!("{}Directive", name);
        let directive_ident = Ident::new(&directive_name, Span::call_site());

        let lifetime = if values.is_empty() {
            quote! { <'a> }
        } else {
            quote! {}
        };

        let directive = match directive_type {
            DirectiveType::Single => quote! {
                #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
                pub struct #directive_ident #lifetime(#name_ident #lifetime);

                impl<'a> crate::Parse<'a> for #directive_ident #lifetime {
                    type Output = #directive_ident #lifetime;
                    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
                        map(preceded(
                            tag_no_case(#name),
                            #name_ident::parse
                        ), |value| #directive_ident(value))(input)
                    }
                }

                impl<'a> From<#directive_ident #lifetime> for Directive<'a> {
                    fn from(directive: #directive_ident #lifetime) -> Self {
                        Directive::#name_ident(directive)
                    }
                }
            },
            DirectiveType::Multiple => quote! {
                #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
                pub struct #directive_ident #lifetime(Vec<#name_ident #lifetime>);

                impl<'a> crate::Parse<'a> for #directive_ident #lifetime {
                    type Output = #directive_ident #lifetime;
                    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
                        map(preceded(
                            tag_no_case(#name),
                            many1(#name_ident::parse)
                        ), |value| #directive_ident(value))(input)
                    }
                }

                impl<'a> From<#directive_ident #lifetime> for Directive<'a> {
                    fn from(directive: #directive_ident #lifetime) -> Self {
                        Directive::#name_ident(directive)
                    }
                }
            },
            DirectiveType::SingleCommaSeparated => todo!(),
            DirectiveType::MultipleCommaSeparated => todo!(),
        };

        let tokens = quote! {
            #includes

            #directive

            #selector
        };

        std::fs::write(
            &project_root().join(format!(
                "src/{filename}.rs",
                filename = name.to_case(Case::Snake)
            )),
            reformat(tokens).unwrap(),
            //tokens.to_string(),
        )
        .unwrap();
    }

    println!("cargo:rerun-if-changed=sshd.toml");
    println!("cargo:rerun-if-changed=build.rs");
}
