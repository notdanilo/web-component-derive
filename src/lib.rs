
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data};

#[proc_macro_derive(WebComponent)]
pub fn derive_web_component(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

    let expanded = if let Data::Struct(data) = input.data {
        let mut fields = quote!{};
        for field in &data.fields {
            if field.attrs.iter().find(|attribute| attribute.tokens.to_string() == "(skip)").is_none() {
                let ident = field.ident.as_ref().expect("Unnamed field not supported.");
                fields = quote! { #fields, #ident };
            }
        }

        quote! {
            impl WebComponentBinding for #struct_name {
                fn update_field(&mut self, name: &str, value: &str) {
                    update_field!(self, name, value #fields);
                }
            }

            web_component!(#struct_name);
        }
    } else {
        quote! {}
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
