use proc_macro::TokenStream;
use quote::ToTokens;
use r3bl_rs_utils::{style_primary, style_prompt};
use syn::{parse_str, ItemFn};

pub fn fn_proc_macro_impl(_input: TokenStream) -> TokenStream {
    let output_toiken_stream = "fn foo() -> u32 {42}";
    let output = output_toiken_stream.parse().unwrap();

    let ast_item_fn: ItemFn = parse_str(output_toiken_stream).unwrap();
    viz_ast(ast_item_fn);

    output
}

fn viz_ast(ast: ItemFn) {
    let ast_clone = ast.clone();
    eprintln!("{} => {:?}", style_primary("Debug::ast"), ast_clone);

    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = ast;

    eprintln!(
        "{} ast_item_fn < attrs.len:{}, vis:{}, sig: '{}' stmt: '{}' >",
        style_primary("=>"),
        style_prompt(&attrs.len().to_string()),
        style_prompt(match vis {
            syn::Visibility::Public(_) => "public",
            syn::Visibility::Crate(_) => "crate",
            syn::Visibility::Restricted(_) => "restricted",
            syn::Visibility::Inherited => "inherited",
        }),
        style_prompt(&sig.ident.to_string()),
        style_prompt(&match block.stmts.first() {
            Some(stmt) => {
                let expr_str = stmt.to_token_stream().to_string().clone();
                expr_str
            }
            None => "empty".to_string(),
        }),
    );
}
