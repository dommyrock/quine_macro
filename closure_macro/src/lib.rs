use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
///capture_closure: Macro captures the function definition and prints it as a string. Uses quote! macro to convert the input 'ItemFn' back into tokens and then into a string.
pub fn capture_closure(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input tokens into AST
    let input = parse_macro_input!(item as ItemFn);
    
    // Convert the function back to tokens
    let function_tokens = quote! { #input };

    let function_str = function_tokens.to_string();
    let output = quote! {
        #input
        // Print the function definition
        println!("Captured closure: {}", #function_str);
    };
    TokenStream::from(output)
}
