use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{
    bracketed, parenthesized, parse::Parse, parse_macro_input, token::Paren, ExprClosure, Ident,
    ItemStruct, Token,
};

struct ChainingValidator {
    extractor: ExprClosure,
    combinator: ExprClosure,
    validator: Ident,
}

impl Parse for ChainingValidator {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let inner;
        parenthesized!(inner in input);

        let extractor: ExprClosure = inner.parse()?;
        inner.parse::<Token![,]>()?;
        let combinator: ExprClosure = inner.parse()?;
        inner.parse::<Token![,]>()?;
        let validator: Ident = inner.parse()?;

        if let (Err(e), true) = (input.parse::<Token![,]>(), input.peek(Paren)) {
            return Err(e);
        }

        Ok(Self {
            extractor,
            combinator,
            validator,
        })
    }
}

impl ChainingValidator {
    pub fn execute_part(&self) -> TokenStream2 {
        let ChainingValidator {
            validator,
            extractor,
            combinator,
        } = self;

        quote::quote! {
            let extractor: ::sava_chain::FieldExtractorFn<Self, <#validator as ::sava_chain::ChainExec>::Type> = #extractor;
            let combinator: ::sava_chain::FieldCombinatorFn<<#validator as ::sava_chain::ChainExec>::Type, Self> = #combinator;

            let extracted_field = extractor(&data);
            let chain_result = #validator::execute(extracted_field)?;
            combinator(&mut data, chain_result);
        }
    }
}

struct Chaining {
    error: Ident,
    chains: Vec<ChainingValidator>,
}

mod kw {
    syn::custom_keyword!(error);
    syn::custom_keyword!(chains);
}

impl Parse for Chaining {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut validators = Vec::new();

        input.parse::<kw::error>()?;
        input.parse::<Token![=]>()?;

        let error: Ident = input.parse()?;

        input.parse::<Token![,]>()?;
        input.parse::<kw::chains>()?;
        input.parse::<Token![=]>()?;

        let inner;
        bracketed!(inner in input);

        while !inner.is_empty() {
            validators.push(inner.parse()?)
        }

        if let (Err(e), true) = (input.parse::<Token![,]>(), input.peek(Paren)) {
            return Err(e);
        }

        Ok(Self {
            error,
            chains: validators,
        })
    }
}

impl Chaining {
    pub fn chain_exec(&self, name: Ident) -> TokenStream2 {
        let Chaining {
            error,
            chains: validators,
        } = self;
        let validation = self.validate();

        let mut execute = Vec::new();

        for validator in validators {
            execute.push(validator.execute_part());
        }

        quote::quote! {
            impl ::sava_chain::ChainExec for #name {
                type Type = Self;
                type Error = #error;

                fn execute(input: Self::Type) -> Result<Self::Type, Self::Error> {
                    #validation

                    let mut data = input;

                    #(#execute)*

                    Ok(data)
                }
            }
        }
    }

    pub fn validate(&self) -> TokenStream2 {
        let Chaining {
            error,
            chains: validators,
        } = self;

        let validator_idents: Vec<&Ident> = validators
            .iter()
            .map(|validator| &validator.validator)
            .collect();

        let assert_error = quote::quote_spanned! {
            error.span() => struct _AssertError where #error: std::error::Error;
        };

        let assert_error_from = quote::quote_spanned! {
            error.span() => struct _AssertErrorFrom
            where
                #error: #(
                    std::convert::From<<#validator_idents as ::sava_chain::ChainExec>::Error>
                )+*;
        };

        quote::quote! {
            #assert_error
            #assert_error_from
        }
    }
}

#[proc_macro_attribute]
pub fn sava(attr: TokenStream, item: TokenStream) -> TokenStream {
    let chaining = parse_macro_input!(attr as Chaining);
    let item = parse_macro_input!(item as ItemStruct);
    let item_name = item.ident.clone();

    let result = chaining.chain_exec(item_name);

    quote::quote! {
        #item
        #result
    }
    .into()
}
