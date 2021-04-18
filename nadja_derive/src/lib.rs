extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::Token;
//TODO text for self

#[proc_macro_attribute]
pub fn channel(attr: TokenStream, item: TokenStream) -> TokenStream {
    //TODO panic if the item is not a function
    let ast: syn::ItemFn = syn::parse(item).unwrap();
    let sig = &ast.sig;
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

    let body = &ast.block;

    let gen = quote! {
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
