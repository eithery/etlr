use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};


#[proc_macro_derive(DeserializeYaml)]
pub fn from_yaml_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl<'de> Deserialize<'de> for #name {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                let value = &Value::deserialize(deserializer)?;
                value.try_into().map_err(serde::de::Error::custom)
            }
        }
    };
    expanded.into()
}
