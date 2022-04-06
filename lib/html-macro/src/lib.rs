use proc_macro::TokenStream;

use quote::quote;

use crate::html::Html;

mod html;
mod html_parse;
mod html_quote;

#[proc_macro]
pub fn html(item: TokenStream) -> TokenStream {

    let html: Html = ::syn::parse(item).unwrap();

    let output: proc_macro2::TokenStream = quote! {
        #html
    };

    TokenStream::from(output)
}
