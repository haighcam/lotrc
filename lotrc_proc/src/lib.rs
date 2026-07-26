use proc_macro2::{TokenStream, Group, TokenTree, Span};
use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::punctuated::Punctuated;
use syn::{
    parse_macro_input, spanned::Spanned, Data, DeriveInput, Fields, Ident, Index, Type, 
    Token, parse::{Parse, ParseStream, Parser}, ItemFn, ItemMod, ItemImpl, Item, Meta, ItemStruct, Field, DataStruct
};

fn ident_endian(ver: &str, ident: Ident) -> Ident {
    let mut s = ident.to_string();
    if s.ends_with("_XE_") {
        s.replace_range(s.len() - 4..s.len(), ver);
        Ident::new(&s, ident.span())
    } else if s.ends_with("_xe_") {
        s.replace_range(s.len() - 3..s.len(), &ver.to_lowercase());
        Ident::new(&s, ident.span())
    } else {
        ident
    }
}

fn replace_endian(ver: &str, item: TokenStream) -> TokenStream {
    TokenStream::from_iter(item.into_iter().map(|x| match x {
        TokenTree::Ident(ident) => TokenTree::Ident(ident_endian(ver, ident)),
        TokenTree::Group(group) => TokenTree::Group(Group::new(
            group.delimiter(),
            replace_endian(ver, group.stream()),
        )),
        tree => tree,
    }))
}

#[proc_macro_attribute]
pub fn make_endian(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item = TokenStream::from(item);

    let s = item.to_string();
    if s.contains("_XE_") || s.contains("_xe_") {
        let mut out = replace_endian("BE", item.clone());
        out.extend(replace_endian("LE", item.clone()));
        out.into()
    } else {
        item.into()
    }
}

struct ConvTraitInput {
    name: Ident,
    _comma: Token![,],
    fields: Punctuated<Field, Token![,]>
}

impl Parse for ConvTraitInput {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        Ok(Self {
            name: input.parse()?,
            _comma: input.parse()?,
            fields: Punctuated::parse_terminated_with(input, Field::parse_named)?,
        })
    }
}

fn get_conv_trait_name(name: &Ident) -> Ident {
    format_ident!("{}ConvTraitFromMacro", name)
}

fn make_trait_impl<'a>(name: &Ident, fields: impl IntoIterator<Item=&'a Field>) -> proc_macro::TokenStream {
    let mut trait_recur = vec![];
    let mut ty_recur = vec![];
    for Field { ident, ty, .. } in fields.into_iter() {
        let name = ident.clone().unwrap();
        let name_ref = format_ident!("{}_ref", name);
        let name_mut = format_ident!("{}_mut", name);
        let name_ty = format_ident!("{}_type", name);
        trait_recur.push(quote_spanned! { name.span() =>
            #[inline(always)]
            fn #name(&self) -> #name_ty { #name_ty::default() }
            #[inline(always)]
            fn #name_ref(&self) -> Option<&#name_ty> { None }
            #[inline(always)]
            fn #name_mut(&mut self) -> Option<&mut #name_ty> { None }
        });
        ty_recur.push(quote_spanned! { name.span() =>
            #[allow(non_camel_case_types)]
            pub type #name_ty = #ty;
        });
    }
    let trait_name = get_conv_trait_name(&name);
    quote! {
        pub(crate) mod #trait_name {
            use super::*;
            pub trait ConvTrait {
                #(#trait_recur)*
            }
            #(#ty_recur)*
        }
    }.into()
}

#[proc_macro]
pub fn make_conv_trait(attr: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ConvTraitInput { name, fields, .. } = parse_macro_input!(attr as ConvTraitInput);
    make_trait_impl(&name, &fields)
}

