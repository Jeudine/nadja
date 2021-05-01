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
        pub struct #channel_name<'a> {
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
    let o_name = quote::format_ident!("{}Output", module_name);
    let proc_name = quote::format_ident!("{}Proc", module_name);
    let m_name = quote::format_ident!("{}m", module_name);

    let attrs: ModuleParse = match struc.fields {
        syn::Fields::Named(p) => p.named.iter().fold(ModuleParse::default(), |mut m, x| {
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
        pub struct #sig_name {
            #(#procs_name: Signal<#procs_type>,)*
        }

        pub struct #i_name<'a> {
            #(#params_name: #params_type,)*
            #(#inputs_name: &'a #inputs_type,)*
        }

        pub struct #o_name<'a> {
            #(pub #outputs_name: &'a #outputs_type,)*
        }

        pub struct #comb_name<'a> {
            #(#channel_fns<'a>,)*
        }

        pub struct #proc_name<'a> {
            #(pub #procs_name: #procs_struc<'a, #procs_type>,)*
        }

        pub struct #module_name<'a> {
            pub o: #o_name<'a>,
            pub p: #proc_name<'a>,
        }

        #[macro_export]
        macro_rules! #module_name {
            (
                $i:ident {
                    $(
                        $fn:ident: $expr:expr
                     ),* $(,)*
                }
            ) => {
                mashup! {
                    #m_name["sig" $i] = sig_ $i;
                    #m_name["input" $i] = input_ $i;
                    #m_name["comb" $i] = comb_ $i;
                }
                #m_name! {
                    let "sig" $i = #sig_name::default();
                    let "input" $i = #i_name {
                        $(
                            $fn: $expr,
                            )*
                    };
                    let "comb" $i = #comb_name::new(& "sig" $i, & "input" $i);
                    let $i = #module_name {
                        o: #o_name::new(& "sig" $i, & "input" $i, & "comb" $i),
                        p: #proc_name::new(& "sig" $i, & "input" $i, & "comb" $i),
                    };
                }
            }
        }
    };
    gen.into()
}

#[proc_macro_attribute]
pub fn out(_: TokenStream, item: TokenStream) -> TokenStream {
    let func = syn::parse_macro_input!(item as syn::ItemFn);
    let ident = &func.sig.ident;
    let comb_name = quote::format_ident!("{}Comb", ident);
    let sig_name = quote::format_ident!("{}Sig", ident);
    let i_name = quote::format_ident!("{}Input", ident);
    let o_name = quote::format_ident!("{}Output", ident);
    let p = OutParse::parse(&func.block.stmts);
    let left = p.left;
    let right = p.right;
    let gen = quote! {
        impl <'a> #o_name<'a> {
            pub fn new(sig: &'a #sig_name, input: &'a #i_name, comb: &'a #comb_name) -> Self {
                Self {
                    #(#left: &#right,)*
                }
            }
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
    let p = CombParse::parse(&func.block.stmts);
    let left = p.left;
    let func = p.func;
    let args = p.args;
    let gen = quote! {
        impl <'a> #comb_name<'a> {
            pub fn new(sig: &'a #sig_name, input: &'a #i_name) -> Self {
                Self {
                    #(#left: #func::new(#args),)*
                }
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
    let p = CombParse::parse(&func.block.stmts);
    let left = p.left;
    let func = p.func;
    let args = p.args;
    let gen = quote! {
        impl <'a> #proc_name<'a> {
            pub fn new(sig: &'a #sig_name, input: &'a #i_name, comb: &'a #comb_name) -> Self {
                Self {
                    #(#left: #func::new(#args),)*
                }
            }
        }
    };
    gen.into()
}

#[derive(Default)]
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

#[derive(Default)]
struct OutParse {
    pub left: Vec<syn::Expr>,
    pub right: Vec<syn::Expr>,
}

impl OutParse {
    fn parse(stmts: &Vec<syn::Stmt>) -> Self {
        stmts.iter().fold(OutParse::default(), |mut m, x| {
            match x {
                syn::Stmt::Semi(x, _) => {
                    match x {
                        syn::Expr::Assign(x) => {
                            m.left.push(*x.left.clone());
                            m.right.push(*x.right.clone());
                            m
                        },
                        _ => panic!("Error, assignment expression expected!"),
                    }
                },
                _ => panic!("Error, expression with trailing semicolon expected!"),
            }
        }
        )
    }
}

#[derive(Default)]
struct CombParse {
    pub left: Vec<syn::Expr>,
    pub func: Vec<syn::Expr>,
    pub args: Vec< syn::punctuated::Punctuated<syn::Expr, syn::token::Comma>>,
}

impl CombParse {
    fn parse(stmts: &Vec<syn::Stmt>) -> Self {
        stmts.iter().fold(CombParse::default(), |mut m, x| {
            match x {
                syn::Stmt::Semi(x, _) => {
                    match x {
                        syn::Expr::Assign(x) => {
                            match &*x.right {
                                syn::Expr::Call(x) => {
                                    m.func.push(*x.func.clone());
                                    m.args.push(x.args.clone());
                                },
                                _ => panic!("Error, function call expression  expected!"),
                            };
                            m.left.push(*x.left.clone());
                            m
                        },
                        _ => panic!("Error, assignment expression expected!"),
                    }
                },
                _ => panic!("Error, expression with trailing semicolon expected!"),
            }
        }
        )
    }
}
