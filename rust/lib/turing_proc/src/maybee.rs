use proc_macro2::{Ident};
use syn::{parse::{Parse}, braced, TraitItemMethod};

#[derive(Debug, Clone)]
pub struct MaybeeOfInput {
    pub target: Ident,
    pub methods: Vec<TraitItemMethod>
}

impl Parse for MaybeeOfInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let target = input.parse::<Ident>()?;
        let br;
        braced!(br in input);

        let mut methods = Vec::with_capacity(1);
        loop {
            match br.parse::<TraitItemMethod>() {
                Ok(x) => methods.push(x),
                Err(_) => break,
            }
        }

        Ok(Self {
            target,
            methods
        })
    }
}