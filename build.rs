use convert_case::{Case, Casing};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::io::Write;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

#[derive(Debug, Deserialize)]
struct Config {
    #[serde(borrow)]
    pub directives: HashMap<&'static str, ConfigOption>,
}

#[derive(Debug, Deserialize)]
struct ConfigOption {
    #[serde(default)]
    pub name_override: Option<&'static str>,
    #[serde(default)]
    pub values: ValueFormat,
    #[serde(borrow, default)]
    pub comment: Vec<&'static str>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum TypedValue {
    Plain(&'static str),
    Commented {
        #[serde(borrow)]
        value: &'static str,
        #[serde(borrow, default)]
        comment: Option<&'static str>,
    },
}

impl From<TypedValue> for &'static str {
    fn from(value: TypedValue) -> Self {
        value.deref()
    }
}

impl Deref for TypedValue {
    type Target = &'static str;

    fn deref(&self) -> &Self::Target {
        match self {
            TypedValue::Plain(value) | TypedValue::Commented { value, .. } => value,
        }
    }
}

impl TypedValue {
    pub fn comment(&self) -> &'static str {
        match self {
            TypedValue::Plain(inner) => inner,
            TypedValue::Commented { comment, value } => comment.unwrap_or(value),
        }
    }

    pub fn as_ident(&self) -> Ident {
        let value = self.replace(|c: char| c.is_ascii_punctuation(), "-");

        // If the value starts with a number (such as 3des)
        // preface the ident with an x
        let value = if value.starts_with(|c: char| c.is_digit(10)) {
            format!("x{}", value)
        } else {
            value
        };
        Ident::new(&value.to_case(Case::Pascal), Span::call_site())
    }

    pub fn as_enum_entry(&self) -> TokenStream {
        let value = self.as_ident();
        let comment = self.comment();
        quote! {
            #[doc = #comment]
            #value
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(bound(deserialize = "'de: 'static"))]
#[serde(rename_all = "snake_case")]
pub enum ValueFormat {
    Wildcard,
    Integer,
    Typed(Vec<TypedValue>),
    CommaSeparated(Box<ValueFormat>),
    SpaceSeparated(Box<ValueFormat>),
    Modifier(Box<ValueFormat>),
    OneOf(HashMap<String, ValueFormat>),
}

impl Default for ValueFormat {
    fn default() -> Self {
        Self::Wildcard
    }
}

impl ValueFormat {
    pub fn output_type(&self, name_ident: &Ident) -> TokenStream {
        let lifetime = self.lifetime();

        match self {
            ValueFormat::Wildcard => quote! {
                #name_ident #lifetime
            },
            ValueFormat::Integer => quote! {
                #name_ident
            },
            ValueFormat::Typed(_) => quote! {
                #name_ident
            },
            ValueFormat::Modifier(inner) => {
                let inner_type = inner.output_type(name_ident);
                quote! { Modifier<#inner_type> }
            }
            ValueFormat::CommaSeparated(inner) | ValueFormat::SpaceSeparated(inner) => {
                let inner_type = inner.output_type(name_ident);
                quote! { Vec<#inner_type> }
            }
            ValueFormat::OneOf(_) => {
                quote! {
                    #name_ident
                }
            }
        }
    }

    fn lifetime(&self) -> TokenStream {
        if self.is_borrowed() {
            quote! { <'a> }
        } else {
            quote! {}
        }
    }

    fn derive_oneof_enum_member(&self, name_ident: &Ident, variant: &str) -> TokenStream {
        match self {
            ValueFormat::Wildcard => {
                let lifetime = self.lifetime();

                quote! {
                    #lifetime str
                }
            }
            ValueFormat::Integer => {
                quote! {
                    u64
                }
            }
            ValueFormat::Typed(_) => {
                let inner_name = format!("{}{}", name_ident, variant);
                let variant_type = Ident::new(&inner_name, name_ident.span());
                quote! {
                    #variant_type
                }
            }
            ValueFormat::CommaSeparated(inner) | ValueFormat::SpaceSeparated(inner) => {
                let inner_type = inner.derive_oneof_enum_member(name_ident, variant);
                quote! {
                    Vec<#inner_type>
                }
            }
            ValueFormat::Modifier(inner) => {
                let inner_type = inner.derive_oneof_enum_member(name_ident, variant);
                quote! {
                    Modifier<#inner_type>
                }
            }
            ValueFormat::OneOf(_) => {
                panic!("nested OneOf is illegal");
            }
        }
    }

    fn impl_struct(&self, original_name: &'static str, name_ident: &Ident) -> TokenStream {
        match self {
            ValueFormat::Wildcard => {
                quote! {
                    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
                    pub struct #name_ident<'a>(Cow<'a, str>);

                    impl<'a> #name_ident<'a> {
                        pub fn new(value: &'a str) -> Self {
                            Self(value.into())
                        }
                    }

                    impl<'a> From<&'a str> for #name_ident<'a> {
                        fn from(value: &'a str) -> Self {
                            #name_ident(value.into())
                        }
                    }
                }
            }
            ValueFormat::Integer => {
                quote! {
                    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
                    pub struct #name_ident(u64);

                    impl #name_ident {
                        pub fn new(value: u64) -> Self {
                            Self(value)
                        }
                    }

                    impl From<u64> for #name_ident {
                        fn from(value: u64) -> Self {
                            #name_ident(value)
                        }
                    }
                }
            }
            ValueFormat::Typed(values) => {
                let value_idents: Vec<TokenStream> =
                    values.iter().map(TypedValue::as_enum_entry).collect();
                quote! {
                    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
                    pub enum #name_ident {
                        #(#value_idents),*
                    }
                }
            }
            ValueFormat::OneOf(formats) => {
                let variants: Vec<TokenStream> = formats
                    .iter()
                    .map(|(key, value)| {
                        let variant = Ident::new(key, Span::call_site());
                        let inner_type = value.derive_oneof_enum_member(name_ident, key);
                        quote! {
                            #variant(#inner_type)
                        }
                    })
                    .collect();

                quote! {
                    pub enum #name_ident {
                        #(#variants),*
                    }
                }
            }
            ValueFormat::Modifier(inner)
            | ValueFormat::CommaSeparated(inner)
            | ValueFormat::SpaceSeparated(inner) => inner.impl_struct(original_name, name_ident),
        }
    }

    fn impl_into_directive(&self, name_ident: &Ident) -> TokenStream {
        let output_type = self.output_type(name_ident);

        quote! {
            impl<'a> From<#output_type> for crate::Directive<'a> {
                fn from(directive: #output_type) -> Self {
                    crate::directive::Directive::#name_ident(directive)
                }
            }
        }
    }

    fn parse_impl_inner(&self, name_ident: &Ident) -> TokenStream {
        match self {
            ValueFormat::Wildcard => quote! {
                map(preceded(space0, take_while1(|c: char| !c.is_whitespace())), #name_ident::from)
            },
            ValueFormat::Integer => quote! {
                map(preceded(space0, nom::character::complete::u64), #name_ident::from)
            },
            ValueFormat::Typed(values) => {
                let value_idents: Vec<Ident> = values.iter().map(TypedValue::as_ident).collect();

                // If there are more than 20 variants, we need to chunk it.
                let mapping: Vec<_> = if values.len() > 20 {
                    values
                        .iter()
                        .map(|value| **value)
                        .zip(value_idents.iter())
                        .collect::<Vec<_>>()
                        .as_slice()
                        .chunks(20)
                        .map(|chunk| {
                            let chunk = chunk.iter().map(|(value, ident)| {
                                quote! {
                                    value(#name_ident::#ident, tag_no_case(#value))
                                }
                            });

                            quote! {
                                alt((
                                    #(#chunk,)*
                                ))
                            }
                        })
                        .collect()
                } else {
                    values
                        .iter()
                        .map(|value| **value)
                        .zip(value_idents.iter())
                        .map(|(value, ident)| {
                            quote! {
                                value(#name_ident::#ident, tag_no_case(#value))
                            }
                        })
                        .collect()
                };

                quote! {
                    preceded(space0, alt((#(#mapping),*)))
                }
            }
            ValueFormat::OneOf(inner) => {
                let patterns: Vec<TokenStream> = inner
                    .iter()
                    .map(|(key, format)| {
                        if let ValueFormat::Typed(_) = format {
                            let inner_ident = Ident::new(&format!("{}{}", name_ident, key), name_ident.span());
                            let inner_parse = format.parse_impl_inner(&inner_ident);

                            quote! {
                                map(#inner_parse, #name_ident::from)
                            }
                        } else {
                            format.parse_impl_inner(name_ident)
                        }                    
                    })
                    .collect();

                quote! {
                    alt((#(#patterns),*,))
                }
            }
            ValueFormat::CommaSeparated(inner) => {
                let inner_pattern = inner.parse_impl_inner(name_ident);
                quote! {
                    separated_list1(tag(","), #inner_pattern)
                }
            }
            ValueFormat::SpaceSeparated(inner) => {
                let inner_pattern = inner.parse_impl_inner(name_ident);
                quote! {
                    separated_list1(tag(" "), #inner_pattern)
                }
            }
            ValueFormat::Modifier(inner) => {
                let inner_pattern = inner.parse_impl_inner(name_ident);
                quote! {
                    map(tuple((opt(one_of("+-^")), #inner_pattern)), Modifier::from)
                }
            }
        }
    }

    pub fn parse_impl(&self, original_name: &'static str, name_ident: &Ident) -> TokenStream {
        let inner_pattern = self.parse_impl_inner(name_ident);
        let output_type = self.output_type(name_ident);
        let lifetime = self.lifetime();
        let name = original_name.to_string();

        quote! {
            impl<'a> crate::ParseDirective<'a> for #name_ident #lifetime {
                type Output = #output_type;
                    fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
                        preceded(
                            tag(#name),
                            preceded(
                                space1,
                                #inner_pattern
                            )
                        )(input)
                    }
                }
        }
    }

    pub fn is_borrowed(&self) -> bool {
        match self {
            ValueFormat::Wildcard => true,
            ValueFormat::Integer | ValueFormat::Typed(_) | ValueFormat::OneOf(_) => false,
            ValueFormat::Modifier(inner)
            | ValueFormat::CommaSeparated(inner)
            | ValueFormat::SpaceSeparated(inner) => inner.is_borrowed(),
        }
    }
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

    if stdout.is_empty() {
        panic!("{}", text);
    }
    Ok(stdout)
}

fn main() {
    let config: Config = serde_json::from_str(include_str!("sshd.json")).unwrap();

    let includes = quote! {
        #[allow(unused_imports)]
        use crate::Modifier;
        #[allow(unused_imports)]
        use std::borrow::Cow;
        #[allow(unused_imports)]
        use nom::{
            character::complete::{space0, space1, alphanumeric1, one_of},
            sequence::{tuple, preceded},
            multi::{separated_list1, many1},
            branch::alt,
            bytes::complete::{tag, take_until, take_while1, tag_no_case},
            combinator::{map, value, not, opt, into},
            IResult
        };
    };

    let mut directive_types: Vec<_> = config
        .directives
        .iter()
        .map(
            |(
                name,
                ConfigOption {
                    values,
                    comment,
                    name_override,
                },
            )| {
                let joined = comment.join("\n").replace(". ", ".\n");
                let mut iter = joined.lines();
                let first = iter.next().unwrap();
                let rest = iter.map(|str| format!("\n/// {}", str)).fold("".to_string(), |mut i,j| {i.push_str(&*j); i});

                let comments =  format!("/// {}\n///{}", first, rest);

                let name_ident = Ident::new(name_override.unwrap_or(name), Span::call_site());
                let parse_impl = values.parse_impl(name, &name_ident);
                let structure = values.impl_struct(name, &name_ident);
                let into_directive = values.impl_into_directive(&name_ident);

                let tokens = format!(
                    r#"{includes}

                {comments}
                {structure}

                {parse_impl}

                {into_directive}
            "#
                );

                let source_code = format!(
                    "//! {comment}\n{code}\n",
                    comment = "This file has been automatically generated. Any changes made to it will be overwritten upon subsequent runs!",
                    code = reformat(tokens).unwrap()
                );

                std::fs::write(
                    &project_root().join(format!(
                        "src/directive/{filename}.rs",
                        filename = name_ident.to_string().to_case(Case::Snake)
                    )),
                    source_code,
                    //tokens.to_string(),
                )
                .unwrap();

                let output_type = values.output_type(&name_ident);
                (name_ident, output_type)
            },
        )
        .collect();
    directive_types.sort_by_key(|(name_ident, _)| name_ident.to_string());

    let enum_members = directive_types.iter().map(|(name_ident, output_type)| {
        quote! {
            #name_ident(#output_type)
        }
    });

    let filenames: Vec<_> = directive_types
        .iter()
        .map(|(name_ident, _)| {
            Ident::new(
                &name_ident.to_string().to_case(Case::Snake),
                Span::call_site(),
            )
        })
        .collect();

    let includes = quote! {
        #(mod #filenames;)*
    };

    let uses = quote! {
        use nom::IResult;
        use crate::{Modifier, ParseDirective};

        use nom::{
            branch::alt,
            combinator::{eof, into},
            character::complete::line_ending,
            sequence::terminated
        };

        #(pub use #filenames::*;)*
    };

    let name_idents: Vec<_> = directive_types
        .iter()
        .map(|(directive_ident, _)| quote! { #directive_ident })
        .collect();

    let structure = quote! {
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
        pub enum Directive<'a> {
            #(#enum_members),*
        }
    };

    // nom's alt() has a limti of 21 child parsers, so we have to chunk them to get around this
    let chunked_idents = name_idents.as_slice().chunks(20).map(|chunk| {
        quote! {
            alt((
                #(directive::<#chunk>),*
            ))
        }
    });

    let parse_impl = quote! {
        fn directive<'a, T: ParseDirective<'a>>(input: &'a str) -> IResult<&'a str, Directive>
        where
            <T as ParseDirective<'a>>::Output: Into<Directive<'a>>,
        {
            terminated(into(<T as ParseDirective<'a>>::parse), alt((line_ending, eof)))(input)
        }

        impl<'a> ParseDirective<'a> for Directive<'a> {
            type Output = Self;

            fn parse(input: &'a str) -> IResult<&'a str, Self::Output> {
                alt((
                    #(#chunked_idents),*
                ))(input)
            }
        }
    };

    let directive = format!(
        r#"
        {includes}

        {uses}

        {structure}

        {parse_impl}
    "#
    );

    std::fs::write(
        &project_root().join("src/directive/mod.rs"),
        reformat(directive).unwrap(),
    )
    .unwrap();

    println!("cargo:rerun-if-changed=sshd.json");
    println!("cargo:rerun-if-changed=build.rs");
}
