use proc_macro::TokenStream;
use syn::{parenthesized, parse::Parse, parse_macro_input, ExprClosure, Ident, Token};

struct Chaining;
impl Parse for Chaining {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let struct_to_validate: Ident = input.parse()?;
        input.parse::<Token![:]>()?;

        let inner;
        parenthesized!(inner in input);

        let extractor_fn: ExprClosure = inner.parse()?;
        inner.parse::<Token![,]>()?;
        let combinator_fn: ExprClosure = inner.parse()?;
        inner.parse::<Token![,]>()?;
        let validator: Ident = inner.parse()?;

        Ok(Self)
    }
}

#[proc_macro]
pub fn chaining(input: TokenStream) -> TokenStream {
    let _ = parse_macro_input!(input as Chaining);

    quote::quote! {
        const _: i8 = 15;
    }
    .into()
}
