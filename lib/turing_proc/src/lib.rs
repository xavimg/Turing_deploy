#![feature(once_cell)]
use std::{ops::Deref, lazy::Lazy, sync::{RwLock}};
use litrs::IntegerLit;
use proc_macro2::{Ident, Span, Literal, TokenStream};
use quote::{quote, ToTokens};
use syn::{parse_macro_input, parse_quote, FnArg, ImplItemMethod, Visibility, DeriveInput, parse::Parse, punctuated::Punctuated, Token, token::{Comma}, parenthesized, LitInt};
use crate::{maybee::MaybeeOfInput};

mod maybee;
const MAYBEES : Lazy<RwLock<Vec<Ident>>> = Lazy::new(|| {
    RwLock::new(Vec::with_capacity(1))
});

#[proc_macro_derive(Maybee)]
pub fn derive_maybee (input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let input = input.ident;

    let ident = MAYBEES;
    let ident = ident.read().unwrap();
    let ident = ident.iter();

    let target = quote! {
        #(
            impl #ident for #input {}
        )*
    };

    target.into()
}

#[proc_macro]
pub fn maybee_of (input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut input = parse_macro_input!(input as MaybeeOfInput);
    input.methods.iter_mut()
        .for_each(|x| {
            x.default = None;
        });

    let og_name = input.target.clone();
    let og_methods = input.methods.clone();
    let og_fn_name = og_methods.iter().map(|x| &x.sig.ident);

    input.methods.iter_mut()
        .for_each(|x| {
            x.sig.output = match &x.sig.output {
                syn::ReturnType::Default => parse_quote! { -> Option<()> },
                syn::ReturnType::Type(_, ty) => parse_quote! { -> Option<#ty> }
            };

            x.sig.ident = Ident::new(&format!("maybee_{}", x.sig.ident), Span::call_site())
        });

    let target_name = Ident::new(&format!("Maybee{}", input.target), proc_macro2::Span::call_site());
    let target_fns = input.methods.iter().map(|x| {
        let mut x = x.clone();
        x.default = Some(parse_quote!({
            None
        }));

        x
    });

    let target_fn_impl = input.methods.iter().cloned()
        .zip(og_fn_name)
        .map(|(x, og)| {
            let input = x.sig.inputs.clone().into_iter()
                .map(|y| {
                    match y {
                        FnArg::Receiver(y) => y.self_token.to_token_stream(),
                        FnArg::Typed(y) => match y.pat.deref() {
                            syn::Pat::Ident(y) => y.ident.to_token_stream(),
                            _ => panic!("Unexpected error")
                        }
                    }
                });

            let mut attrs = x.attrs.clone();
            attrs.push(parse_quote! { #[inline(always)] });

            ImplItemMethod {
                attrs,
                vis: Visibility::Inherited,
                defaultness: None,
                sig: x.sig,
                block: parse_quote! {{
                    Some(Self::#og(#(#input,)*))
                }}
            }
        });

    MAYBEES.write().unwrap().push(target_name.clone());
    let target = quote! {
        pub trait #target_name {
            #(#target_fns)*
        }

        impl<T: #og_name> #target_name for T {
            #(#target_fn_impl)*
        }
    };

    target.into()
}


// Composite
struct Composite {
    pub name: Ident,
    pub children: Punctuated<CompositeChild, Comma>,
}

struct CompositeChild {
    pub name: Ident,
    pub count: u8
}

impl Parse for CompositeChild {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse::<Ident>()?;
        input.parse::<Token![=]>()?;

        let count = input.parse::<Literal>()?;
        let count = IntegerLit::try_from(count).unwrap();
        
        Ok(Self {
            name,
            count: count.value::<u8>().expect("Input integer overflows")
        })
    }
}

impl Parse for Composite {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse::<Ident>()?;
        let children;
        parenthesized!(children in input);

        let children = Punctuated::<CompositeChild, Token![,]>::parse_terminated(&children)?;
        Ok(Self { name, children })
    }
}

#[proc_macro]
pub fn impl_composite (tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let Composite { name, children } = parse_macro_input!(tokens as Composite);
    
    let components = children.iter()
        .map(|CompositeChild { name, count }| quote! {(#name, #count)})
        .collect::<Punctuated<TokenStream, Token![,]>>();

    quote! {
        pub struct #name;

        impl Resource for #name {
            fn size (&self) -> f32 {
                todo!()
            }
        }

        impl Composite for #name {
            #[inline]
            fn components (&self) -> &[(dyn Resource, u8)] {
                #components
            }
        }
    }.into()
}