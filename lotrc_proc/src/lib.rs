use proc_macro2::{TokenStream, Group, TokenTree, Span};
use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::punctuated::Punctuated;
use syn::{
    parse_macro_input, spanned::Spanned, Data, DeriveInput, Fields, Ident, Index, 
    Token
};
use syn::{Attribute, Visibility};
use indexmap::IndexMap;

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
        let pc = make_platform("Pc", item.clone());
        let xbox = make_platform("Xbox", item.clone());
        let ps3 = make_platform("Ps3", item);
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
    for a in attrs.iter() {
        if a.path().is_ident("ordered_data") {
            skip = a.parse_args_with(Punctuated::<Ident, Token![,]>::parse_terminated)
                .unwrap()
                .into_iter()
                .find(|i| i==&skip_name)
                .is_some();
        } else if a.path().is_ident(&name_name) {
            name = a.parse_args_with(Punctuated::<Ident, Token![,]>::parse_terminated)
                .unwrap()
                .into_iter()
                .next();
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
        #[repr(C)]
        #[derive(Default, Debug, Clone, zerocopy::KnownLayout, zerocopy::FromBytes, zerocopy::IntoBytes, zerocopy::Immutable)]
        #[cfg_attr(feature = "ffi", safer_ffi::derive_ReprC)]
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
/*
fn filter_attrs(attrs: &Vec<Attribute>, ver: &Ident, name: Ident) -> (Ident, bool, Ident) {
    let mut val_ver = ver.clone();
    let mut skip = false;
    for val in attrs
        .iter()
        .filter(|a| a.path().is_ident("ordered_data"))
        .flat_map(|a| {
            a.parse_args_with(Punctuated::<Ident, Token![,]>::parse_terminated)
                .unwrap()
        })
    {
        if val == "Pc" || val == "Xbox" || val == "Ps3" {
            val_ver = val;
        } else if val == "skipPC" && ver == "Pc" {
            skip = true;
        } else if val == "skipXBOX" && ver == "Xbox" {
            skip = true;
        } else if val == "skipPS3" && ver == "Ps3" {
            skip = true;
        }
    }
    let alt_name = if ver == "Pc" {
        attrs
            .iter()
            .filter(|a| a.path().is_ident("name_pc"))
            .flat_map(|a| {
                a.parse_args_with(Punctuated::<Ident, Token![,]>::parse_terminated)
                    .unwrap()
            })
            .next()
    } else if ver == "Xbox" {
        attrs
            .iter()
            .filter(|a| a.path().is_ident("name_xbox"))
            .flat_map(|a| {
                a.parse_args_with(Punctuated::<Ident, Token![,]>::parse_terminated)
                    .unwrap()
            })
            .next()
    } else if ver == "Ps3" {
        attrs
            .iter()
            .filter(|a| a.path().is_ident("name_ps3"))
            .flat_map(|a| {
                a.parse_args_with(Punctuated::<Ident, Token![,]>::parse_terminated)
                    .unwrap()
            })
            .next()
    } else {
        None
    }
    .unwrap_or(name);

    (val_ver, skip, alt_name)
}

fn conv_def(data: &Data, ver: &Ident) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().filter_map(|f| {
                    let name = &f.ident;
                    let (_, skip, alt_name) = filter_attrs(&f.attrs, ver, name.clone().unwrap());
                    if !skip {
                        Some(quote_spanned! {
                            f.span() => #alt_name: OrderedData::conv(&value.#alt_name)
                        })
                    } else {
                        None
                    }
                });
                quote! {
                    {
                        #(#recurse),*,
                        ..Default::default()
                    }
                }
            }
            Fields::Unnamed(ref fields) => {
                let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                    let index = Index::from(i);
                    quote_spanned! {
                        f.span() => OrderedData::conv(&value.#index)
                    }
                });
                quote! {
                    (
                        #(#recurse),*
                    )
                }
            }
            Fields::Unit => {
                quote! {}
            }
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}

fn conv_back_def(data: &Data, ver: &Ident) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().filter_map(|f| {
                    let name = &f.ident;
                    let (_, skip, alt_name) = filter_attrs(&f.attrs, ver, name.clone().unwrap());
                    if !skip {
                        Some(quote_spanned! {
                            f.span() => #alt_name: OrderedData::conv(&value.#alt_name)
                        })
                    } else {
                        None
                    }
                });
                quote! {
                    {
                        #(#recurse),*,
                        ..Default::default()
                    }
                }
            }
            Fields::Unnamed(ref fields) => {
                let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                    let index = Index::from(i);
                    quote_spanned! {
                        f.span() => OrderedData::conv(&value.#index)
                    }
                });
                quote! {
                    (
                        #(#recurse),*
                    )
                }
            }
            Fields::Unit => {
                quote! {}
            }
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}

fn alt_class_def(data: &Data, classname: &Ident, ver: &Ident, vis: &Visibility) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().filter_map(|f| {
                    let name = &f.ident;
                    let ty = &f.ty;
                    let (val, skip, alt_name) =
                        filter_attrs(&f.attrs, ver, name.clone().unwrap());
                    if !skip {
                        Some(quote_spanned! {
                            f.span() => #vis #alt_name: <#ty as OrderedDataStrict>::#val
                        })
                    } else {
                        None
                    }
                });
                quote! {
                    struct #classname {
                        #(#recurse),*
                    }
                }
            }
            Fields::Unnamed(ref fields) => {
                let recurse = fields.unnamed.iter().map(|f| {
                    let ty = &f.ty;
                    quote_spanned! {
                        f.span() => #vis <#ty as OrderedDataStrict>::#ver
                    }
                });
                quote! {
                    struct #classname (
                        #(#recurse),*
                    );
                }
            }
            Fields::Unit => {
                quote! {
                    struct #classname;
                }
            }
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}
*/
