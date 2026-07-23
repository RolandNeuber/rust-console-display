use proc_macro::TokenStream;
use quote::quote;
use syn::{
    DeriveInput,
    Expr,
    Ident,
    Token,
    parse::{
        Parse,
        ParseStream,
    },
    parse_macro_input,
    visit_mut::{
        VisitMut,
        visit_expr_mut,
    },
};

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
    impl #impl_generics const SingleWidget<T> for #name #ty_generics #where_clause {
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
    impl #impl_generics const TwoWidget<S, T> for #name #ty_generics #where_clause {
        fn children(&self) -> (&S, &T) {
            (&self.children.0, &self.children.1)
        }
        fn children_mut(&mut self) -> (&mut S, &mut T) {
            (&mut self.children.0, &mut self.children.1)
        }
    }))
}

struct ReplaceExpr {
    old: Ident,
    new: Expr,
    expr: Expr,
}

impl Parse for ReplaceExpr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // syntax: <ident> => <expr> ; <expr>
        let old: Ident = input.parse()?;
        input.parse::<Token![=>]>()?;
        let new: Expr = input.parse()?;
        let _: Token![;] = input.parse()?;
        let expr: Expr = input.parse()?;
        Ok(Self { old, new, expr })
    }
}

#[proc_macro]
pub fn replace(input: TokenStream) -> TokenStream {
    let ReplaceExpr { old, new, expr, .. } =
        parse_macro_input!(input as ReplaceExpr);
    let expr = replace_in_expr(expr, &old, &new);
    quote!(#expr).into()
}

fn replace_in_expr(mut expr: Expr, old: &Ident, new: &Expr) -> Expr {
    struct Replacer<'a> {
        old: &'a Ident,
        new: &'a Expr,
    }

    impl<'a> VisitMut for Replacer<'a> {
        fn visit_expr_mut(&mut self, node: &mut Expr) {
            if let Expr::Path(path) = node &&
                path.path.segments.len() == 1 &&
                path.path.segments[0].ident == *self.old
            {
                *node = self.new.clone();
                return;
            }

            visit_expr_mut(self, node);
        }
    }

    let mut replacer = Replacer { old, new };
    replacer.visit_expr_mut(&mut expr);
    expr
}
