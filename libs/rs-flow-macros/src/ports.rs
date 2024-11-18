use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Fields, LitStr};

type Result<T> = core::result::Result<T, syn::Error>;

pub enum Ports {
    Inputs,
    Outputs,
}

fn impl_unit_struct(input: DeriveInput, port_trait: Ports) -> Result<TokenStream> {
    let ty = &input.ident;
    let trait_name = match port_trait {
        Ports::Inputs => quote! { ::rs_flow::ports::Inputs },
        Ports::Outputs => quote! { ::rs_flow::ports::Outputs },
    };

    let label = ty.to_string();

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let description = input
        .attrs
        .iter()
        .find(|attr| attr.path.is_ident("description"));

    let description = if let Some(attr) = description {
        let description: LitStr = attr.parse_args()?;
        quote! { Some(#description) }
    } else {
        quote! { None }
    };

    let token = quote! {
        impl #impl_generics #trait_name for #ty #ty_generics #where_clause {
            const PORTS: ::rs_flow::ports::Ports = ::rs_flow::ports::Ports::new(&[
                ::rs_flow::ports::Port::from(0, #label, #description)
            ]);

            fn into_port(&self) -> ::rs_flow::ports::PortId {
                0
            }
        }
    };

    Ok(token.into())
}

fn impl_enum(input: DeriveInput, port_trait: Ports) -> Result<TokenStream> {
    let ty = &input.ident;
    let trait_name = match port_trait {
        Ports::Inputs => quote! { ::rs_flow::ports::Inputs },
        Ports::Outputs => quote! { ::rs_flow::ports::Outputs },
    };

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let data = if let Data::Enum(data) = input.data {
        data
    } else {
        unreachable!()
    };

    let mut ports = Vec::<TokenStream>::with_capacity(data.variants.len());
    let mut intos = Vec::<TokenStream>::with_capacity(data.variants.len());

    for (index, variant) in data.variants.into_iter().enumerate() {
        if let Fields::Unit = variant.fields {
            let ident = variant.ident;

            let id = index as u16;
            let label = ident.to_string();
            let description = variant
                .attrs
                .into_iter()
                .find(|attr| attr.path.is_ident("description"));

            let description = if let Some(attr) = description {
                let description: LitStr = attr.parse_args()?;
                quote! { Some(#description) }
            } else {
                quote! { None }
            };

            ports.push(quote! { ::rs_flow::ports::Port::from(#id, #label, #description), });
            intos.push(quote! { Self::#ident => #id, })
        } else {
            return Err(syn::Error::new(
                variant.ident.span(),
                match port_trait {
                    Ports::Inputs => "Derive 'Inputs' only support in Unit Variants",
                    Ports::Outputs => "Derive 'Outputs' only support in Unit Variants",
                },
            ));
        }
    }

    let intos = match intos.len() {
        0 => match port_trait {
            Ports::Inputs => quote! { panic!("Component not have a input port"); },
            Ports::Outputs => quote! { panic!("Component not have a output port"); },
        },
        1 => {
            quote! { 0 }
        }
        _ => {
            quote! {
                match self {
                    #(#intos)*
                }
            }
        }
    };

    let token = quote! {
        impl #impl_generics #trait_name for #ty #ty_generics #where_clause {
            const PORTS: ::rs_flow::ports::Ports = ::rs_flow::ports::Ports::new(&[
                #(#ports)*
            ]);

            fn into_port(&self) -> ::rs_flow::ports::PortId {
                #intos
            }
        }
    };

    Ok(token)
}

fn try_expand(input: DeriveInput, port_trait: Ports) -> Result<TokenStream> {
    let err = match port_trait {
        Ports::Inputs => "Derive 'Inputs' only supported for enums or unit structs",
        Ports::Outputs => "Derive 'Outputs' only supported for enums or unit structs",
    };
    match &input.data {
        syn::Data::Enum(_) => impl_enum(input, port_trait),
        syn::Data::Struct(DataStruct { fields, .. }) => match fields {
            Fields::Unit => impl_unit_struct(input, port_trait),
            _ => Err(syn::Error::new(input.ident.span(), err)),
        },
        _ => Err(syn::Error::new(input.ident.span(), err)),
    }
}

pub(crate) fn derive_ports(input: DeriveInput, port_trait: Ports) -> TokenStream {
    match try_expand(input, port_trait) {
        Ok(expand) => expand,
        Err(error) => {
            let error = error.to_compile_error();

            let token = quote! {
                #error
            };

            token.into()
        }
    }
}
