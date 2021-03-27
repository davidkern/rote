use proc_macro::TokenStream;


#[proc_macro]
pub fn rote(expr: TokenStream) -> TokenStream {
    expr
}
