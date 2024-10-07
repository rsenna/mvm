// Copyright ©️ 2024 Rogério Senna. All rights reserved.
//
// Licensed under the EUPL, Version 1.2 or – as soon they will be approved by
// the European Commission - subsequent versions of the EUPL (the "Licence");
// You may not use this work except in compliance with the Licence.
// You may obtain a copy of the Licence at:
//
// https://joinup.ec.europa.eu/software/page/eupl
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the Licence is distributed on an "AS IS" basis,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the Licence for the specific language governing permissions and
// limitations under the Licence.
//

/// Aliases - can be used (mostly) as regular enum values:
#[macro_export]
macro_rules! enum_aliases {
    // Match the enum type followed by a block of alias mappings
    ($enumType:ident : $($sourceName:ident = $targetName:ident),* $(,)?) => {
        impl $enumType {
            $(
                pub const $sourceName: Self = Self::$targetName;
            )*
        }
    };
}

#[macro_export]
macro_rules! impl_common_bitfield_traits {
    ($($enumType:ident),* $(,)?) => {
        $(
            impl derive_more::Display for $enumType {
                fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                    fmt::Debug::fmt(self, f) // Just reuse Debug implementation
                }
            }

            impl From<crate::memory::Word> for $enumType {
                fn from(value: crate::memory::Word) -> Self {
                    $enumType::new_with_raw_value(value.value)
                }
            }
        )*
    };
}
