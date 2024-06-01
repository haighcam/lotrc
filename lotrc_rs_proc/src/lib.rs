use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::punctuated::Punctuated;
use syn::Attribute;
use syn::{parse_macro_input, spanned::Spanned, Data, DeriveInput, Fields,Ident, Index, Token};


#[proc_macro_derive(OrderedData, attributes(ordered_data, name_be, name_le))]
pub fn derive_ordered_data_fn(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let little_endian = format_ident!("LE");
    let big_endian = format_ident!("BE");
    let name_le = format_ident!("{}LE", name);
    let name_be = format_ident!("{}BE", name);

    let vis = input.vis;

    // let generics = conv_from(input.generics);
    let alt_class_le = alt_class_def(&input.data, &name_le, &little_endian);
    let alt_class_be = alt_class_def(&input.data, &name_be, &big_endian);
    let conv_le: TokenStream = conv_def(&input.data, &little_endian);
    let conv_back_le = conv_back_def(&input.data, &little_endian);
    let conv_be = conv_def(&input.data, &big_endian);
    let conv_back_be = conv_back_def(&input.data, &big_endian);

    let expanded = quote! {
        #[repr(C)]
        #[derive(Default, Debug, Clone, zerocopy::FromZeroes, zerocopy::FromBytes, zerocopy::AsBytes)]
        #vis #alt_class_le

        #[repr(C)]
        #[derive(Default, Debug, Clone, zerocopy::FromZeroes, zerocopy::FromBytes, zerocopy::AsBytes)]
        #vis #alt_class_be

        impl OrderedData for #name {
            type LE = #name_le;
            type BE = #name_be;
        }

        impl From<#name_le> for #name {
            fn from(value: #name_le) -> Self {
                #conv_back_le
            }
        }

        impl From<#name> for #name_le {
            fn from(value: #name) -> Self {
                #conv_le
            }
        }

        impl From<#name_be> for #name {
            fn from(value: #name_be) -> Self {
                #conv_back_be
            }
        }

        impl From<#name> for #name_be {
            fn from(value: #name) -> Self {
                #conv_be
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

fn filter_attrs(attrs: &Vec<Attribute>, endian: &Ident, name: Ident) -> (Ident, bool, Ident) {
    let mut val_endian = endian.clone();
    let mut skip = false;
    match attrs.iter().filter(|a| a.path().is_ident("ordered_data")).flat_map(|a| {
        a.parse_args_with(Punctuated::<Ident, Token![,]>::parse_terminated)
            .unwrap()
    }).next() {
        Some(val) => {
            if val == "LE" || val == "BE" {
                val_endian = val;
            } else if val == "skipBE" && endian == "BE" {
                skip = true;
            } else if val == "skipLE" && endian == "LE" {
                skip = true;
            }
        }
        None => ()
    }
    let alt_name = if endian == "BE" {
        attrs.iter().filter(|a| a.path().is_ident("name_be")).flat_map(|a| {
            a.parse_args_with(Punctuated::<Ident, Token![,]>::parse_terminated)
                .unwrap()
        }).next()
    } else {
        attrs.iter().filter(|a| a.path().is_ident("name_le")).flat_map(|a| {
            a.parse_args_with(Punctuated::<Ident, Token![,]>::parse_terminated)
                .unwrap()
        }).next()
    }.unwrap_or(name);

    (val_endian, skip, alt_name)
}

fn conv_def(data: &Data, endian: &Ident) -> TokenStream {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let recurse = fields.named.iter().filter_map(|f| {
                        let name = &f.ident;
                        let (_, skip, alt_name) = filter_attrs(&f.attrs, endian, name.clone().unwrap());
                        if !skip {
                            Some(quote_spanned! {
                                f.span() => #name: value.#alt_name.into()
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
                },
                Fields::Unnamed(ref fields) => {
                    let recurse = fields.unnamed.iter().enumerate().map(|(i,f)| {
                        let index = Index::from(i);
                        quote_spanned! {
                            f.span() => value.#index.into()
                        }  
                    });
                    quote! {
                        Self (
                            #(#recurse),*
                        )
                    }
                },
                Fields::Unit => {
                    quote! {
                        Self
                    }
                }
            }
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}

fn conv_back_def(data: &Data, endian: &Ident) -> TokenStream {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let recurse = fields.named.iter().filter_map(|f| {
                        let name = &f.ident;
                        let (_, skip, alt_name) = filter_attrs(&f.attrs, endian, name.clone().unwrap());
                        if !skip {
                            Some(quote_spanned! {
                                f.span() => #alt_name: value.#name.into()
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
                },
                Fields::Unnamed(ref fields) => {
                    let recurse = fields.unnamed.iter().enumerate().map(|(i,f)| {
                        let index = Index::from(i);
                        quote_spanned! {
                            f.span() => value.#index.into()
                        }  
                    });
                    quote! {
                        Self (
                            #(#recurse),*
                        )
                    }
                },
                Fields::Unit => {
                    quote! {
                        Self
                    }
                }
            }
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}


fn alt_class_def(data: &Data, classname: &Ident, endian: &Ident) -> TokenStream {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let recurse = fields.named.iter().filter_map(|f| {
                        let name = &f.ident;
                        let ty = &f.ty;
                        let (val, skip, _) = filter_attrs(&f.attrs, endian, name.clone().unwrap());
                        if !skip {
                            Some(quote_spanned! {
                                f.span() => #name: <#ty as OrderedData>::#val
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
                },
                Fields::Unnamed(ref fields) => {
                    let recurse = fields.unnamed.iter().map(|f| {
                        let ty = &f.ty;
                        quote_spanned! {
                            f.span() => <#ty as OrderedData>::#endian
                        }

                    });
                    quote! {
                        struct #classname (
                            #(#recurse),*
                        );
                    }
                },
                Fields::Unit => {
                    quote! {
                        struct #classname;
                    }
                }
            }
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}