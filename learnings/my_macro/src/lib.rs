use proc_macro::TokenStream ;
use syn::parse_macro_input;

#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    println!("{:#?}",input);
    "fn hello() { println!(\"Hello, world!\"); }".parse().unwrap()
}
 

#[proc_macro]
pub fn generate(input: TokenStream) -> TokenStream {
    println!("{:#?}",input);
   TokenStream::default()
}

#[proc_macro_derive(Builder)]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    let input  = parse_macro_input!(input as syn::DeriveInput);
    println!("{:#?}",input);
    TokenStream::default()
}