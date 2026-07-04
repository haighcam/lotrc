use proc_macro2::{TokenStream, Group, TokenTree, Span};
use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::punctuated::Punctuated;
use syn::{
    parse_macro_input, spanned::Spanned, Data, DeriveInput, Fields, Ident, Index, Type, 
    Token, parse::{Parse, ParseStream, Parser}, ItemFn, ItemMod, ItemImpl, Item, Meta, ItemStruct
};
use syn::{Attribute, Visibility};
use indexmap::IndexMap;

type ExportAttrs = Punctuated<Meta, Token![,]>;



#[proc_macro_attribute]
pub fn export_slice(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attr = parse_macro_input!(attr with ExportAttrs::parse_terminated);
    let item = parse_macro_input!(item as ItemStruct);
    let name = &item.ident;
    let mod_name = format_ident!("ffi_{}", name);

    quote! {

        mod #mod_name { 
        /// This is a proxy struct for correct size and alignment only
        /// don't construct this directly and use the provided methods to access
        #[repr(C)]
        pub struct $name {
            align: u64,
            pad: [u8; $n]
        }
        assert_layout!(Option<$t>, $name);
        #[lotrc::macros::export(base_only)]
        impl $name {
            fn get<'a>(slice: Option<&'a Option<$t>>) -> Option<&'a $t> {
                slice.and_then(|x| x.as_ref())
            }
        }
        }
        #item
    }.into()
}

enum ExportInput {
    Impl(ItemImpl),
    Mod(ItemMod),
    Fn(ItemFn)
}

impl ToTokens for ExportInput {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            ExportInput::Impl(val) => val.to_tokens(tokens),
            ExportInput::Mod(val) => val.to_tokens(tokens),
            ExportInput::Fn(val) => val.to_tokens(tokens)
        }
    }
}

impl Parse for ExportInput {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        if let Ok(val) = ItemImpl::parse(input) {
            Ok(Self::Impl(val))
        } else if let Ok(val) = ItemMod::parse(input) {
            Ok(Self::Mod(val))
        } else if let Ok(val) = ItemFn::parse(input) {
            Ok(Self::Fn(val))
        } else {
            Err(input.error("expected impl, mod, or fn"))
        }
    }
}

fn export_impl(input: &mut ItemImpl, path: &str, attr: &ExportAttrs) {
    let mut path = path.to_string();
    let mut ty_name = input.self_ty.to_token_stream().to_string();
    if let Some(Meta::NameValue(val)) = attr.iter().find(|x| x.path().is_ident("impl_name")) {
        ty_name = val.value.to_token_stream().to_string();
    }
    if attr.iter().any(|x| x.path().is_ident("base_only")) {
        path = ty_name;
    } else {
        path = path + "_" + ty_name.as_str();
    }
    for item in input.items.iter_mut() {
        if let syn::ImplItem::Fn(f) = item {
            let name = f.sig.ident.to_string();
            f.sig.ident = format_ident!("{}_{}", path, name);
            f.sig.abi.replace(syn::Abi { 
                extern_token: syn::token::Extern {
                    span: f.sig.span() 
                },
                name: Some(syn::LitStr::new("C", f.sig.span()))
            });
            if !f.attrs.iter().any(|x| x.path().is_ident("no_mangle")) {
                let mut no_mangle = Attribute::parse_outer.parse_str("#[unsafe(no_mangle)]").unwrap();
                f.attrs.append(&mut no_mangle);
            }
        }
    }
}

fn export_mod(input: &mut ItemMod, path: &str, attr: &ExportAttrs) {
    let mut path = path.to_string() + input.ident.to_string().as_str() + "_";
    if let Some(Meta::NameValue(val)) = attr.iter().find(|x| x.path().is_ident("mod_name")) {
        path = val.value.to_token_stream().to_string() + "_";
    }
    if let Some((_, vals)) = &mut input.content {
        for val in vals {
            match val {
                Item::Fn(val) => export_fn(val, &path, attr),
                _ => ()
            }
        }
    }
}

fn export_fn(input: &mut ItemFn, path: &str, _attr: &ExportAttrs) {
    let name = input.sig.ident.to_string();
    input.sig.ident = format_ident!("{}{}", path, name);
    input.sig.abi.replace(syn::Abi { 
        extern_token: syn::token::Extern {
            span: input.sig.span() 
        },
        name: Some(syn::LitStr::new("C", input.sig.span()))
    });
    if !input.attrs.iter().any(|x| x.path().is_ident("no_mangle")) {
        let mut no_mangle = Attribute::parse_outer.parse_str("#[unsafe(no_mangle)]").unwrap();
        input.attrs.append(&mut no_mangle);
    }
}

