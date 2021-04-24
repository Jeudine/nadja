extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::Token;

#[proc_macro_attribute]
pub fn channel(attr: TokenStream, item: TokenStream) -> TokenStream {
    //TODO panic if the item is not a function
    let func = syn::parse_macro_input!(item as syn::ItemFn);
    let sig = &func.sig;
    let channel_name = &sig.ident;
    let inputs_type = sig.inputs.iter().map(|x|
                                            match x {
                                                syn::FnArg::Typed(p) => p.ty.clone(),
                                                _ => panic!("Not supported on types with `self`!"),
                                            }
                                           ).collect::<Vec<_>>();

    let inputs_name = sig.inputs.iter().map(|x|
                                            match x {
                                                syn::FnArg::Typed(p) => p.pat.clone(),
                                                _ => panic!("Not supported on types with `self`!"),
                                            }
                                           ).collect::<Vec<_>>();
    let output_type = match &sig.output {
        syn::ReturnType::Type(_, p) => p,
        _ => panic!("Not supported on functions without return types!"),
    };
    let body = &func.block;

    let gen = quote! {
        #[derive(new)]
        struct #channel_name<'a> {
            #(#inputs_name: &'a dyn Channel<#inputs_type>,)*
        }

        impl<'a> Channel<#output_type> for #channel_name<'a> {
            fn read(&self) -> #output_type {
                #(let #inputs_name = self.#inputs_name.read();)*
                #body
            }
        }
    };
    gen.into()
}

#[proc_macro_attribute]
pub fn module(attr: TokenStream, item: TokenStream) -> TokenStream {
    //TODO panic if the item is not a struct
    let struc = syn::parse_macro_input!(item as syn::ItemStruct);

    let module_name = &struc.ident;
    let sig_name = quote::format_ident!("{}Sig", module_name);
    let comb_name = quote::format_ident!("{}Comb", module_name);
    let proc_name = quote::format_ident!("{}Proc", module_name);

    let attrs: moduleParse = match(struc.fields) {
        syn::Fields::Named(p) => p.named.iter().fold(moduleParse::new(), |m, x| match(x.ty.clone()) {
            syn::Type::Path(p) => {p.path.segments;m}
            _ => panic!("Error while parsing module, wrong type!"),
        }
        ),
        _ => panic!("Error while parsing module!"),
    };

    let gen = quote! {
        #[derive(Default)]
        struct #sig_name {
        }

        struct #comb_name {
        }

        struct #proc_name {
        }
    };
    gen.into()
}

struct moduleParse {
    parameters: Vec<syn::Field>,
    inputs: Vec<syn::Field>,
    outputs: Vec<syn::Field>,
    channel_fns: Vec<syn::Field>,
    procs: Vec<syn::Field>,
    int_signs: Vec<syn::Field>
}

impl moduleParse {
    fn new() -> Self {
        Self {
            parameters: Vec::new(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            channel_fns: Vec::new(),
            procs: Vec::new(),
            int_signs: Vec::new()
        }
    }
}
