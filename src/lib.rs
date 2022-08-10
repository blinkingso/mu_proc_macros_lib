extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn fn_macro_ast_viz_debug(input: TokenStream) -> TokenStream {
    ast_viz_debug::fn_proc_macro_impl(input)
}

#[proc_macro_derive(Dubbo)]
pub fn my_derive_proc_macro(input: TokenStream) -> TokenStream {
    // 1. Use `syn` to parse the input tokens into a syntax tree.
    // 2. Generate new tokens based ont the syntax tree. This is additive to the `enum` or `struct`
    //    that is annotated (it doesn't replace them).
    // 3. Return the generated tokens
    input
}

#[proc_macro_attribute]
pub fn log_entry_and_exit(_args: TokenStream, input: TokenStream) -> TokenStream {
    // 1. Use `syn` to parse the args & input tokens into a syntax tree.
    // 2. Generate new tokens based on the syntax tree. This will replace whatever `item` is
    // annotated w/ this attribute proc macro.
    // 3 Reutrn the generated tokens.
    input
}

mod ast_viz_debug;
