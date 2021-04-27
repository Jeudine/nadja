extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_attribute]
pub fn channel(_: TokenStream, item: TokenStream) -> TokenStream {
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
pub fn module(_: TokenStream, item: TokenStream) -> TokenStream {
    let struc = syn::parse_macro_input!(item as syn::ItemStruct);
    let module_name = &struc.ident;
    let sig_name = quote::format_ident!("{}Sig", module_name);
    let comb_name = quote::format_ident!("{}Comb", module_name);
    let i_name = quote::format_ident!("{}Input", module_name);
    let proc_name = quote::format_ident!("{}Proc", module_name);

    let attrs: ModuleParse = match struc.fields {
        syn::Fields::Named(p) => p.named.iter().fold(ModuleParse::new(), |mut m, x| {
            match x.ty {
                syn::Type::Path(ref p) => {
                    let ps = p.path.segments.first().unwrap();
                    match ps.ident.to_string().as_str() {
                        "Param" => {
                            m.params_type.push(
                                match &ps.arguments {
                                    syn::PathArguments::AngleBracketed(p) => p.args.first().unwrap().clone(),
                                    _ => panic!("Error, `<` expected!"),
                                });
                            m.params_name.push(x.ident.as_ref().unwrap().clone());
                        },
                        "Input" => {
                            m.inputs_type.push(x.ty.clone());
                            m.inputs_name.push(x.ident.as_ref().unwrap().clone());
                        },
                        "Output" => {
                            m.outputs_type.push(x.ty.clone());
                            m.outputs_name.push(x.ident.as_ref().unwrap().clone());
                        },
                        "RegRst" | "Reg" => {
                            m.procs_type.push(
                                match &ps.arguments {
                                    syn::PathArguments::AngleBracketed(p) => p.args.first().unwrap().clone(),
                                    _ => panic!("Error, `<` expected!"),
                                });
                            m.procs_name.push(x.ident.as_ref().unwrap().clone());
                            m.procs_struc.push(ps.ident.clone());
                        },
                        _ => m.channel_fns.push(x.clone()),
                    };
                    m
                }
                _ => panic!("Error, wrong type!"),
            }
        }
        ),
        _ => panic!("Error, named field expected!"),
    };

    let params_type = &attrs.params_type;
    let params_name = &attrs.params_name;
    let inputs_type = &attrs.inputs_type;
    let inputs_name = &attrs.inputs_name;
    let outputs_type = &attrs.outputs_type;
    let outputs_name = &attrs.outputs_name;
    let channel_fns = &attrs.channel_fns;
    let procs_name = &attrs.procs_name;
    let procs_type = &attrs.procs_type;
    let procs_struc = &attrs.procs_struc;

    let gen = quote! {
        #[derive(Default)]
        struct #sig_name {
            #(pub #procs_name: Signal<#procs_type>,)*
        }

        struct #i_name<'a> {
            #(pub #params_name: #params_type,)*
            #(#inputs_name: &'a #inputs_type,)*
        }

        struct #comb_name<'a> {
            #(#channel_fns<'a>,)*
            #(pub #outputs_name: &'a #outputs_type,)*
        }

        struct #proc_name<'a> {
            #(#procs_name: #procs_struc<'a, #procs_type>,)*
        }
    };
    gen.into()
}

#[proc_macro_attribute]
pub fn comb(_: TokenStream, item: TokenStream) -> TokenStream {
    let func = syn::parse_macro_input!(item as syn::ItemFn);
    let ident = &func.sig.ident;
    let comb_name = quote::format_ident!("{}Comb", ident);
    let sig_name = quote::format_ident!("{}Sig", ident);
    let i_name = quote::format_ident!("{}Input", ident);
    let body = &func.block;
    let gen = quote! {
        impl <'a> #comb_name<'a> {
            pub fn new(sig: &'a #sig_name, input: &'a #i_name) -> Self {
                    #body
            }
        }
    };
    gen.into()
}

#[proc_macro_attribute]
pub fn proc(_: TokenStream, item: TokenStream) -> TokenStream {
    let func = syn::parse_macro_input!(item as syn::ItemFn);
    let ident = &func.sig.ident;
    let proc_name = quote::format_ident!("{}Proc", ident);
    let comb_name = quote::format_ident!("{}Comb", ident);
    let sig_name = quote::format_ident!("{}Sig", ident);
    let i_name = quote::format_ident!("{}Input", ident);
    let body = &func.block;
    let gen = quote! {
        impl <'a> #proc_name<'a> {
            pub fn new(sig: &'a #sig_name, input: &'a #i_name, comb: &'a #comb_name) -> Self {
                    #body
            }
        }
    };
    gen.into()
}

struct ModuleParse {
    pub params_type: Vec<syn::GenericArgument>,
    pub params_name: Vec<syn::Ident>,
    pub inputs_type: Vec<syn::Type>,
    pub inputs_name: Vec<syn::Ident>,
    pub outputs_type: Vec<syn::Type>,
    pub outputs_name: Vec<syn::Ident>,
    pub channel_fns: Vec<syn::Field>,
    pub procs_type: Vec<syn::GenericArgument>,
    pub procs_name: Vec<syn::Ident>,
    pub procs_struc: Vec<syn::Ident>,
}

impl ModuleParse {
    fn new() -> Self {
        Self {
            params_type: Vec::new(),
            params_name: Vec::new(),
            inputs_type: Vec::new(),
            inputs_name: Vec::new(),
            outputs_type: Vec::new(),
            outputs_name: Vec::new(),
            channel_fns: Vec::new(),
            procs_type: Vec::new(),
            procs_name: Vec::new(),
            procs_struc: Vec::new(),
        }
    }
}
