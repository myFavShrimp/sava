use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{parenthesized, parse::Parse, parse_macro_input, ExprClosure, Ident, Token};

struct Chaining {
    to_validate: Ident,
    extractor_fn: ExprClosure,
    combinator_fn: ExprClosure,
    validator: Ident,
}

impl Parse for Chaining {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let to_validate: Ident = input.parse()?;
        input.parse::<Token![:]>()?;

        let inner;
        parenthesized!(inner in input);

        let extractor_fn: ExprClosure = inner.parse()?;
        inner.parse::<Token![,]>()?;
        let combinator_fn: ExprClosure = inner.parse()?;
        inner.parse::<Token![,]>()?;
        let validator: Ident = inner.parse()?;

        if let (Err(e), true) = (input.parse::<Token![,]>(), input.peek(Ident)) {
            return Err(e);
        }

        Ok(Self {
            to_validate,
            extractor_fn,
            combinator_fn,
            validator,
        })
    }
}

impl Chaining {
    pub fn to_chain_exec(self) -> TokenStream2 {
        quote::quote! {
            const _: i8 = 12;
        }
    }
}

struct Chainings(Vec<Chaining>);

impl Parse for Chainings {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut chainings = Vec::new();

        while !input.is_empty() {
            chainings.push(input.parse()?)
        }

        Ok(Self(chainings))
    }
}

#[proc_macro]
pub fn chaining(input: TokenStream) -> TokenStream {
    let Chainings(chainings) = parse_macro_input!(input as Chainings);

    let mut result = TokenStream2::new();

    for chaining in chainings {
        result.extend(chaining.to_chain_exec());
    }

    result.into()
}
