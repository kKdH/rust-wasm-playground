extern crate core;

use proc_macro::TokenStream;

use quote::quote;

use crate::html::{Html, Html2};
use crate::html_analyse::{analyse_html, AnalyseResult};
use crate::html_parse::{HtmlTokenStream, parse_html};

mod html;
mod html_analyse;
mod html_parse;
mod html_quote;

#[proc_macro]
pub fn html(item: TokenStream) -> TokenStream {

    let html: Html = ::syn::parse(item.clone()).unwrap();
    let html_tokens: HtmlTokenStream = parse_html(item);
    let _: AnalyseResult = analyse_html(html_tokens);

    let output: proc_macro2::TokenStream = quote! {
        #html
    };

    TokenStream::from(output)
}
