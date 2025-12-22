use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned, ToTokens};
use std::collections::HashSet;
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{
    parse_macro_input, spanned::Spanned, Data, DeriveInput, Fields, Ident, Index, ItemEnum,
    ItemImpl, ItemStruct, Token,
};
use syn::{Attribute, Visibility};

mod platforms;

#[proc_macro_attribute]
pub fn make_platforms(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    platforms::attr_impl(attr, item)
}

#[proc_macro_attribute]
pub fn staticmethod(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item
}
#[proc_macro_attribute]
pub fn getter(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item
}

fn pymethods(attrs: &HashSet<String>) -> Vec<TokenStream> {
    let mut fns = vec![
        quote! {
            pub fn __str__(&self) -> String {
                format!("{:?}", self)
            }
        },
        quote! {
            #[pyo3(name = "to_json")]
            pub fn _to_json(&self) -> Result<String> {
                <Self as crate::types::PyMethods>::to_json(self)
            }
        },
        quote! {
            #[staticmethod]
            #[pyo3(name = "from_json")]
            pub fn _from_json(s: &str) -> Result<Self> {
                <Self as crate::types::PyMethods>::from_json(s)
            }
        },
    ];
    if !attrs.contains("no_bytes") {
        fns.push(quote! {
            #[pyo3(name = "dump_bytes")]
            pub fn _dump_bytes(&self, args: <Self as crate::types::AsData>::OutArgs) -> Vec<u8> {
                <Self as crate::types::AsData>::dump_bytes::<crate::types::PC>(self, args)
            }
        });
        fns.push(quote! {
            #[staticmethod]
            #[pyo3(name = "from_bytes")]
            pub fn _from_bytes(val: &[u8], args: <Self as crate::types::AsData>::InArgs) -> Result<Self> {
                <Self as crate::types::AsData>::from_bytes::<crate::types::PC>(val, args)
            }
        });
        fns.push(quote! {
            #[pyo3(name = "size_bytes")]
            pub fn _size(&self) -> usize {
                <Self as crate::types::AsData>::size::<crate::types::PC>(self)
            }
        });
    }
    if !attrs.contains("no_new") {
        fns.push(quote! {
            #[new]
            pub fn _default() -> Self {
                Self::default()
            }
        });
    }
    fns
}

struct Args {
    vars: HashSet<String>,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        let vars = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;
        let vars = vars.into_iter().map(|x| x.to_string()).collect();
        Ok(Args { vars })
    }
}

#[proc_macro_attribute]
pub fn basicpymethods(
    args: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut args = parse_macro_input!(args as Args);
    if let Ok(mut items) = syn::parse::<ItemImpl>(item.clone()) {
        items.items.extend(
            pymethods(&args.vars)
                .into_iter()
                .map(|x| syn::parse(x.into()).unwrap()),
        );
        items.into_token_stream().into()
    } else if let Ok(input) = syn::parse::<ItemEnum>(item.clone()) {
        let name = input.ident.clone();
        args.vars.insert("no_new".to_string());
        let fns = TokenStream::from_iter(pymethods(&args.vars));
        quote! {
            #input

            #[pymethods]
            impl #name {
                #fns
            }
        }
        .into()
    } else if let Ok(input) = syn::parse::<ItemStruct>(item.clone()) {
        let name = input.ident.clone();
        let fns = TokenStream::from_iter(pymethods(&args.vars));
        quote! {
            #input

            #[pymethods]
            impl #name {
                #fns
            }
        }
        .into()
    } else {
        item
    }
}

#[proc_macro_derive(PyMethods)]
pub fn derive_pymethods(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    quote! {
        impl <'a> crate::types::PyMethods<'a> for #name {
            fn from_json(s: &'a str) -> Result<Self> {
                Ok(serde_json::from_str(s)?)
            }
            fn to_json(&self) -> Result<String> {
                Ok(serde_json::to_string_pretty(&self)?)
            }
        }
    }
    .into()
}

