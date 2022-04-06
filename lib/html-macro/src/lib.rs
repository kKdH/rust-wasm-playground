mod html;
mod html_parse;
mod html_quote;

use proc_macro::TokenStream;
use std::collections::VecDeque;
use std::process::id;

use proc_macro2::{Literal, Span};
use quote::{quote, ToTokens};
use syn::{Attribute, DeriveInput, Error, Expr, FnArg, GenericArgument, Generics, Ident, Index, ItemFn, Lit, LitStr, Meta, parse_macro_input, parse_quote, PathArguments, PatType, Signature, spanned::Spanned, Token, Type, TypePath, Visibility};
use syn::__private::TokenStream2;
use syn::parse::{Parse, ParseStream, Peek, Result};
use syn::token::Token;

use vdom::{VNode, VRef, VTree};
use crate::html::Html;

#[proc_macro]
pub fn html(item: TokenStream) -> TokenStream {

    let html: Html = ::syn::parse(item).unwrap();

    let output: proc_macro2::TokenStream = quote! {
        #html
    };

    TokenStream::from(output)
}
