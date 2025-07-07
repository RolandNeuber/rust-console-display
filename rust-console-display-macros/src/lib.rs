use proc_macro::TokenStream;
use quote::quote;
use syn::{
    DeriveInput,
    GenericParam,
    Generics,
    parse_macro_input,
    parse_quote,
};

#[proc_macro_derive(StaticWidget)]
pub fn derive_static_widget(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let generics = add_static_widget_bound_to_t(input.generics);
    let (impl_generics, ty_generics, where_clause) =
        generics.split_for_impl();

    TokenStream::from(quote!(
    impl #impl_generics StaticWidget for #name #ty_generics #where_clause {
        const WIDTH_CHARACTERS: usize = <T as StaticWidget>::WIDTH_CHARACTERS;
        const HEIGHT_CHARACTERS: usize = <T as StaticWidget>::HEIGHT_CHARACTERS;
    }))
}

#[proc_macro_derive(DynamicWidget)]
pub fn derive_dynamic_widget(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let generics = add_static_widget_bound_to_t(input.generics);
    let (impl_generics, ty_generics, where_clause) =
        generics.split_for_impl();

    TokenStream::from(quote!(
    impl #impl_generics DynamicWidget for #name #ty_generics #where_clause {
        fn get_width_characters(&self) -> usize {
            self.child.get_width_characters()
        }
        fn get_height_characters(&self) -> usize {
            self.child.get_height_characters()
        }
    }))
}

#[proc_macro_derive(SingleWidget)]
pub fn derive_single_widget(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) =
        generics.split_for_impl();

    TokenStream::from(quote!(
    impl #impl_generics SingleWidget<T> for #name #ty_generics #where_clause {
        type Borrowed<'a>
            = &'a T
        where
            T: 'a,
            Self: 'a;

        type BorrowedMut<'a>
            = &'a mut T
        where
            T: 'a,
            Self: 'a;

        fn get_child(&self) -> Self::Borrowed<'_> {
            &self.child
        }

        fn get_child_mut(&mut self) -> Self::BorrowedMut<'_> {
            &mut self.child
        }
    }))
}

#[proc_macro_derive(TwoWidget)]
pub fn derive_two_widget(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) =
        generics.split_for_impl();

    TokenStream::from(quote!(
    impl #impl_generics TwoWidget<S, T> for #name #ty_generics #where_clause {
        fn get_children(&self) -> (&S, &T) {
            (&self.children.0, &self.children.1)
        }
        fn get_children_mut(&mut self) -> (&mut S, &mut T) {
            (&mut self.children.0, &mut self.children.1)
        }
    }))
}

fn add_static_widget_bound_to_t(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(type_param) = param &&
            type_param.ident == "T"
        {
            type_param.bounds.push(parse_quote!(StaticWidget));
        }
    }
    generics
}