#[proc_macro_derive(OrderedData, attributes(ordered_data, name_pc, name_xbox, name_ps3))]
pub fn derive_ordered_data_fn(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let pc = format_ident!("PC");
    let xbox = format_ident!("XBOX");
    let ps3 = format_ident!("PS3");
    let name_pc = format_ident!("{}Pc", name);
    let name_xbox = format_ident!("{}Xbox", name);
    let name_ps3 = format_ident!("{}Ps3", name);

    let vis = &input.vis;

    let alt_class_pc = alt_class_def(&input.data, &name_pc, &pc, &vis);
    let alt_class_xbox = alt_class_def(&input.data, &name_xbox, &xbox, &vis);
    let alt_class_ps3 = alt_class_def(&input.data, &name_ps3, &ps3, &vis);
    let conv_pc: TokenStream = conv_def(&input.data, &pc);
    let conv_back_pc = conv_back_def(&input.data, &pc);
    let conv_xbox: TokenStream = conv_def(&input.data, &xbox);
    let conv_back_xbox = conv_back_def(&input.data, &xbox);
    let conv_ps3: TokenStream = conv_def(&input.data, &ps3);
    let conv_back_ps3 = conv_back_def(&input.data, &ps3);

    let get_pc = get_def(&input, &pc);
    let get_xbox = get_def(&input, &pc);
    let get_ps3 = get_def(&input, &ps3);

    let expanded = quote! {
        #[repr(C)]
        #[derive(Default, Debug, Clone, zerocopy::KnownLayout, zerocopy::FromBytes, zerocopy::IntoBytes, zerocopy::Unaligned, zerocopy::Immutable)]
        #vis #alt_class_pc
        impl #name_pc {
            #vis #get_pc
        }
        impl From<#name_pc> for #name {
            fn from(value: #name_pc) -> Self {
                #conv_back_pc
            }
        }
        impl From<#name> for #name_pc {
            fn from(value: #name) -> Self {
                #conv_pc
            }
        }
        impl From<&#name_pc> for #name {
            fn from(value: &#name_pc) -> Self {
                #conv_back_pc
            }
        }
        impl From<&#name> for #name_pc {
            fn from(value: &#name) -> Self {
                #conv_pc
            }
        }
        #[cfg(feature = "python")]
        impl <'py> pyo3::IntoPyObject<'py> for #name_pc {
            type Target = <#name as pyo3::IntoPyObject<'py>>::Target;
            type Output = <#name as pyo3::IntoPyObject<'py>>::Output;
            type Error = <#name as pyo3::IntoPyObject<'py>>::Error;
            #[inline(always)]
            fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
                #name::from(self).into_pyobject(py)
            }
        }
        #[cfg(feature = "python")]
        impl <'a, 'py> pyo3::IntoPyObject<'py> for &'a #name_pc {
            type Target = <#name as pyo3::IntoPyObject<'py>>::Target;
            type Output = <#name as pyo3::IntoPyObject<'py>>::Output;
            type Error = <#name as pyo3::IntoPyObject<'py>>::Error;
            #[inline(always)]
            fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
                <#name>::from(self).into_pyobject(py)
            }
        }

        #[repr(C)]
        #[derive(Default, Debug, Clone, zerocopy::KnownLayout, zerocopy::FromBytes, zerocopy::IntoBytes, zerocopy::Unaligned, zerocopy::Immutable)]
        #vis #alt_class_xbox
        impl #name_xbox {
            #vis #get_xbox
        }
        impl From<#name_xbox> for #name {
            fn from(value: #name_xbox) -> Self {
                #conv_back_xbox
            }
        }
        impl From<#name> for #name_xbox {
            fn from(value: #name) -> Self {
                #conv_xbox
            }
        }
        impl From<&#name_xbox> for #name {
            fn from(value: &#name_xbox) -> Self {
                #conv_back_xbox
            }
        }
        impl From<&#name> for #name_xbox {
            fn from(value: &#name) -> Self {
                #conv_xbox
            }
        }
        #[cfg(feature = "python")]
        impl <'py> pyo3::IntoPyObject<'py> for #name_xbox {
            type Target = <#name as pyo3::IntoPyObject<'py>>::Target;
            type Output = <#name as pyo3::IntoPyObject<'py>>::Output;
            type Error = <#name as pyo3::IntoPyObject<'py>>::Error;
            #[inline(always)]
            fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
                #name::from(self).into_pyobject(py)
            }
        }
        #[cfg(feature = "python")]
        impl <'a, 'py> pyo3::IntoPyObject<'py> for &'a #name_xbox {
            type Target = <#name as pyo3::IntoPyObject<'py>>::Target;
            type Output = <#name as pyo3::IntoPyObject<'py>>::Output;
            type Error = <#name as pyo3::IntoPyObject<'py>>::Error;
            #[inline(always)]
            fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
                #name::from(self).into_pyobject(py)
            }
        }

        #[repr(C)]
        #[derive(Default, Debug, Clone, zerocopy::KnownLayout, zerocopy::FromBytes, zerocopy::IntoBytes, zerocopy::Unaligned, zerocopy::Immutable)]
        #vis #alt_class_ps3
        impl #name_ps3 {
            #vis #get_ps3
        }
        impl From<#name_ps3> for #name {
            fn from(value: #name_ps3) -> Self {
                #conv_back_ps3
            }
        }
        impl From<#name> for #name_ps3 {
            fn from(value: #name) -> Self {
                #conv_ps3
            }
        }
        impl From<&#name_ps3> for #name {
            fn from(value: &#name_ps3) -> Self {
                #conv_back_ps3
            }
        }
        impl From<&#name> for #name_ps3 {
            fn from(value: &#name) -> Self {
                #conv_ps3
            }
        }
        #[cfg(feature = "python")]
        impl <'py> pyo3::IntoPyObject<'py> for #name_ps3 {
            type Target = <#name as pyo3::IntoPyObject<'py>>::Target;
            type Output = <#name as pyo3::IntoPyObject<'py>>::Output;
            type Error = <#name as pyo3::IntoPyObject<'py>>::Error;
            #[inline(always)]
            fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
                #name::from(self).into_pyobject(py)
            }
        }
        #[cfg(feature = "python")]
        impl <'a, 'py> pyo3::IntoPyObject<'py> for &'a #name_ps3 {
            type Target = <#name as pyo3::IntoPyObject<'py>>::Target;
            type Output = <#name as pyo3::IntoPyObject<'py>>::Output;
            type Error = <#name as pyo3::IntoPyObject<'py>>::Error;
            #[inline(always)]
            fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
                #name::from(self).into_pyobject(py)
            }
        }

        impl crate::types::OrderedDataStrict for #name {
            type PC = #name_pc;
            type XBOX = #name_xbox;
            type PS3 = #name_ps3;
        }
    };

    proc_macro::TokenStream::from(expanded)
}

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
        if val == "PC" || val == "XBOX" || val == "PS3" {
            val_ver = val;
        } else if val == "skipPC" && ver == "PC" {
            skip = true;
        } else if val == "skipXBOX" && ver == "XBOX" {
            skip = true;
        } else if val == "skipPS3" && ver == "PS3" {
            skip = true;
        }
    }
    let alt_name = if ver == "PC" {
        attrs
            .iter()
            .filter(|a| a.path().is_ident("name_pc"))
            .flat_map(|a| {
                a.parse_args_with(Punctuated::<Ident, Token![,]>::parse_terminated)
                    .unwrap()
            })
            .next()
    } else if ver == "XBOX" {
        attrs
            .iter()
            .filter(|a| a.path().is_ident("name_xbox"))
            .flat_map(|a| {
                a.parse_args_with(Punctuated::<Ident, Token![,]>::parse_terminated)
                    .unwrap()
            })
            .next()
    } else if ver == "PS3" {
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
                            f.span() => #alt_name: Into::into(&value.#alt_name)
                        })
                    } else {
                        None
                    }
                });
                quote! {
                    Self {
                        #(#recurse),*,
                        ..Default::default()
                    }
                }
            }
            Fields::Unnamed(ref fields) => {
                let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                    let index = Index::from(i);
                    quote_spanned! {
                        f.span() => Into::into(&value.#index)
                    }
                });
                quote! {
                    Self (
                        #(#recurse),*
                    )
                }
            }
            Fields::Unit => {
                quote! {
                    Self
                }
            }
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}

