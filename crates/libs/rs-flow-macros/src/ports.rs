use std::fmt::{Formatter, Debug};

use proc_macro::Span;

use syn::punctuated::Punctuated;
use syn::token::Eq;
use syn::{
    braced, Ident, Lit, LitStr, Token
};
use syn::parse::{Parse, ParseStream};



struct IdentAttr {
    ident: Ident,
    literal: Lit,
}
impl Parse for IdentAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;
        let _: Eq = input.parse()?;
        let literal: Lit = input.parse()?;
        Ok(IdentAttr { ident, literal })
    }
}

pub struct Port {
    pub label: Ident,
    pub description: Option<LitStr>
}
impl Debug for Port {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        f.debug_struct("Port")
            .field("label", &self.label.to_string())
            .field("description", &self.description.as_ref().map(|l| l.token().to_string()))
            .finish()
    }
}


impl Parse for Port {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let label: Ident = input.parse()?;

        let lookahead = input.lookahead1();
        if  lookahead.peek(Token![=>]) {
            let _ :syn::token::FatArrow = input.parse()?;

            let content;
            let _ = braced!(content in input);
            let attrs: Punctuated<IdentAttr, Token![,]> = content.parse_terminated(IdentAttr::parse)?;

            let mut description = None;
            for attr in attrs {
                if attr.ident == Ident::new("description", Span::call_site().into()) {
                    if description.is_some() {
                        return Err(syn::Error::new(attr.ident.span(), "alredy defined."))
                    } else {
                        if let Lit::Str(lit) = attr.literal {
                            description = Some(lit);
                        } else {
                            return Err(syn::Error::new(attr.literal.span(), "Expect a string literal"))
                        }
                    }
                    continue;
                }

                return Err(syn::Error::new(attr.ident.span(), "Expect description"))
            }
            return Ok(Self { label, description });
        }
        Ok(Self { label, description: None })
    }
}

#[derive(Debug)]
pub(crate) struct Ports(pub(crate) Vec<Port>);

impl Parse for Ports {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ports: Punctuated<Port, Token![,]> = input.parse_terminated(Port::parse)?;

        Ok(Ports(ports.into_iter().collect()))
    }
}