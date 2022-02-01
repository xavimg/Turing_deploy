use std::ops::Deref;

use proc_macro2::{Ident, Span};
use quote::{quote, ToTokens};
use syn::{parse_macro_input, parse_str, ReturnType, parse_quote, Receiver, FnArg, Token, ImplItemMethod, VisPublic, Visibility};
use crate::maybee::MaybeeOfInput;
mod maybee;

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
    let target_fns = input.methods.iter();
    let target_fn_impl = input.methods.iter().cloned()
        .zip(og_fn_name)
        .map(|(x, og)| {
            let attrs = x.sig.inputs.clone().into_iter()
                .map(|y| {
                    match y {
                        FnArg::Receiver(y) => y.self_token.to_token_stream(),
                        FnArg::Typed(y) => match y.pat.deref() {
                            syn::Pat::Ident(y) => y.ident.to_token_stream(),
                            _ => panic!("Unexpected error")
                        }
                    }
                });

            ImplItemMethod {
                attrs: x.attrs,
                vis: Visibility::Inherited,
                defaultness: None,
                sig: x.sig,
                block: parse_quote! {{
                    Some(Self::#og(#(#attrs,)*))
                }}
            }
        });

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