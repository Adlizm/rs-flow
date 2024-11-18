use proc_macro;
use syn::{parse_macro_input, DeriveInput};

mod ports;

#[proc_macro_derive(Inputs, attributes(description))]
pub fn derive_inputs(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    ports::derive_ports(input, ports::Ports::Inputs).into()
}

#[proc_macro_derive(Outputs, attributes(description))]
pub fn derive_outputs(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    ports::derive_ports(input, ports::Ports::Outputs).into()
}
