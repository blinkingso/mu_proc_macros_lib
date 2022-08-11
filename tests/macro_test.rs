use proc_macros_lib::{fn_foo, fn_macro_ast_viz_debug};

#[test]
fn test_proc_macro() {
    fn_macro_ast_viz_debug!();
    assert_eq!(foo(), 42);
}

#[test]
fn fn_foo() {
    assert_eq!(fn_foo!(2 + 3), 1);
}
