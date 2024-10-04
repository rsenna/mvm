# mvm

The Monolog Virtual Machine

## Installation

```bash
```

## Ubiquitous Language

- **Monolog**: The programming language.
- **Monolog Virtual Machine (MVM)**: The virtual machine that runs Monolog programs. Based on the RISC-V architecture.
- **Chomp**: A CPU word or instruction (neologism, avoids confusion with "word" in the RISC-V architecture).
- **Byte**: An 8-bit value in memory.
- **HalfWord**: A 16-bit value in memory.
- **Word**: A 32-bit value in memory.
- **DoubleWord**: A 64-bit value in memory.
- **QuadWord**: A 128-bit value in memory.

## Style Guide

```rust

// Top-level declarations //////////////////////////////////////////////////////

// External crates - not usually needed in Rust 2018+:
extern crate external_crate;

// Standard library imports:
use std::any::TypeId;
use std::module::path;

// External crate imports:
use external_crates::module::path;

// Local crate imports:
use crate::module::path;

// Top-level module declarations:
mod module_name;

// Global variables ////////////////////////////////////////////////////////////

// Constants:
const CONST_NAME: Type = value;

// Statics:
static STATIC_NAME: Type = value;

// Type definitions ////////////////////////////////////////////////////////////

// Type aliases:
type TypeAlias = Type;

// Traits:
trait TraitName {
    fn trait_method_name(&self) -> Type;
}

// Structs:
struct StructName {
    field_name: Type,
}

// Enums:
enum EnumName {
    SimpleVariant = 0,
    StructVariant { field_name: Type },
    TupleVariant(Type),
}

// Unions:
union UnionName {
    field_name1: Type,
    field_name2: Type,
    field_name3: Type,
}

// Macro definitions ///////////////////////////////////////////////////////////

// Declarative macros:
macro_rules! declarative_macro_name {
    ($l:tt) => { bar!($l); }
    () => {};
}

// Functional macros:
#[proc_macro]
fn functional_macro_name(input: TokenStream) -> TokenStream {
    let input = TokenStream::from(input);
    TokenStream::from(quote! {
        #input
    })
}

// Derive macros:
#[proc_macro_derive(DeriveMacroName)]
// Note: `derive_macro_impl` name is not relevant to the derive macro implementation or usage
fn derive_macro_impl(input: TokenStream) -> TokenStream {
    let input = TokenStream::from(input);
    TokenStream::from(quote! {
        #input
    })
}

// Attribute macros:
#[proc_macro_attribute]
fn attribute_macro_name(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = TokenStream::from(args);
    let input = TokenStream::from(input);
    TokenStream::from(quote! {
        #args
        #input
    })
}

// Macro calls /////////////////////////////////////////////////////////////////

declarative_macro_name!(foo);
functional_macro_name!(bar);

#[derive(DeriveMacroName)] // Should be used in the Type definition area, not here
struct SomeType;

#[attribute_macro_name]    // Should be used in the Type definition area, not here
struct AnotherType;

// Implementations /////////////////////////////////////////////////////////////

impl StructName {
    fn method_name(&self) -> Type { ... }
}

impl TraitName for StructName {
    fn trait_method_name(&self) -> Type { ... }
}

fn function_name(arg_name: Type) -> Type { ... }

// Inner modules ////////////////////////////////////////////////////////////////

mod inner_module_name_1 {
    use super::outer_module_name;

    // module contents
}

mod inner_module_name_n {
    // module contents
}

mod tests {
    #[test]
    fn test_name() {
        // test body
    }
}

// Visibility Order /////////////////////////////////////////////////////////////
// Using fn as an example, the order is the same for other items.

pub fn public_function() { ... }

pub(crate) fn visible_on_current_crate_function() { ... }

pub(super) fn visible_on_parent_module_and_siblings_function() { ... }

pub(self) fn visible_on_current_module_function() { ... }

fn private_function() { /* default visibility */ }

```
