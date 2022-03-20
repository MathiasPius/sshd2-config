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
    pub enums: HashMap<&'static str, Enum>,
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
struct Enum {
    #[serde(borrow)]
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
        use crate::Directive;
        use nom::{
            character::complete::space1,
            sequence::preceded,
            branch::alt,
            bytes::complete::tag_no_case,
            combinator::value,
            IResult
        };
    };

    for (name, definition) in config.enums {
        let filename = name.to_case(Case::Snake);
        let destination = project_root().join(format!("src/{filename}.rs"));

        let enum_ident = Ident::new(name, Span::call_site());

        let value_idents: Vec<Ident> = definition.values.iter().map(value_to_ident).collect();

        let parse_idents: Vec<_> = definition
            .values
            .iter()
            .zip(value_idents.iter())
            .map(|(value, ident)| {
                quote! {
                    value(#enum_ident::#ident, tag_no_case(#value))
                }
            })
            .collect();

        let parse_impl = match definition.directive_type {
            DirectiveType::Single => quote! {
                impl crate::Parse for #enum_ident {
                    type Output = Self;
                    fn parse(input: &str) -> IResult<&str, Self> {
                        preceded(
                            tag_no_case(#name),
                            preceded(space1,
                        alt((
                            #(#parse_idents),*
                        ))))(input)
                    }
                }
            },
            DirectiveType::Multiple => todo!(),
            DirectiveType::SingleCommaSeparated => todo!(),
            DirectiveType::MultipleCommaSeparated => todo!(),
        };

        let comments = definition.comment;
        let tokens = quote! {
            #includes

            #(#[doc = #comments])*
            #[doc = "See also: [sshd_config(5)](https://man7.org/linux/man-pages/man5/sshd_config.5.html)"]
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
            pub enum #enum_ident {
                #(#value_idents),*
            }

            #parse_impl

            impl Into<Directive> for #enum_ident {
                fn into(self) -> Directive {
                    Directive::#enum_ident(self)
                }
            }
        };

        std::fs::write(&destination, reformat(tokens).unwrap()).unwrap();
    }

    println!("cargo:rerun-if-changed=sshd.toml");
    println!("cargo:rerun-if-changed=build.rs");
}
