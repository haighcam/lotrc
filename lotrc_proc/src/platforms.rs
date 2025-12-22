use proc_macro2::{Group, Ident, TokenStream, TokenTree};
use quote::quote;

fn parse_stream(ver: &str, item: TokenStream) -> TokenStream {
    TokenStream::from_iter(item.into_iter().map(|x| match x {
        TokenTree::Ident(ident) => {
            let mut s = ident.to_string();
            if s.ends_with("VER") {
                s.replace_range(s.len() - 3..s.len(), ver);
                TokenTree::Ident(Ident::new(&s, ident.span()))
            } else if s.ends_with("_ver") {
                s.replace_range(s.len() - 3..s.len(), &ver.to_lowercase());
                TokenTree::Ident(Ident::new(&s, ident.span()))
            } else if s == "IS_PC" {
                TokenTree::Ident(Ident::new(&format!("{}", ver == "PC"), ident.span()))
            } else if s == "IS_XBOX" {
                TokenTree::Ident(Ident::new(&format!("{}", ver == "XBOX"), ident.span()))
            } else if s == "IS_PS3" {
                TokenTree::Ident(Ident::new(&format!("{}", ver == "PS3"), ident.span()))
            } else {
                TokenTree::Ident(ident)
            }
        }
        TokenTree::Group(group) => TokenTree::Group(Group::new(
            group.delimiter(),
            parse_stream(ver, group.stream()),
        )),
        tree => tree,
    }))
}

pub fn attr_impl(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item = TokenStream::from(item);

    let s = item.to_string();
    if s.contains("VER") || s.contains("_ver") {
        let pc = parse_stream("Pc", item.clone());
        let xbox = parse_stream("Xbox", item.clone());
        let ps3 = parse_stream("Ps3", item);
        quote! {
            #pc
            #xbox
            #ps3
        }
        .into()
    } else {
        item.into()
    }
}
