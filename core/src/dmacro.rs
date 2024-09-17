/// Aliases - can be used (mostly) as regular enum values:
#[macro_export] macro_rules! enum_aliases {
    // Match the enum type followed by a block of alias mappings
    ($enumType:ident : $($sourceName:ident = $targetName:ident),* $(,)?) => {
        impl $enumType {
            $(
                pub const $sourceName: Self = Self::$targetName;
            )*
        }
    };
}

#[macro_export]  macro_rules! impl_traits {
    ($($enumType:ident),* $(,)?) => {
        $(
            impl PartialEq for $enumType {
                fn eq(&self, other: &Self) -> bool {
                    self.0 == other.0
                }

                fn ne(&self, other: &Self) -> bool {
                    self.0 != other.0
                }
            }

            impl Display for $enumType {
                fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                    fmt::Debug::fmt(self, f) // Just reuse Debug implementation
                }
            }
        )*
    };
}
