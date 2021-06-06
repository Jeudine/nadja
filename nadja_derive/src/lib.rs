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
        //TODO remove derive(new)
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
    proc: &'a syn::Ident,
    args: syn::punctuated::Punctuated<syn::Expr, syn::token::Comma>,
}

struct CombNode<'a> {
    name: &'a syn::Ident,
    channel: &'a syn::ExprStruct,
}

#[derive(Default)]
struct ModuleAst<'a> {
    consts: Vec<&'a syn::ItemConst>,
    uses: Vec<&'a syn::ItemUse>,
    ins: Vec<IoNode<'a>>,
    outs: Vec<IoNode<'a>>,
    combs: Vec<CombNode<'a>>,
    procs: Vec<ProcNode<'a>>,
    ffs: Vec<ProcNode<'a>>,
    output: syn::punctuated::Punctuated<syn::FieldValue, syn::token::Comma>,
}

impl<'a> ModuleAst<'a> {
    fn push_io(&mut self, item: &'a syn::ItemStruct) {
        assert!(
            item.ident.to_string().eq(&String::from("io")),
            "unexpected struct in module definition"
        );
        match &item.fields {
            syn::Fields::Named(x) => x.named.iter().for_each(|x| match &x.ty {
                syn::Type::Path(p) => {
                    match p.path.segments.last().unwrap().ident.to_string().as_str() {
                        "In" => self.ins.push(IoNode {
                            name: x.ident.as_ref().unwrap(),
                            ty: &x.ty,
                        }),
                        "Out" => self.outs.push(IoNode {
                            name: x.ident.as_ref().unwrap(),
                            ty: &x.ty,
                        }),
                        _ => panic!("unexpected expression"),
                    }
                }
                _ => panic!("`In<T>` or `Out<T>` expected"),
            }),
            _ => panic!("unexpected field"),
        };
    }

    fn push_core(&mut self, item: &'a syn::ItemFn) {
        assert!(
            item.sig.ident.to_string().eq(&String::from("core")),
            "unexpected function in module definition"
        );

        item.block.stmts.iter().for_each(|x| match x {
            syn::Stmt::Semi(x, _) => match x {
                syn::Expr::Struct(x) => {
                    let path = &x.path.segments;
                    assert!(
                        (path.len() == 1)
                            & path
                                .last()
                                .unwrap()
                                .ident
                                .to_string()
                                .eq(&String::from("Output")),
                        "unexpected struct"
                    );
                    self.output = x.fields.clone();
                }
                _ => panic!("unexpected expression"),
            },
            syn::Stmt::Local(x) => match &*x.init.as_ref().unwrap().1 {
                syn::Expr::Call(p) => self.push_proc(p, &x.pat),
                syn::Expr::Struct(p) => self.push_comb(p, &x.pat),
                _ => panic!("unexpected expression"),
            },
            _ => panic!("expression with trailing semicolon expected"),
        });
    }

    fn push_proc(&mut self, reg: &'a syn::ExprCall, sig: &'a syn::Pat) {
        let (ty, name) = match sig {
            syn::Pat::Type(x) => (
                &*x.ty,
                match &*x.pat {
                    syn::Pat::Ident(x) => &x.ident,
                    _ => panic!("unexpected identifier"),
                },
            ),
            _ => panic!("type ascription expected"),
        };

        let (proc, args) = (
            match &*reg.func {
                syn::Expr::Path(x) => &x.path.segments.last().unwrap().ident,
                _ => panic!("function identifier expected"),
            },
            {
                let mut args = reg.args.clone();
                args.push(syn::Expr::Path(syn::ExprPath {
                    attrs: Vec::new(),
                    qself: None,
                    path: syn::Path {
                        leading_colon: None,
                        segments: {
                            let mut path = syn::punctuated::Punctuated::new();
                            path.push(syn::PathSegment {
                                ident: name.clone(),
                                arguments: syn::PathArguments::None,
                            });
                            path
                        },
                    },
                }));
                args
            },
        );
        let node = ProcNode {
                ty: ty,
                name: name,
                proc: proc,
                args: args,
        };

        if proc.to_string().eq(&String::from("FF")) {
            self.ffs.push(node);
        } else {
            self.procs.push(node);
        }
    }

