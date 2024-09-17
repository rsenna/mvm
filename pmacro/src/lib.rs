use proc_macro::TokenStream;
use syn::parse_macro_input;

mod enum_aliases;

use enum_aliases::derive_enum_alias_impl;

#[proc_macro_derive(EnumAliases)]
pub fn derive_enum_aliases(input: TokenStream) -> TokenStream {
    let parsed_input = parse_macro_input!(input);
    derive_enum_alias_impl(parsed_input).unwrap().into()
}
