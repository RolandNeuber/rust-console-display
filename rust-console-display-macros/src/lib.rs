use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Fields};

#[proc_macro_derive(StaticWidget)]
pub fn derive_static_widget(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    if let syn::Data::Struct(ref data) = input.data {
        // const WIDTH_CHARACTERS: usize = T::WIDTH_CHARACTERS;
        if let Fields::Named(ref fields) = data.fields {
            let name = input.ident;
            let generics = input.generics;
            let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

            return TokenStream::from(quote!(
            impl #impl_generics StaticWidget for #name #ty_generics #where_clause {
                const WIDTH_CHARACTERS: usize = T::WIDTH_CHARACTERS;
                const HEIGHT_CHARACTERS: usize = T::HEIGHT_CHARACTERS;
            }));
        }
    }

    TokenStream::from(
        syn::Error::new(
            input.ident.span(),
            "Only structs with named fields can derive `StaticWidget`"
        ).to_compile_error()
    )
}