extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashMap;
use syn;

#[proc_macro_attribute]
pub fn channel(_: TokenStream, item: TokenStream) -> TokenStream {
    let func = syn::parse_macro_input!(item as syn::ItemFn);
    let sig = &func.sig;
    let channel_name = &sig.ident;
    let inputs_type = sig
        .inputs
        .iter()
        .map(|x| match x {
            syn::FnArg::Typed(p) => p.ty.clone(),
            _ => panic!("not supported on types with `self`"),
        })
        .collect::<Vec<_>>();

    let inputs_name = sig
        .inputs
        .iter()
        .map(|x| match x {
            syn::FnArg::Typed(p) => p.pat.clone(),
            _ => panic!("not supported on types with `self`"),
        })
        .collect::<Vec<_>>();
    let output_type = match &sig.output {
        syn::ReturnType::Type(_, p) => p,
        _ => panic!("not supported on functions without return types"),
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

struct IoNode<'a> {
    name: &'a syn::Ident,
    ty: &'a syn::Type,
}

struct ProcNode<'a> {
    name: &'a syn::Ident,
    ty: &'a syn::Type,
    proc: &'a syn::Expr,
    args: Vec<syn::punctuated::Punctuated<syn::Expr, syn::token::Comma>>,
}

struct CombNode<'a> {
    name: &'a syn::Ident,
    channel: &'a syn::Expr,
    args: Vec<syn::punctuated::Punctuated<syn::Expr, syn::token::Comma>>,
}

#[derive(Default)]
struct ModuleAst<'a> {
    consts: Vec<&'a syn::ItemConst>,
    ins: Vec<IoNode<'a>>,
    outs: Vec<IoNode<'a>>,
    combs: Vec<CombNode<'a>>,
    procs: Vec<ProcNode<'a>>,
}

impl<'a> ModuleAst<'a> {
    fn push_io(&mut self, item: &'a syn::ItemStruct) {
        assert!(
            item.ident.to_string().eq(&String::from("io")),
            "unexpected struct in module definition"
        );
        match item.fields {
            syn::Fields::Named(ref x) => x.named.iter().for_each(|x| match x.ty {
                syn::Type::Path(ref p) => {
                    match p.path.segments.last().unwrap().ident.to_string().as_str() {
                        "In" => self.ins.push(IoNode {
                            name: x.ident.as_ref().unwrap(),
                            ty: &x.ty,
                        }),
                        "Out" => self.outs.push(IoNode {
                            name: x.ident.as_ref().unwrap(),
                            ty: &x.ty,
                        }),
                        _ => panic!("unexpected path"),
                    }
                }
                _ => panic!("unexpected type"),
            }),
            _ => panic!("unexpected field"),
        };
    }

