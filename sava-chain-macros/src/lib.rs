use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use syn::{bracketed, parenthesized, parse::Parse, parse_macro_input, ExprClosure, Ident, Token};

struct ChainingValidator {
    extractor_fn: ExprClosure,
    combinator_fn: ExprClosure,
    validator: Ident,
}

impl Parse for ChainingValidator {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let inner;
        parenthesized!(inner in input);

        let extractor_fn: ExprClosure = inner.parse()?;
        inner.parse::<Token![,]>()?;
        let combinator_fn: ExprClosure = inner.parse()?;
        inner.parse::<Token![,]>()?;
        let validator: Ident = inner.parse()?;

        Ok(Self {
            extractor_fn,
            combinator_fn,
            validator,
        })
    }
}

impl ChainingValidator {
    pub fn chaining_return_type_part(&self, to_validate: &Ident) -> TokenStream2 {
        let validator = self.validator.clone();

        quote::quote! {
            (
                ::sava_chain::FieldExtractorFn<#to_validate, <#validator as ::sava_chain::ChainExec>::Type>,
                ::sava_chain::FieldCombinatorFn<<#validator as ::sava_chain::ChainExec>::Type, #to_validate>,
            )
        }
    }

    pub fn chaining_return_part(&self) -> TokenStream2 {
        let extractor_fn = self.extractor_fn.clone();
        let combinator_fn = self.combinator_fn.clone();

        quote::quote! {
            (
                #extractor_fn,
                #combinator_fn,
            )
        }
    }
}

struct Chaining {
    to_validate: Ident,
    validators: Vec<ChainingValidator>,
}

impl Parse for Chaining {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut validators = Vec::new();

        let to_validate: Ident = input.parse()?;
        input.parse::<Token![:]>()?;

        let inner;
        bracketed!(inner in input);

        while !inner.is_empty() {
            validators.push(inner.parse()?)
        }

        if let (Err(e), true) = (input.parse::<Token![,]>(), input.peek(Ident)) {
            return Err(e);
        }

        Ok(Self {
            to_validate,
            validators,
        })
    }
}

impl Chaining {
    pub fn chaining_impl(&self) -> TokenStream2 {
        let Chaining {
            to_validate,
            validators,
        } = self;
        let chaining_ident = Ident::new(&format!("{}Validator", to_validate), Span::call_site());
        let return_type: Vec<TokenStream2> = validators
            .into_iter()
            .map(|valdator| ChainingValidator::chaining_return_type_part(valdator, to_validate))
            .collect();

        let return_value: Vec<TokenStream2> = validators
            .into_iter()
            .map(ChainingValidator::chaining_return_part)
            .collect();

        quote::quote! {
            struct #chaining_ident;
            impl #chaining_ident {
                pub fn chaining() -> (#(#return_type),*) {
                    (
                        #(#return_value),*
                    )
                }
            }
        }
    }

    pub fn chain_exec(self) -> TokenStream2 {
        quote::quote! {}
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
        result.extend(chaining.chaining_impl());
    }

    result.into()
}