#[proc_macro_attribute]
pub fn export(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut input = parse_macro_input!(item as ExportInput);
    let attr = parse_macro_input!(attr with ExportAttrs::parse_terminated);
    let module_path = "";
    match &mut input {
        ExportInput::Impl(val) => export_impl(val, &module_path, &attr),
        ExportInput::Mod(val) => export_mod(val, &module_path, &attr),
        ExportInput::Fn(val) => export_fn(val, &module_path, &attr)
    }
    input.into_token_stream().into()
}

fn make_platform(ver: &str, item: TokenStream) -> TokenStream {
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
                TokenTree::Ident(Ident::new(&format!("{}", ver == "Pc"), ident.span()))
            } else if s == "IS_XBOX" {
                TokenTree::Ident(Ident::new(&format!("{}", ver == "Xbox"), ident.span()))
            } else if s == "IS_PS3" {
                TokenTree::Ident(Ident::new(&format!("{}", ver == "Ps3"), ident.span()))
            } else {
                TokenTree::Ident(ident)
            }
        }
        TokenTree::Group(group) => TokenTree::Group(Group::new(
            group.delimiter(),
            make_platform(ver, group.stream()),
        )),
        tree => tree,
    }))
}

#[proc_macro_attribute]
pub fn make_platforms(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item = TokenStream::from(item);

    let s = item.to_string();
    if s.contains("VER") || s.contains("_ver") {
        let mut out = make_platform("Pc", item.clone());
        out.extend(make_platform("Xbox", item.clone()));
        out.extend(make_platform("Ps3", item));
        out.into()
    } else {
        item.into()
    }
}

enum VersionStruct {
    Named(IndexMap<Ident, (Visibility, TokenStream, Vec<Attribute>, Span)>),
    Unamed(Vec<(Visibility, TokenStream, Span)>),
    Unit
}

fn make_ver(ver: &str, val: TokenStream) -> TokenStream {
    let mut vals = val.into_iter();
    let mut new_vals = vec![];
    let mut prev = None;
    while let Some(val) = vals.next() {
        if let Some(val) = prev.replace(val) {
            new_vals.push(val);
        }
    }
    if let Some(val) = prev {
        if let TokenTree::Ident(val) = val {
            new_vals.push(TokenTree::Ident(format_ident!("{}{}", val, ver)));
        } else {
            new_vals.push(val);
        }
    }
    TokenStream::from_iter(new_vals.into_iter())
}

fn process_attrs(attrs: &Vec<Attribute>, ver: &str) -> (bool, Option<Ident>) {
    let mut skip = false;
    let mut name = None;
    let skip_name = format_ident!("skip_{}", ver.to_lowercase());
    let name_name = format_ident!("name_{}", ver.to_lowercase());
    if let Some(attr) = attrs.iter().find(|x| x.path().is_ident("ordered_data")) {
        if let Meta::List(val) = &attr.meta {
            if let Some(vals) = ExportAttrs::parse_terminated.parse(val.tokens.clone().into()).ok() {
                for val in vals {
                    match val {
                        Meta::Path(x) if x.is_ident(&skip_name) => {
                            skip = true;
                        },
                        Meta::NameValue(x) if x.path.is_ident(&name_name) => {
                            name.replace(format_ident!("{}", x.value.to_token_stream().to_string()));
                        }
                        _ => ()
                    }
                }
            }
        }
    }
    (skip, name)
}

