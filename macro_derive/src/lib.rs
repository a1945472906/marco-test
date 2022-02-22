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
        trait CheckEmptyAny{
            fn is_empty_string(&self) -> bool;
        }
        impl <T> CheckEmptyAny for &T{
            fn is_empty_string(&self) -> bool{
                false
            }
        }
        trait CheckEmptyString {
            fn is_empty_string(&self) -> bool;
        }
        impl CheckEmptyString for String{
            fn is_empty_string(&self) -> bool {
                self.is_empty()
            }
        }
        impl CheckStringFields for #struct_name{
            fn check(&self) -> bool {
                #(
                    if (&self.#fields).is_empty_string(){
                        return false
                    }
                )*
                true
            }
        }
    })
}