extern crate proc_macro;

use quote::{quote, ToTokens};
use syn::{*, parse::{Parse, ParseStream}};
use proc_macro2::{Span, Ident};


struct Port {
    _name: Ident
}
struct Ports {
    ports: Vec<Port>
}
struct ComponentAttribute {
    inputs: Option<Ports>,
    outputs: Option<Ports>,
}


impl Parse for Port {
    fn parse(input: ParseStream) -> Result<Port> {
        let name: Ident = input.parse()?;
        Ok(Port { _name : name })
    }
}
impl Parse for Ports {
    fn parse(input: ParseStream) -> Result<Ports> {
        let content;
        bracketed!(content in input);
        let mut ports = Vec::new();
        for port in content.parse_terminated::<Port, token::Comma>(Port::parse)? {
            ports.push(port)
        }
        Ok(Ports { ports })
    }
}


#[proc_macro_derive(Component, attributes(inputs, outputs))]
pub fn derive_component(item: proc_macro::TokenStream) -> proc_macro::TokenStream  {
    let derive = parse_macro_input!(item as DeriveInput);
    
    let name = derive.ident;
    let component_attrs = get_componets_attributes(derive.attrs);

    let (inputs, outputs) = construct_ports(component_attrs);

    let component_name = name.to_string() + "Component";
    let component_name = Ident::new(component_name.as_str(), Span::call_site());

    let tokens = quote!(
        pub struct #component_name<__GlobalType> {
            id: rs_flow::component::ComponentId,
            
            inputs: Vec<rs_flow::port::InPort>,
            outputs: Vec<rs_flow::port::OutPort>,
            
            data: #name,
            context: Option<rs_flow::flow::ComponentContext<__GlobalType>>
        }

        impl<__GlobalType> #component_name<__GlobalType> {
            pub fn new(id: rs_flow::component::ComponentId, data: #name) -> Self {
                Self {
                    id,
                    context: None,
                    inputs: vec![#(#inputs)*],
                    outputs: vec![#(#outputs)*],
                    data
                }
            }
        }

        impl<__GlobalType> rs_flow::component::ComponentDefault<__GlobalType> for #component_name<__GlobalType> {
            fn id(&self) -> rs_flow::component::ComponentId { self.id }
            fn inputs(&self) -> &Vec<rs_flow::port::InPort> {
                &self.inputs
            }
            fn outputs(&self) -> &Vec<rs_flow::port::OutPort> {
                &self.outputs
            }
            fn context(&self) -> Result<&rs_flow::flow::ComponentContext<__GlobalType>, rs_flow::errors::Errors> {
                if let Some(context) = &self.context {
                    return Ok(context);
                }
                Err(rs_flow::errors::Errors::ContextNotLoaded)
            }
            fn set_context(&mut self, context: rs_flow::flow::ComponentContext<__GlobalType>) {
                self.context = Some(context);
            }
        }  
    ); 
        
    return tokens.into();
}

fn construct_ports(component_attrs: ComponentAttribute) -> (Vec<__private::TokenStream2>, Vec<__private::TokenStream2>) {
    
    let inputs: Vec<__private::TokenStream2> = component_attrs.inputs
        .unwrap_or(Ports { ports: Vec::new() }).ports
        .iter()
        .enumerate()
        .map(|(index, _)| { 
            if index == 0 {
                quote!(rs_flow::port::InPort::new(#index as rs_flow::port::PortId))
            } else {
                quote!(, rs_flow::port::InPort::new(#index as rs_flow::port::PortId))
            }
        })
        .collect();

    let outputs: Vec<__private::TokenStream2> = component_attrs.outputs
        .unwrap_or(Ports { ports: Vec::new() }).ports
        .iter().enumerate()
        .map(|(index, _)| { 
            if index == 0 {
                quote!(rs_flow::port::OutPort::new(#index as rs_flow::port::PortId))
            } else {
                quote!(, rs_flow::port::OutPort::new(#index as rs_flow::port::PortId))
            }
        })
        .collect();
    
    (inputs, outputs)
}

fn get_componets_attributes(attrs: Vec<Attribute>) -> ComponentAttribute {
    let mut component = ComponentAttribute {
        inputs: None, 
        outputs: None 
    };

    for attr in attrs {
        let tokens = attr.tokens;
        match attr.path.into_token_stream().to_string().as_str() {
            "inputs" => {
                if component.inputs.is_some() {
                    panic!("'outputs' alrealdy defined");    
                }
                let ports = parse(tokens.into()).unwrap();
                component.inputs = Some(ports);   
            },
            "outputs" => {
                if component.outputs.is_some() {
                    panic!("'outputs' alrealdy defined");    
                }
                let ports = parse(tokens.into()).unwrap();
                component.outputs = Some(ports);
            },
            _ => panic!()
        }
    }

    return component;
}