impl VersionStruct {
    fn from_def(data: &Data) -> Self {
        match data {
            Data::Struct(ref data) => match data.fields {
                Fields::Named(ref fields) => Self::Named(
                    fields.named.iter().map(|f| (f.ident.as_ref().unwrap().clone(), (f.vis.clone(), f.ty.to_token_stream(), f.attrs.clone(), f.span()))).collect()
                ),
                Fields::Unnamed(ref fields) => Self::Unamed(
                    fields.unnamed.iter().map(|f| (f.vis.clone(), f.ty.to_token_stream(), f.span())).collect()
                ),
                Fields::Unit => Self::Unit,
            },
            Data::Enum(_) | Data::Union(_) => unimplemented!(),
        }
    }
    fn get_ver(&self, ver: &str) -> Self {
        match self {
            Self::Unit => Self::Unit,
            //Self::Unamed(fields) => Self::Unamed(fields.iter().map(|(vis, ty, span)| (vis.clone(), make_platform(ver, ty.clone()), span.clone())).collect()),
            Self::Unamed(fields) => Self::Unamed(fields.iter().map(|(vis, ty, span)| (vis.clone(), make_ver(ver, ty.clone()), span.clone())).collect()),
            Self::Named(fields) => Self::Named(fields.iter().filter_map(|(k, v)| {
                let (skip, name) = process_attrs(&v.2, ver);
                let (mut vis, mut ty, mut attrs, mut span) = v.clone();
                let mut base_name = k.clone();
                if skip {
                    None
                } else {
                    if let Some(name) = name {
                        base_name = name;
                        (vis, ty, attrs, span) = fields.get(&base_name).unwrap().clone();
                    }
                    //Some((base_name, (vis, make_platform(ver, ty), attrs, span)))
                    Some((base_name, (vis, make_ver(ver, ty), attrs, span)))
                }
            }).collect())
        }
    }
    fn get_def_impl(&self, name: &Ident) -> TokenStream {
        match self {
            Self::Unit => quote!{ struct #name; },
            Self::Unamed(fields) => {
                let recurse = fields.iter().map(|(vis, ty, span)| quote_spanned! {
                    span.clone() => #vis #ty
                });
                quote! {
                    struct #name(#(#recurse),*);
                }
            },
            Self::Named(fields) => {
                let recurse = fields.iter().map(|(name, (vis, ty, _, span))| quote_spanned! {
                    span.clone() => #vis #name: #ty
                });
                quote! {
                    struct #name {
                        #(#recurse),*
                    }
                }
            }
        }.into()
    }
    fn get_conv_impl(&self, name: &str) -> TokenStream {
        let name = format_ident!("{}", name);
        match self {
            Self::Unit => quote! {},
            Self::Unamed(fields) => {
                let recurse = fields.iter().enumerate().map(|(i, f)| {
                    let index = Index::from(i);
                    let span = f.2;
                    quote_spanned! {
                        span => OrderedData::conv(&#name.#index)
                    }
                });
                quote! {
                    (#(#recurse),*)
                }
            },
            Self::Named(fields) => {
                let recurse = fields.iter().map(|(fname, f)| {
                     let span = f.3;
                     quote_spanned! {
                        span => #fname: OrderedData::conv(&#name.#fname)
                     }
                });
                quote! {
                    { 
                        #(#recurse),*,
                        ..Default::default()
                    }
                }
            }
        }.into()
    }
}

fn get_ver_impl(name: &Ident, vis: &Visibility, info: &VersionStruct, ver: &str) -> (Ident, TokenStream) {
    let alt_name = format_ident!("{}{}", name, ver);
    let alt_info = info.get_ver(ver);
    let def_impl = alt_info.get_def_impl(&alt_name);
    let conv_impl = alt_info.get_conv_impl("self");
    let from_impl = alt_info.get_conv_impl("val");

    let expanded = quote! {
        ///gen_ffi:export
        #[repr(C)]
        #[derive(PartialEq, Default, Debug, Clone, zerocopy::KnownLayout, zerocopy::FromBytes, zerocopy::IntoBytes, zerocopy::Immutable)]
        #vis #def_impl
        impl From<#alt_name> for #name {
            fn from(val: #alt_name) -> Self {
                Self #from_impl
            }
        }
        impl From<#name> for #alt_name {
            fn from(val: #name) -> Self {
                Self #from_impl
            }
        }
        impl OrderedData<#name> for #alt_name {
            fn conv(&self) -> #name {
                #name #conv_impl
            }
        }
        impl OrderedData<#alt_name> for #name {
            fn conv(&self) -> #alt_name {
                #alt_name #conv_impl
            }
        }
    }.into();
    (alt_name, expanded)
}

#[proc_macro_derive(OrderedData, attributes(ordered_data, name_pc, name_xbox, name_ps3))]
pub fn derive_ordered_data_fn(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let vis = &input.vis;
    let info = VersionStruct::from_def(&input.data);
    let (name_pc, impl_pc) = get_ver_impl(name, vis, &info, "Pc");
    let (name_xbox, impl_xbox) = get_ver_impl(name, vis, &info, "Xbox");
    let (name_ps3, impl_ps3) = get_ver_impl(name, vis, &info, "Ps3");

    quote! {
        #impl_pc
        #impl_xbox
        #impl_ps3
        impl OrderedDataStrict for #name {
            type Pc = #name_pc;
            type Xbox = #name_xbox;
            type Ps3 = #name_ps3;
        }
    }.into()
}

