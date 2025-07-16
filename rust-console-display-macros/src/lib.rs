use proc_macro::TokenStream;
use quote::quote;
use syn::{
    DeriveInput,
    GenericParam,
    Generics,
    parse_macro_input,
    parse_quote,
};

/// Derives `StaticWidget` for a struct.
/// Sets the width and height in characters to the dimensions of the child element.
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

/// Derives `DynamicWidget` for a struct.
/// Sets the width and height in characters to the dimensions of the child element.
#[proc_macro_derive(DynamicWidget)]
pub fn derive_dynamic_widget(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let generics = add_static_widget_bound_to_t(input.generics);
    let (impl_generics, ty_generics, where_clause) =
        generics.split_for_impl();

    TokenStream::from(quote!(
    impl #impl_generics DynamicWidget for #name #ty_generics #where_clause {
        fn width_characters(&self) -> usize {
            self.child.width_characters()
        }
        fn height_characters(&self) -> usize {
            self.child.height_characters()
        }
        fn string_data(&self) -> Vec<Vec<DataCell>> {
            self.child.string_data()
        }
    }))
}

/// Derives `SingleWidget` for a struct.
/// Implements getter (+ mut) for the child element assuming the child is of type `T`.
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

        fn child(&self) -> Self::Borrowed<'_> {
            &self.child
        }

        fn child_mut(&mut self) -> Self::BorrowedMut<'_> {
            &mut self.child
        }
    }))
}

/// Derives `TwoWidget` for a struct.
/// Implements getter (+ mut) for the child elements assuming the children are of type `S` and `T` respectively.
#[proc_macro_derive(TwoWidget)]
pub fn derive_two_widget(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) =
        generics.split_for_impl();

    TokenStream::from(quote!(
    impl #impl_generics TwoWidget<S, T> for #name #ty_generics #where_clause {
        fn children(&self) -> (&S, &T) {
            (&self.children.0, &self.children.1)
        }
        fn children_mut(&mut self) -> (&mut S, &mut T) {
            (&mut self.children.0, &mut self.children.1)
        }
    }))
}

/// Adds a bound that restricts generics such that: `T: StaticWidget`.
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
