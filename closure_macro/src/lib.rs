// src/lib.rs
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
///capture_closure: The macro captures the function definition and prints it as a string. It uses the quote! macro to convert the input ItemFn back into tokens and then into a string.
pub fn capture_closure(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(item as ItemFn);
    
    // Convert the function back to tokens
    let function_tokens = quote! { #input };

    // Convert the tokens to a string
    let function_str = function_tokens.to_string();

    // Generate the new function with logging
    let output = quote! {
        #input

        // Print the function definition
        println!("Captured closure: {}", #function_str);
    };

    // Return the generated tokens
    TokenStream::from(output)
}
