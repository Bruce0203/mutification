use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, GenericParam, ItemStruct};

#[proc_macro_derive(ToMut)]
pub fn derive_as_mut(input: TokenStream) -> TokenStream {
    let item_struct = parse_macro_input!(input as ItemStruct);
    let name = &item_struct.ident;
    let params_in_impl = &item_struct.generics.params;
    let mut params = params_in_impl.clone();
    for param in params.iter_mut() {
        match param {
            GenericParam::Lifetime(value) => {
                value.bounds.clear();
            }
            GenericParam::Type(value) => {
                value.bounds.clear();
            }
            _ => {}
        }
    }
    let where_clause = item_struct.generics.where_clause;
    quote! {
        impl<#params_in_impl> mutification::ToMut for #name<#params> #where_clause {
            fn to_mut<'mutification_to_mut>(&'mutification_to_mut self) -> &'mutification_to_mut mut Self {
                unsafe { &mut *(self as *const Self as *mut Self) }
            }
        }
    }
    .into()
}