fn get_def(input: &DeriveInput, ver: &Ident) -> TokenStream {
    let classname = &input.ident;
    let body = match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().filter_map(|f| {
                    let name = &f.ident;
                    let (_, skip, alt_name) = filter_attrs(&f.attrs, ver, name.clone().unwrap());
                    if !skip {
                        Some(quote_spanned! {
                            //f.span() => #alt_name: crate::types::OrderedDataRef::into(&value.#alt_name, ())
                            f.span() => #alt_name: Into::into(&self.#alt_name)
                        })
                    } else {
                        None
                    }
                });
                quote! {
                    #classname {
                        #(#recurse),*,
                        ..Default::default()
                    }
                }
            }
            Fields::Unnamed(ref fields) => {
                let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                    let index = Index::from(i);
                    quote_spanned! {
                        f.span() => Into::into(&self.#index)
                    }
                });
                quote! {
                    #classname (
                        #(#recurse),*
                    )
                }
            }
            Fields::Unit => {
                quote! {
                   #classname
                }
            }
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    };
    quote! {
        fn get(&self) -> #classname {
           #body
        }
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
                            //f.span() => #alt_name: crate::types::OrderedDataRef::into(&value.#alt_name, ())
                            f.span() => #alt_name: Into::into(&value.#alt_name)
                        })
                    } else {
                        None
                    }
                });
                quote! {
                    Self {
                        #(#recurse),*,
                        ..Default::default()
                    }
                }
            }
            Fields::Unnamed(ref fields) => {
                let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                    let index = Index::from(i);
                    quote_spanned! {
                        f.span() => Into::into(&value.#index)
                    }
                });
                quote! {
                    Self (
                        #(#recurse),*
                    )
                }
            }
            Fields::Unit => {
                quote! {
                    Self
                }
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
                            f.span() => #vis #alt_name: <#ty as crate::types::OrderedDataStrict>::#val
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
                        f.span() => #vis <#ty as crate::types::OrderedDataStrict>::#ver
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
