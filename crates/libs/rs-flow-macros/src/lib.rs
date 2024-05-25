use proc_macro::TokenStream;

use quote::ToTokens;
use syn::parse_macro_input;
use syn::punctuated::Punctuated;
use syn::Data;
use syn::DeriveInput;
use quote::quote;

mod ports;
use ports::Ports;
use syn::Token;


#[proc_macro_attribute]
pub fn inputs(attr: TokenStream, item: TokenStream) -> TokenStream {
    let ports = parse_macro_input!(attr as Ports);
    
    let mut item_clone = item.clone();
    let input: DeriveInput = parse_macro_input!(item);

    let name = input.ident.clone();
    let _ = match input.data {
        Data::Union(_) => panic!("Inputs cannot be implemented in unions"),
        _ => {}
    };

    if !input.generics.params.is_empty() {
        panic!("Inputs cannot be implemented with generics");
    }
    
    let ports_vec = ports.0.iter().enumerate().map(|(i, port)| {
        let label = &port.label.to_string();
        let description = port.description.as_ref()
            .map(|desc| quote!{ Some(#desc) }.into_token_stream())
            .unwrap_or(quote!{ None }.into_token_stream());

        quote!{ rs_flow::ports::Port::from(#i as u16, #label, #description) }
            .into_token_stream()
    });
    let ports_vec = Punctuated::<_, Token![,]>::from_iter(ports_vec);
    
    let ports_match = match ports.0.len() {
        0 => { quote!{ panic!("This component not have a input port") }},
        1 => { 
            let label = ports.0.get(0).unwrap().label.to_string();
            quote!{ 
                if label == #label { 
                    0 
                } else { 
                    panic!("This component not have '{label}' input port");
                }
            }
        },
        _ => {
            let matches = ports.0.into_iter().enumerate().map(|(i, port)| {
                let i = i as u16;
                let label = port.label.to_string();
                quote!{
                    #label => { #i }
                }.into_token_stream()
            });
            let matches = Punctuated::<_, Token![,]>::from_iter(matches);

            quote! {
                match label {
                    #matches,
                    _ => { panic!("This component not have '{label}' input port"); }
                }
            }
        }
    }.into_token_stream();

    let expand: TokenStream = quote! {
        impl rs_flow::ports::Inputs for #name {
            fn inputs(&self) -> &rs_flow::ports::Ports {
                static INPUTS: std::sync::OnceLock<rs_flow::ports::Ports> = std::sync::OnceLock::new();
        
                INPUTS.get_or_init(|| {
                    rs_flow::ports::Ports::new(vec![#ports_vec])
                })
            }
            fn input(&self, label: &'static str) -> u16 {
                #ports_match
            }
        }
    }.into();

    item_clone.extend(expand);
    item_clone
}


#[proc_macro_attribute]
pub fn outputs(attr: TokenStream, item: TokenStream) -> TokenStream {
    let ports = parse_macro_input!(attr as Ports);
    
    let mut item_clone = item.clone();
    let input: DeriveInput = parse_macro_input!(item);

    let name = input.ident.clone();
    let _ = match input.data {
        Data::Union(_) => panic!("Outputs cannot be implemented in unions"),
        _ => {}
    };

    if !input.generics.params.is_empty() {
        panic!("Outputs cannot be implemented with generics");
    }
    
    let ports_vec = ports.0.iter().enumerate().map(|(i, port)| {
        let label = &port.label.to_string();
        let description = port.description.as_ref()
            .map(|desc| quote!{ Some(#desc) }.into_token_stream())
            .unwrap_or(quote!{ None }.into_token_stream());

        quote!{ rs_flow::ports::Port::from(#i as u16, #label, #description) }
            .into_token_stream()
    });
    let ports_vec = Punctuated::<_, Token![,]>::from_iter(ports_vec);
    
    let ports_match = match ports.0.len() {
        0 => { quote!{ panic!("This component not have a output port") }},
        1 => { 
            let label = ports.0.get(0).unwrap().label.to_string();
            quote!{ 
                if label == #label { 
                    0 
                } else { 
                    panic!("This component not have '{label}' output port");
                }
            }
        },
        _ => {
            let matches = ports.0.into_iter().enumerate().map(|(i, port)| {
                let i = i as u16;
                let label = port.label.to_string();
                quote!{
                    #label => { #i }
                }.into_token_stream()
            });
            let matches = Punctuated::<_, Token![,]>::from_iter(matches);

            quote! {
                match label {
                    #matches,
                    _ => { panic!("This component not have '{label}' output port"); }
                }
            }
        }
    }.into_token_stream();

    let expand: TokenStream = quote! {

        impl rs_flow::ports::Outputs for #name {
            fn outputs(&self) -> &rs_flow::ports::Ports {
                static OUTPUTS: std::sync::OnceLock<rs_flow::ports::Ports> = std::sync::OnceLock::new();
        
                OUTPUTS.get_or_init(|| {
                    rs_flow::ports::Ports::new(vec![#ports_vec])
                })
            }
            fn output(&self, label: &'static str) -> u16 {
                #ports_match
            }
        }
    }.into();

    item_clone.extend(expand);
    item_clone
}

