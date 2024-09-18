use std::iter::once;

use anyhow::Context;
use proc_macro2::{Group, Ident, TokenStream, TokenTree};
use quote::quote;
use syn::{parse::Parser, punctuated::Punctuated, token::Comma};
use syn::{parse2, DeriveInput, Expr, LitStr, MetaNameValue, Result};

struct Pair(Ident, Ident);

type TokenPair = Punctuated<MetaNameValue, Comma>;

const DERIVE: &'static str = "derive";
const ENUM_ALIAS_IDENT: &'static str = "EnumAlias";

const CONTEXT_DERIVE_NOT_FOUND: &'static str = "Derive not found.";
const CONTEXT_LIT_STR_REQUIRED: &'static str = "Expected a literal string.";
const CONTEXT_CANNOT_PARSE_LIT_STR: &'static str = "Cannot parse literal string.";
const CONTEXT_INVALID_ALIAS_LIST: &'static str =
    "Expected a valid list of aliases for existing enum items.";
const CONTEXT_IDENT_REQUIRED: &'static str = "Expected an identifier.";

fn get_token<T, F>(token_tree_vec: &Vec<TokenTree>, getter: F) -> Option<&T>
where
    F: FnMut(&TokenTree) -> Option<&T>,
{
    token_tree_vec.into_iter().filter_map(getter).next()
}

// TODO: must return LitStr AND Ident (same stream)
pub fn extract_enum_alias_list(input: &DeriveInput) -> Result<(Ident, LitStr)> {
    let ident = input.ident.clone();

    let lit_str = input
        .attrs
        .iter()
        .filter(|&a| a.meta.path().is_ident(DERIVE))
        .map(|a| {
            let meta = &a.meta;
            let token_list = meta.require_list().unwrap();
            let inner_token_stream = token_list.tokens.clone();
            let tokens = inner_token_stream.into_iter().collect::<Vec<_>>();
            tokens
        })
        .map(|tokens| {
            let ident = get_token(&tokens, get_ident)?;

            if ident.to_string() != ENUM_ALIAS_IDENT {
                return None;
            }

            let group_stream = get_token(&tokens, get_group).map(|group| group.stream());
            group_stream
        })
        .filter(|group_stream| group_stream.is_some())
        .filter_map(|ts| parse2::<LitStr>(ts?).context(CONTEXT_LIT_STR_REQUIRED).ok())
        .next()
        .ok_or_else(|| syn::Error::new_spanned(&input, CONTEXT_DERIVE_NOT_FOUND));

    lit_str.map(|l| (ident, l))
}

// TODO: simplify, refactor, extract functions, etc.
pub fn derive_enum_alias_impl(input: DeriveInput) -> Result<TokenStream> {
    let (enum_ident, lit_str) = extract_enum_alias_list(&input)?;

    let consts = once(lit_str.clone()) // TODO: using once is a hack.
        .flat_map(|l: LitStr| {
            TokenPair::parse_terminated
                .parse_str(&l.value())
                .context(CONTEXT_CANNOT_PARSE_LIT_STR)
                .ok()
        })
        .flat_map(|p| p.into_iter())
        .flat_map(|mnv| parse_meta_name_value(&mnv))
        .map(|Pair(alias, variant)| {
            quote! {
                pub const #alias: Self = Self::#variant;
            }
        })
        .collect::<Vec<_>>();

    if consts.is_empty() {
        return Err(syn::Error::new_spanned(
            &lit_str,
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

fn get_ident(tt: &TokenTree) -> Option<&Ident> {
    if let TokenTree::Ident(ident) = tt {
        Some(ident)
    } else {
        None
    }
}

fn get_group(tt: &TokenTree) -> Option<&Group> {
    if let TokenTree::Group(group) = tt {
        Some(group)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::{parse_quote, DeriveInput};

    #[test]
    fn should_accept_a_list_of_aliases_for_existing_enum_items() {
        let input: DeriveInput = parse_quote! {
            #[derive(EnumAlias("Alias1 = Variant1, Alias2 = Variant2"))]
            enum TestEnum {
                Variant1,
                Variant2,
            }
        };

        let result = derive_enum_alias_impl(input).unwrap();

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
        let input: DeriveInput = parse_quote! {
            #[derive(EnumAlias("Alias1 = Variant1, Alias2 = Variant2, Alias3 = Variant3"))]
            enum TestEnum {
                Variant1,
                Variant2,
            }
        };

        let result = derive_enum_alias_impl(input).unwrap();

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
        let input: DeriveInput = parse_quote! {
            #[derive(EnumAlias("Alias1 = Variant1, Alias2 = Variant2, Alias3"))]
            enum TestEnum {
                Variant1,
                Variant2,
            }
        };

        let result = derive_enum_alias_impl(input);

        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn should_not_accept_an_empty_list_of_aliases() {
        let input: DeriveInput = parse_quote! {
            #[derive(EnumAlias(""))]
            enum TestEnum {
                Variant1,
            }
        };

        let result = derive_enum_alias_impl(input);

        assert_eq!(result.is_err(), true);
    }
}
