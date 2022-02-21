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
    let field_name = fields.iter().map(|field| &field.ident);
    TokenStream::from(quote! {
        impl #struct_name{
            fn check_empty(_s: &String) -> bool {
                _s.is_empty()
            }
            
            fn is_string<T: std::any::Any>(_s:&T) -> bool{
                std::any::TypeId::of::<String>() == std::any::TypeId::of::<T>()
            }
        }
        impl CheckStringFields for #struct_name{
            fn check(&self) -> bool{
                #(
                    if #struct_name::is_string(&self.#field_name){
                        // let _a:&String =&self.#field_name;
                        if #struct_name::check_empty(&self.#field_name){
                            return false;
                        }
                    }
                )*
                true
            }
        }
    })
}