    fn push_comb_out(&mut self, item: &'a syn::ItemFn) {
        let fn_id = item.sig.ident.to_string();
        let comb_id = String::from("comb");
        assert!(
            fn_id.eq(&comb_id) | fn_id.eq(&String::from("out")),
            "unexpected function in module definition"
        );

        if fn_id.eq(&comb_id) {

        } else {

        }
    }
}
#[proc_macro_attribute]
pub fn seq(_: TokenStream, item: TokenStream) -> TokenStream {
    let module = syn::parse_macro_input!(item as syn::ItemMod);
    let mod_name = &module.ident;
    let mod_vis = &module.vis;
    let content = module.content.expect("module has an empty content").1;

    let mod_ast = content.iter().fold(ModuleAst::default(), |mut m, x| {
        match x {
            syn::Item::Const(x) => m.consts.push(x),
            syn::Item::Struct(x) => m.push_io(x),
            syn::Item::Fn(x) => m.push_comb_out(x),
            //procs
            syn::Item::Static(x) => {} //TODO,
            _ => panic!("unexpected item in module definition"),
        };
        m
    });
    let consts = &mod_ast.consts;
    let ins_name = mod_ast.ins.iter().map(|x| x.name);
    let ins_ty = mod_ast.ins.iter().map(|x| x.ty);
    let outs_name = mod_ast.outs.iter().map(|x| x.name);
    let outs_ty = mod_ast.outs.iter().map(|x| x.ty);

    let gen = quote! {
        #mod_vis mod #mod_name {
            use nadja::logic::{concat, Logic, VLogic};
            use nadja::process::{Clk, RegRst, Rst};
            use nadja::{Channel, In, Out, Signal, Simulator, Wire, Param};
            #(#consts)*
            //TODO: visibility of each struct
            struct Input<'a> {
                #(#ins_name: &'a #ins_ty,)*
            }

            struct Sig {
            }

            struct Comb {
            }

            struct Proc {
            }

            struct Output {
                #(#outs_name: #outs_ty,)*
            }
        }
    };
    gen.into()
}
/*
#[proc_macro_attribute]
pub fn module(_: TokenStream, item: TokenStream) -> TokenStream {
let struc = syn::parse_macro_input!(item as syn::ItemStruct);
let module_name = &struc.ident;
let sig_name = quote::format_ident!("{}Sig", module_name);
let comb_name = quote::format_ident!("{}Comb", module_name);
let i_name = quote::format_ident!("{}Input", module_name);
let i_sig_name = quote::format_ident!("{}InputSig", module_name);
let c_i_sig_name = quote::format_ident!("{}CombInputSig", ident);
let o_name = quote::format_ident!("{}Output", module_name);
let proc_name = quote::format_ident!("{}Proc", module_name);
let m_name = quote::format_ident!("{}m", module_name);
let attrs: ModuleParse = match struc.fields {
syn::Fields::Named(ref p) => p.named.iter().fold(ModuleParse::default(), |mut m, x| {
match x.ty {
syn::Type::Path(ref p) => {
let ps = p.path.segments.first().unwrap();
match ps.ident.to_string().as_str() {
"Param" => {
m.params_type.push(
match &ps.arguments {
syn::PathArguments::AngleBracketed(p) => p.args.first().unwrap(),
_ => panic!("Error, `<` expected!"),
});
m.params_name.push(x.ident.as_ref().unwrap());
},
"Input" => {
m.inputs_type.push(&x.ty);
m.inputs_name.push(x.ident.as_ref().unwrap());
},
"Output" => {
m.outputs_type.push(&x.ty);
m.outputs_name.push(x.ident.as_ref().unwrap());
},
"RegRst" | "Reg" => {
m.procs_type.push(
match &ps.arguments {
syn::PathArguments::AngleBracketed(p) => p.args.first().unwrap(),
_ => panic!("Error, `<` expected!"),
});
m.procs_name.push(x.ident.as_ref().unwrap());
m.procs_struc.push(&ps.ident);
},

_ => panic!("Error, unexpected field!"),
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

pub struct #i_sig_name<'a> {
    #(#params_name: &'a #params_type,)*
    #(#inputs_name: &'a #inputs_type,)*
    #(#procs_name: &'a Signal<#procs_type>,)*
}

impl<'a> #i_sig_name<'a> {
    fn new(input: &'a#i_name, sig: &'a#sig_name) -> Self {
        Self {
            #(#params_name: &input.#params_name,)*
            #(#inputs_name: input.#inputs_name,)*
            #(#procs_name: &sig.#procs_name,)*
        }
    }

    fn toComb(&self) -> #c_i_sig_name {

    }
}

pub struct #o_name<'a> {
    #(pub #outputs_name: &'a #outputs_type,)*
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
            #m_name["i_sig" $i] = input_sig_ $i;
            #m_name["comb" $i] = comb_ $i;
        }
        #m_name! {
            let "sig" $i = #sig_name::default();
            let "input" $i = #i_name {
                $(
                    $fn: $expr,
                 )*
            };
            let "i_sig" $i = #i_sig_name::new(& "input" $i, & "sig" $i);
            let "comb" $i = #comb_name::new(& "i_sig" $i);
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
pub fn comb(_: TokenStream, item: TokenStream) -> TokenStream {
    let func = syn::parse_macro_input!(item as syn::ItemFn);
    let ident = &func.sig.ident;
    let comb_name = quote::format_ident!("{}Comb", ident);
    let i_sig_name = quote::format_ident!("{}InputSig", ident);
    let c_i_sig_name = quote::format_ident!("{}CombInputSig", ident);
    let p = CombParse::parse(&func.block.stmts);
    let left = p.left;
    let func = p.func;
    let args = p.args;
    //TODO: modify args
    let gen = quote! {
        pub struct #comb_name<'a> {
            #(#left: #func<'a>,)*
        }

        impl <'a> #comb_name<'a> {
            pub fn new(i_sig: &'a #i_sig_name) -> Self {
                Self {
                    #(#left: #func::new(#args),)*
                }
            }
        }
        pub struct #c_i_sig_name<'a> {
            #(#params_name: &'a #params_type,)*
            #(#inputs_name: &'a #inputs_type,)*
            #(#procs_name: &'a Signal<#procs_type>,)*
            #(#left:
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




#[derive(Default)]
struct OutParse<'a> {
    pub left: Vec<&'a syn::Expr>,
        pub right: Vec<&'a syn::Expr>,
}

impl<'a> OutParse<'a> {
    fn parse(stmts: &'a Vec<syn::Stmt>) -> Self {
        stmts.iter().fold(OutParse::default(), |mut m, x| {
            match x {
                syn::Stmt::Semi(x, _) => {
                    match x {
                        syn::Expr::Assign(x) => {
                            m.left.push(&x.left);
                            m.right.push(&x.right);
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
struct CombParse<'a> {
    pub left: Vec<&'a syn::Expr>,
        pub func: Vec<&'a syn::Expr>,
        pub args: Vec<syn::punctuated::Punctuated<syn::Expr, syn::token::Comma>>,
}

impl<'a> CombParse<'a> {
    fn parse(stmts: &'a Vec<syn::Stmt>) -> Self {
        stmts.iter().fold(CombParse::default(), |mut m, x| {
            match x {
                syn::Stmt::Semi(x, _) => {
                    match x {
                        syn::Expr::Assign(x) => {
                            match &*x.right {
                                syn::Expr::Call(x) => {
                                    m.func.push(&x.func);
                                    m.args.push(x.args.iter().map(|x| {
                                        syn::punctuated::Pair::Punctuated(x.clone(),syn::token::Comma::default())
                                    }).collect());
                                },
                                _ => panic!("Error, function call expression  expected!"),
                            };
                            m.left.push(&x.left);
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
*/
