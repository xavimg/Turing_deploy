extern crate proc_macro;
use std::ops::Deref;

use proc_macro::TokenStream;
use proc_macro2::{Group, TokenTree};
use quote::{quote};
use syn::{parse_macro_input, ExprReference, Expr, parse::Parse, Token, ItemFn, ExprClosure, ReturnType, Type, Stmt, ExprTuple, punctuated::Punctuated, token::Comma, Block, Pat, Path};

#[derive(Debug)]
struct CreateScene {
    renderer: Expr,
    title: Expr,
    width: Expr,
    height: Expr,
    on_start: ExprClosure
}

macro_rules! parse_exprs {
    ($input:expr, $($name:ident),+) => {
        $(
            let $name : Expr = $input.parse()?;
            $input.parse::<Token![,]>()?;
        )*
    };
}

macro_rules! deref {
    ($input:expr, $($name:ident),+) => {
        $(
            let $name = &$input.$name;
        )*
    };
}

impl Parse for CreateScene {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        parse_exprs!(input, renderer, title, width, height);
        let on_start : ExprClosure = input.parse()?;

        Ok(Self {
            renderer,
            title,
            width,
            height,
            on_start
        })
    }
}

#[proc_macro]
pub fn create_scene(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as CreateScene);
    deref!(input, renderer, title, width, height, on_start);
    
    let block = match on_start.body.deref() {
        Expr::Block(x) => &x.block,
        _ => panic!("No function block found")
    };

    let scene_input_token = quote! { let input = (#on_start)(lock); };
    let scene_input_type = scene_input_token.clone().into();    
    let scene_input_type = match parse_macro_input!(scene_input_type as Stmt) {
        Stmt::Local(local) => {
            let path = get_pat_types(&local.pat);
            let path : Punctuated<Path, Comma> = path.into_iter().map(|x| x.clone()).collect();
            path
        },
        _ => panic!("Invalid statement provided")
    };

    println!("{scene_input_type:?}");

    let target = quote! {
        let window = #renderer.create_instance(#title, #width, #height)?;
        let mut lock = window.lock().map_err(|e| e.to_string())?;
        #scene_input_token
        drop(lock);

        std::thread::spawn(|| {

        });
    };

    target.into()
}

fn get_pat_types (pat: &Pat) -> Punctuated<&Path, Comma> {
    match pat {
        Pat::Struct(strct) => Punctuated::<&Path, Comma>::from_iter([&strct.path].into_iter()),
        Pat::TupleStruct(strct) => Punctuated::<&Path, Comma>::from_iter([&strct.path].into_iter()),
        Pat::Tuple(tple) => tple.elems.iter().flat_map(|x| get_pat_types(x)).collect(),
        Pat::Reference(rf) => get_pat_types(&rf.pat),
        Pat::Ident(idnt) => 
        x => panic!("No types found: {x:?}")
    }
}