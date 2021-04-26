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
    let proc_name = quote::format_ident!("{}Proc", module_name);

    let attrs: ModuleParse = match struc.fields {
        syn::Fields::Named(p) => p.named.iter().fold(ModuleParse::new(), |mut m, x| {
            match x.ty {
                syn::Type::Path(ref p) => {
                    let ps = p.path.segments.first().unwrap();
                    match ps.ident.to_string().as_str() {
                        "Param" => m.parameters.push(x.clone()),
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

    let inputs_type = &attrs.inputs_type;
    let inputs_name = &attrs.inputs_name;
    let outputs_type = &attrs.outputs_type;
    let outputs_name = &attrs.outputs_name;
    let channel_fns = &attrs.channel_fns;
    let int_signs = &attrs.int_signs;
    let procs_name = &attrs.procs_name;
    let procs_type = &attrs.procs_type;
    let procs_struc = &attrs.procs_struc;

    let gen = quote! {
        #[derive(Default)]
        struct #sig_name {
            #(pub #procs_name: Signal<#procs_type>,)*
        }

        struct #comb_name<'a> {
            #(#inputs_name: &'a #inputs_type,)*
            #(pub #outputs_name: &'a #outputs_type,)*
            #(#channel_fns<'a>,)*
        }

        struct #proc_name<'a> {
            #(#procs_name: #procs_struc<'a, #procs_type>,)*
        }
    };
    gen.into()
}

struct ModuleParse {
    pub parameters: Vec<syn::Field>,
    pub inputs_type: Vec<syn::Type>,
    pub inputs_name: Vec<syn::Ident>,
    pub outputs_type: Vec<syn::Type>,
    pub outputs_name: Vec<syn::Ident>,
    pub channel_fns: Vec<syn::Field>,
    pub procs_type: Vec<syn::GenericArgument>,
    pub procs_name: Vec<syn::Ident>,
    pub procs_struc: Vec<syn::Ident>,
    pub int_signs: Vec<syn::Field>
}

impl ModuleParse {
    fn new() -> Self {
        Self {
            parameters: Vec::new(),
            inputs_type: Vec::new(),
            inputs_name: Vec::new(),
            outputs_type: Vec::new(),
            outputs_name: Vec::new(),
            channel_fns: Vec::new(),
            procs_type: Vec::new(),
            procs_name: Vec::new(),
            procs_struc: Vec::new(),
            int_signs: Vec::new(),
        }
    }
}
