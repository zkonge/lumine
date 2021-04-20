use proc_macro::TokenStream;
use proc_macro2::{Ident, Span as Span2};
use quote::quote;

#[proc_macro_attribute]
pub fn handler_fn(_attr: TokenStream, item: TokenStream) -> TokenStream {
    if let syn::Item::Fn(mut function_item) = syn::parse(item.clone()).unwrap() {
        let name = function_item.sig.ident.clone();
        let new_name = Ident::new(&format!("__async_{}", name), Span2::call_site());
        function_item.sig.ident = new_name.clone();

        let visibility = function_item.vis.clone();
        let arguments = function_item.sig.inputs.clone();
        let generics = function_item.sig.generics.clone();

        let context_type = match &arguments[0] {
            syn::FnArg::Typed(cap) => &cap.ty,
            _ => panic!("Expected the first argument to be a context type"),
        };
        let event_type = match &arguments[1] {
            syn::FnArg::Typed(cap) => &cap.ty,
            _ => panic!("Expected the second argument to be a context type"),
        };

        let new_return_type = Ident::new(&format!("__AsyncFnWrapper_{}", name), Span2::call_site());
        let crate_path = quote! { lumine::{ AsyncFnReturnType as #new_return_type } };

        let gen = quote! {
            #function_item

            use #crate_path;
            #visibility fn #name#generics(bot: #context_type, event: #event_type) -> #new_return_type<()> {
                Box::pin(#new_name(bot, event))
            }
        };
        gen.into()
    } else {
        item
    }
}