#[proc_macro_derive(FromConvImpl, attributes(create_conv_trait))]
pub fn derive_from_conv_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let add_impl = input.attrs.iter().find(|x| x.path().is_ident("create_conv_trait")).is_some();
    if let Data::Struct(DataStruct { fields: Fields::Named(fields), .. }) = input.data {
        let name = input.ident;
        let trait_name = get_conv_trait_name(&name);
        let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
        let trait_recurse = fields.named.iter().map(|f| {
            let name = f.ident.clone().unwrap();
            let name_ref = format_ident!("{}_ref", name);
            let name_mut = format_ident!("{}_mut", name);
            let name_ty = format_ident!("{}_type", name);
            quote_spanned! { f.span() => 
                #[inline(always)]
                fn #name(&self) -> #trait_name::#name_ty { self.#name.clone() }
                #[inline(always)]
                fn #name_ref(&self) -> Option<&#trait_name::#name_ty> { Some(&self.#name) }
                #[inline(always)]
                fn #name_mut(&mut self) -> Option<&mut #trait_name::#name_ty> { Some(&mut self.#name) }
            }
        });
        let from_recurse = fields.named.iter().map(|f| {
            let name = f.ident.clone().unwrap();
            quote_spanned! { f.span() => 
                #name: #trait_name::ConvTrait::#name(val)
            }
        });
        let mut val: proc_macro::TokenStream = quote! {
            impl #impl_generics #trait_name::ConvTrait for #name #ty_generics #where_clause {
                #(#trait_recurse)*
            }
            impl #impl_generics #name #ty_generics #where_clause {
                #[inline(always)]
                fn from_conv_impl(val: &impl #trait_name::ConvTrait) -> Self {
                    Self {
                        #(#from_recurse),*
                    }
                }
            }
        }.into();
        if add_impl {
            val.extend(make_trait_impl(&name, &fields.named))
        }
        val
    } else {
        quote! {}.into()
    }
}

#[proc_macro_derive(IntoConvImpl, attributes(conv_base))]
pub fn derive_into_conv_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let mut base_name = name.clone();
    if let Some(attr) = input.attrs.iter().find(|x| x.path().is_ident("conv_base")) {
        if let Meta::List(val) = &attr.meta {
            let input: proc_macro::TokenStream = val.tokens.clone().into();
            base_name = parse_macro_input!(input as Ident);
        }
    }

    if let Data::Struct(DataStruct { fields: Fields::Named(fields), .. }) = input.data {
        let make_endian_impl = if name == base_name {
            quote!{}
        } else {
            quote! { #[lotrc_proc::make_endian] }
        };
        let trait_name = get_conv_trait_name(&base_name);
        let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
        let trait_recurse = fields.named.iter().map(|f| {
            let name = f.ident.clone().unwrap();
            let name_ty = format_ident!("{}_type", name);
            quote_spanned! { f.span() => 
                #[inline(always)]
                fn #name(&self) -> #trait_name::#name_ty { self.#name.conv() }
            }
        });
        let from_recurse = fields.named.iter().map(|f| {
            let name = f.ident.clone().unwrap();
            quote_spanned! { f.span() => 
                #name: lotrc_wrappers::OrderedData::conv(&#trait_name::ConvTrait::#name(self))
            }
        });
        quote! {
            #make_endian_impl
            impl #impl_generics #trait_name::ConvTrait for #name #ty_generics #where_clause {
                #(#trait_recurse)*
            }
            #make_endian_impl
            impl #impl_generics lotrc_wrappers::OrderedData<#base_name #ty_generics> for #name #ty_generics #where_clause {
                #[inline(always)]
                fn conv(&self) -> #base_name {
                    #base_name::from_conv_impl(self)
                }
            }
            #make_endian_impl
            impl #impl_generics lotrc_wrappers::OrderedData<#name #ty_generics> for #base_name #ty_generics #where_clause {
                #[inline(always)]
                fn conv(&self) -> #name {
                    #name {
                        #(#from_recurse),*
                    }
                }
            }
        }.into()
    } else {
        quote! {}.into()
    }
}

#[proc_macro_attribute]
pub fn derive_ordered_data(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item_ = item.clone();
    let item_struct = parse_macro_input!(item_ as ItemStruct);
    let name_base = ident_endian("", item_struct.ident);
    let item: TokenStream = item.into();
    let base = replace_endian("", item.clone());
    quote! {
        #[derive(lotrc_proc::FromConvImpl)]
        #[create_conv_trait]
        #base

        #[lotrc_proc::make_endian]
        #[derive(lotrc_proc::IntoConvImpl, zerocopy::KnownLayout, zerocopy::Immutable, zerocopy::IntoBytes, zerocopy::FromBytes)]
        #[conv_base(#name_base)]
        #item
    }.into() 
}
