extern crate core;

use proc_macro::TokenStream;

use quote::quote;

use crate::html::Html;
use crate::html_analyse::{analyse_html, AnalyseResult};
use crate::html_parse::{HtmlTokenStream, parse_html};

mod html;
mod html_analyse;
mod html_parse;
mod html_quote;

#[proc_macro]
pub fn html(item: TokenStream) -> TokenStream {

    let parse_result: HtmlTokenStream = parse_html(item);
    let analyse_result: AnalyseResult = analyse_html(parse_result);
    let html = analyse_result.ok().unwrap();

    let output: proc_macro2::TokenStream = quote! {
        #html
    };

    TokenStream::from(output)
}