    fn push_comb(&mut self, channel: &'a syn::ExprStruct, wire: &'a syn::Pat) {
        self.combs.push(CombNode {
            name: match wire {
                syn::Pat::Ident(x) => &x.ident,
                _ => panic!("unexpected identifier"),
            },
            channel: channel,
        });
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
            syn::Item::Use(x) => m.uses.push(x),
            syn::Item::Struct(x) => m.push_io(x),
            syn::Item::Fn(x) => m.push_core(x),
            _ => panic!("unexpected item in module definition"),
        };
        m
    });
    let consts = &mod_ast.consts;
    let uses = &mod_ast.uses;
    let ins_name = mod_ast.ins.iter().map(|x| x.name).collect::<Vec<_>>();
    let ins_ty = mod_ast.ins.iter().map(|x| x.ty).collect::<Vec<_>>();
    let outs_name = mod_ast.outs.iter().map(|x| x.name).collect::<Vec<_>>();
    let outs_ty = mod_ast.outs.iter().map(|x| x.ty).collect::<Vec<_>>();
    let procs_name = mod_ast.procs.iter().map(|x| x.name).collect::<Vec<_>>();
    let procs_ty = mod_ast.procs.iter().map(|x| x.ty).collect::<Vec<_>>();
    let procs_proc = mod_ast.procs.iter().map(|x| x.proc).collect::<Vec<_>>();
    let procs_args = mod_ast.procs.iter().map(|x| &x.args).collect::<Vec<_>>();
    let combs_name = mod_ast.combs.iter().map(|x| x.name).collect::<Vec<_>>();
    let combs_chann_name = mod_ast
        .combs
        .iter()
        .map(|x| &x.channel.path)
        .collect::<Vec<_>>();
    let combs_chann = mod_ast.combs.iter().map(|x| &x.channel).collect::<Vec<_>>();
    let output = mod_ast.output;
    let ffs_name = mod_ast.ffs.iter().map(|x| x.name).collect::<Vec<_>>();
    let ffs_ty = mod_ast.ffs.iter().map(|x| x.ty).collect::<Vec<_>>();
    let ffs_proc = mod_ast.ffs.iter().map(|x| x.proc).collect::<Vec<_>>();
    let ffs_args = mod_ast.ffs.iter().map(|x| &x.args).collect::<Vec<_>>();

    let gen = quote! {
        #mod_vis mod #mod_name {
            use nadja::logic::{concat, Logic, VLogic};
            use nadja::logic::Logic::{Logic0, Logic1};
            use nadja::process::{Clk, Reg, RegRst, Rst, FF};
            use nadja::{Channel, In, Out, Signal, Simulator, Wire, Param};
            #(#consts)*
            #(#uses)*
            //TODO: visibility of each struct
            pub struct Input<'a> {
                #(pub #ins_name: &'a #ins_ty,)*
            }

            #[derive(Default)]
            pub struct Sig {
                #(#procs_name: Signal<#procs_ty>,)*
                #(#ffs_name: Signal<VLogic<#ffs_ty>>,)*
            }

            pub struct Comb<'a> {
                #(#combs_name: #combs_chann_name<'a>,)*
            }

            impl<'a> Comb<'a> {
                #[allow(unused_variables)]
                pub fn init(input: &'a Input, sig: &'a Sig) -> Self {
                    #(let #ins_name = input.#ins_name;)*
                    #(let #procs_name = &sig.#procs_name;)*
                    #(let #ffs_name = &sig.#ffs_name;)*
                    Self {
                        #(#combs_name: #combs_chann,)*
                    }
                }
            }

            pub struct Proc<'a> {
                #(#procs_name: #procs_proc<'a, #procs_ty>,)*
                #(#ffs_name: #ffs_proc<'a, #ffs_ty>,)*
            }

            impl<'a> Proc<'a> {
                #[allow(unused_variables)]
                pub fn init(input: &'a Input, sig: &'a Sig, comb: &'a Comb) -> Self {
                    #(let #ins_name = input.#ins_name;)*
                    #(let #procs_name = &sig.#procs_name;)*
                    #(let #ffs_name = &sig.#ffs_name;)*
                    #(let #combs_name = &comb.#combs_name;)*
                    Self {
                        #(#procs_name: #procs_proc::new(#procs_args),)*
                        #(#ffs_name: #ffs_proc::new(#ffs_args),)*
                    }
                }
            }

            pub struct Instance<'a> {
                pub #(#outs_name: &'a #outs_ty,)*
                pub _nadja_proc_: &'a Proc<'a>,
            }

            impl<'a> Instance<'a> {
                #[allow(unused_variables)]
                pub fn init(input: &'a Input, sig: &'a Sig, comb: &'a Comb, proc: &'a Proc) -> Self {
                    #(let #ins_name = input.#ins_name;)*
                    #(let #procs_name = &sig.#procs_name;)*
                    #(let #ffs_name = &sig.#ffs_name;)*
                    #(let #combs_name = &comb.#combs_name;)*
                    Self {
                        #output
                        _nadja_proc_: proc,
                    }
                }
            }
        }
#[macro_export]
        macro_rules! #mod_name {
            (
                $instance:ident {
                    $(
                        $in:ident: $ext:expr
                     ),* $(,)*
                }
            ) => {
                let _nadja_input_ = #mod_name::Input {
                    $(
                        $in: &$ext
                     )*
                };
                let _nadja_sig_ = #mod_name::Sig::default();
                let _nadja_comb_ = #mod_name::Comb::init(&_nadja_input_, &_nadja_sig_);
                let _nadja_proc_ = #mod_name::Proc::init(&_nadja_input_, &_nadja_sig_, &_nadja_comb_);
                let $instance = #mod_name::Instance::init(&_nadja_input_, &_nadja_sig_, &_nadja_comb_, &_nadja_proc_);
            }
        }
    };
    gen.into()
}
