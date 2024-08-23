use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::punctuated::Punctuated;
use syn::Attribute;
use syn::{parse_macro_input, spanned::Spanned, Data, DeriveInput, Fields,Ident, Index, Token};


#[proc_macro_derive(OrderedData, attributes(ordered_data, name_pc, name_xbox, name_ps3))]
pub fn derive_ordered_data_fn(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let pc = format_ident!("PC");
    let xbox = format_ident!("XBOX");
    let ps3 = format_ident!("PS3");
    let name_pc = format_ident!("{}PC", name);
    let name_xbox = format_ident!("{}XBOX", name);
    let name_ps3 = format_ident!("{}PS3", name);

    let vis = input.vis;

    let alt_class_pc = alt_class_def(&input.data, &name_pc, &pc);
    let alt_class_xbox = alt_class_def(&input.data, &name_xbox, &xbox);
    let alt_class_ps3 = alt_class_def(&input.data, &name_ps3, &ps3);
    let conv_pc: TokenStream = conv_def(&input.data, &pc);
    let conv_back_pc = conv_back_def(&input.data, &pc);
    let conv_xbox: TokenStream = conv_def(&input.data, &xbox);
    let conv_back_xbox = conv_back_def(&input.data, &xbox);
    let conv_ps3: TokenStream = conv_def(&input.data, &ps3);
    let conv_back_ps3 = conv_back_def(&input.data, &ps3);

    let expanded = quote! {
        #[repr(C)]
        #[derive(Default, Debug, Clone, zerocopy::FromZeroes, zerocopy::FromBytes, zerocopy::AsBytes)]
        #vis #alt_class_pc

        #[repr(C)]
        #[derive(Default, Debug, Clone, zerocopy::FromZeroes, zerocopy::FromBytes, zerocopy::AsBytes)]
        #vis #alt_class_xbox

        #[repr(C)]
        #[derive(Default, Debug, Clone, zerocopy::FromZeroes, zerocopy::FromBytes, zerocopy::AsBytes)]
        #vis #alt_class_ps3


        impl OrderedDataImpl for #name {
            type PC = #name_pc;
            type XBOX = #name_xbox;
            type PS3 = #name_ps3;
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
    };

    proc_macro::TokenStream::from(expanded)
}

fn filter_attrs(attrs: &Vec<Attribute>, endian: &Ident, name: Ident) -> (Ident, bool, Ident) {
    let mut val_endian = endian.clone();
    let mut skip = false;
    for val in attrs.iter().filter(|a| a.path().is_ident("ordered_data")).flat_map(|a| {
        a.parse_args_with(Punctuated::<Ident, Token![,]>::parse_terminated)
            .unwrap()
    }) {
        if val == "PC" || val == "XBOX" || val == "PS3" {
            val_endian = val;
        } else if val == "skipPC" && endian == "PC" {
            skip = true;
        } else if val == "skipXBOX" && endian == "XBOX" {
            skip = true;
        } else if val == "skipPS3" && endian == "PS3" {
            skip = true;
        }
    }
    // match attrs.iter().filter(|a| a.path().is_ident("ordered_data")).flat_map(|a| {
    //     a.parse_args_with(Punctuated::<Ident, Token![,]>::parse_terminated)
    //         .unwrap()
    // }).next() {
    //     Some(val) => {
    //         if val == "PC" || val == "XBOX" || val == "PS3" {
    //             val_endian = val;
    //         } else if val == "skipPC" && endian == "PC" {
    //             skip = true;
    //         } else if val == "skipXBOX" && endian == "XBOX" {
    //             skip = true;
    //         } else if val == "skipPS3" && endian == "PS3" {
    //             skip = true;
    //         }    
    //     }
    //     None => ()
    // }
    let alt_name = if endian == "PC" {
        attrs.iter().filter(|a| a.path().is_ident("name_pc")).flat_map(|a| {
            a.parse_args_with(Punctuated::<Ident, Token![,]>::parse_terminated)
                .unwrap()
        }).next()
    } else if endian == "XBOX" {
        attrs.iter().filter(|a| a.path().is_ident("name_xbox")).flat_map(|a| {
            a.parse_args_with(Punctuated::<Ident, Token![,]>::parse_terminated)
                .unwrap()
        }).next()
    } else if endian == "PS3" {
        attrs.iter().filter(|a| a.path().is_ident("name_ps3")).flat_map(|a| {
            a.parse_args_with(Punctuated::<Ident, Token![,]>::parse_terminated)
                .unwrap()
        }).next()
    } else {
        None
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
                                f.span() => #name: <#ty as OrderedDataImpl>::#val
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
                            f.span() => <#ty as OrderedDataImpl>::#endian
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