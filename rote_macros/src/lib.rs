use proc_macro::TokenStream;
use quote::quote;


#[proc_macro]
pub fn rote(_expr: TokenStream) -> TokenStream {
    let result = quote! {
        0
    };
    result.into()
}
