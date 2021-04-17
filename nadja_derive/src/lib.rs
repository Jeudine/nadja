extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_attribute]
pub fn channel(attr: TokenStream, item: TokenStream) -> TokenStream {
    //TODO panic if the item is not a function
    let ast: syn::ItemFn = syn::parse(item).unwrap();
    let sig = &ast.sig;

    let channel_name = &sig.ident;
    let output_type = &sig.output;
    //TODO if empty
    let inputs = &sig.inputs;

    let gen = quote! {
        struct #channel_name {
            #inputs
        }

        //impl Channel<#output_type> for #channel_name {
        //}

    };
    gen.into()
}
