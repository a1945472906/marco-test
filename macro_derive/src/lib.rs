extern crate proc_macro;
use proc_macro::{TokenStream};
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields};



#[proc_macro_derive(CheckStringFields)]
pub fn macro_derive(input: TokenStream) -> TokenStream{
    impl_check_macro(input)
}

fn impl_check_macro(input: TokenStream) -> TokenStream{
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    let fields = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => {
            panic!("expected a struct with named fields")
        }
    };
    let fields = fields.iter().map(|field|(&field.ident));
    TokenStream::from(quote!{
        impl #struct_name{
            fn is_empty_string(value: Box<dyn std::any::Any>) -> bool{
                if let Ok(s) = value.downcast::<String>(){
                    s.is_empty()
                } else {
                    false
                }
            }
        }
        impl CheckStringFields for #struct_name{
            fn check(&self) -> bool {
                #(
                    if  #struct_name::is_empty_string(Box::new(self.#fields.clone())){
                        return false;
                    }
                )*
                true
            }
        }
    })
}