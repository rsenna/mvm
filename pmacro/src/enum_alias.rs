/*
 * Copyright ©️ 2024 Rogério Senna. All rights reserved.
 *
 * Licensed under the EUPL, Version 1.2 or – as soon they will be approved by
 * the European Commission - subsequent versions of the EUPL (the "Licence");
 * You may not use this work except in compliance with the Licence.
 * You may obtain a copy of the Licence at:
 *
 * https://joinup.ec.europa.eu/software/page/eupl
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the Licence is distributed on an "AS IS" basis,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the Licence for the specific language governing permissions and
 * limitations under the Licence.
 *
 */

use anyhow::Context;
use itertools::Itertools;
use proc_macro2::{Ident, TokenStream, TokenTree};
use quote::quote;
use syn::{parse::Parser, punctuated::Punctuated, token::Comma};
use syn::{DeriveInput, Expr, MetaNameValue, Result};

struct Pair(Ident, Ident);

type TokenPair = Punctuated<MetaNameValue, Comma>;

const CONTEXT_CANNOT_PARSE_LIT_STR: &'static str = "Cannot parse literal string.";
const CONTEXT_IDENT_REQUIRED: &'static str = "Expected an identifier.";
const CONTEXT_INVALID_ALIAS_LIST: &'static str =
    "Expected a valid list of aliases for existing enum items.";
const ENUM_ALIAS_IDENT: &'static str = "enum_alias";

// TODO: simplify, refactor, extract functions, etc.
pub fn enum_alias_impl(metadata: DeriveInput, input: DeriveInput) -> Result<TokenStream> {
    eprintln!("metadata: {:?}", metadata);
    let enum_ident = input.ident.clone();
    eprintln!("enum_ident: {:?}", enum_ident);

    let enum_alias_list = metadata
        .attrs
        .iter()
        .map(|a| {
            eprintln!("a {:?}", a);
            let token_list = a.meta.require_list().unwrap();
            eprintln!("token_list {:?}", token_list);
            token_list
        })
        .filter_map(|ml| {
            eprintln!("ts {:?}", ml);

            if ml.path.segments.iter().any(|s| s.ident == ENUM_ALIAS_IDENT) {
                eprintln!("Found ENUM_ALIAS_IDENT");

                let inner_token_stream = ml.tokens.clone();
                eprintln!("inner_token_stream {:?}", inner_token_stream);
                Some(inner_token_stream.into_iter().collect_vec()) // TODO: TEMP
            } else {
                None
            }
        })
        .flatten()
        .filter_map(|ts| {
            let literal_opt = if let TokenTree::Literal(l) = ts {
                Some(l)
            } else {
                None
            };
            eprintln!("literal {:?}", literal_opt);
            literal_opt
        })
        .next();

    let consts = enum_alias_list
        .iter()
        .flat_map(|l| {
            eprintln!("l {:?}", l);
            let inner_args = l
                .to_string()
                .chars()
                .skip(1)
                .take(l.to_string().len() - 2)
                .collect::<String>();
            eprintln!("inner_args {:}", inner_args);
            let token_pair = TokenPair::parse_terminated
                .parse_str(inner_args.as_str())
                .context(CONTEXT_CANNOT_PARSE_LIT_STR)
                .ok();
            eprintln!("token_pair {:?}", token_pair);
            token_pair
        })
        .flatten()
        .flat_map(|mnv| parse_meta_name_value(&mnv))
        .map(|Pair(alias, variant)| {
            quote! {
                pub const #alias: Self = Self::#variant;
            }
        })
        .collect::<Vec<_>>();

    if consts.is_empty() {
        return Err(syn::Error::new_spanned(
            &enum_alias_list,
            CONTEXT_INVALID_ALIAS_LIST,
        ));
    }

    let expanded = quote! {
        impl #enum_ident {
            #(#consts)*
        }
    };

    Ok(expanded.into())
}

fn parse_meta_name_value(meta_name_value: &MetaNameValue) -> Option<Pair> {
    let MetaNameValue {
        path,
        eq_token: _eq_token,
        value,
    } = meta_name_value;

    let path = path.get_ident()?;
    let value = match value {
        Expr::Path(p) => p.path.get_ident()?,
        _ => panic!("{}", CONTEXT_IDENT_REQUIRED),
    };

    Some(Pair(path.clone(), value.clone()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::{parse_quote, DeriveInput};

    #[test]
    fn should_accept_a_list_of_aliases_for_existing_enum_items() {
        let metadata: DeriveInput = parse_quote! {
            #[EnumAlias("Alias1 = Variant1, Alias2 = Variant2")]
            struct Metadata;
        };

        let input: DeriveInput = parse_quote! {
            enum TestEnum {
                Variant1,
                Variant2,
            }
        };

        let result = enum_alias_impl(metadata, input).unwrap();

        let expected = quote! {
            impl TestEnum {
                pub const Alias1: Self = Self::Variant1;
                pub const Alias2: Self = Self::Variant2;
            }
        };

        assert_eq!(result.to_string(), expected.to_string());
    }

    // Note: this test is counterintuitive, as the enum items should exist. However, the test
    // is still useful, because the implementation does not check for the existence of the enum
    // items, and the actual error will be raised during compilation anyway.
    #[test]
    fn should_accept_a_list_of_aliases_for_non_existing_enum_items() {
        let metadata: DeriveInput = parse_quote! {
            #[EnumAlias("Alias1 = Variant1, Alias2 = Variant2, Alias3 = Variant3")]
            struct Metadata;
        };

        let input: DeriveInput = parse_quote! {
            enum TestEnum {
                Variant1,
                Variant2,
            }
        };

        let result = enum_alias_impl(metadata, input).unwrap();

        let expected = quote! {
            impl TestEnum {
                pub const Alias1: Self = Self::Variant1;
                pub const Alias2: Self = Self::Variant2;
                pub const Alias3: Self = Self::Variant3;
            }
        };

        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn should_not_accept_an_invalid_list_of_aliases() {
        let metadata = parse_quote! {
            #[EnumAlias("Alias1 = Variant1, Alias2 = Variant2, Alias3")]
            struct Metadata;
        };

        let input: DeriveInput = parse_quote! {
            enum TestEnum {
                Variant1,
                Variant2,
            }
        };

        let result = enum_alias_impl(metadata, input);

        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn should_not_accept_an_empty_list_of_aliases() {
        let metadata: DeriveInput = parse_quote! {
            #[EnumAlias("")]
            struct Metadata;
        };

        let input: DeriveInput = parse_quote! {
            enum TestEnum {
                Variant1,
            }
        };

        let result = enum_alias_impl(metadata, input);

        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn should_not_accept_a_complex_enum_item() {
        let metadata: DeriveInput = parse_quote! {
            #[EnumAlias("Alias1 = Variant1, Alias2 = Variant2")]
            struct Metadata;
        };

        let input: DeriveInput = parse_quote! {
            enum TestEnum {
                Variant1,
                Variant2 {
                    field: u32,
                },
            }
        };

        let token_stream_result = enum_alias_impl(metadata, input);
        let is_err = token_stream_result.is_err();

        eprintln!("{:?}", &token_stream_result.unwrap().to_string());

        assert_eq!(is_err, true);
    }

    #[test]
    fn should_not_be_applicable_to_non_enum_types() {
        // TODO implement
    }
}
