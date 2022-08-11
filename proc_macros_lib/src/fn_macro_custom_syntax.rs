use proc_macro2::Ident;
use syn::{
    parse::Parse, punctuated::Punctuated, token::Comma, GenericArgument, Token, Type, WhereClause,
};

struct ManagerOfThingInfo {
    manager_name_ident: Ident,
    manager_ty: Type,
    manager_ty_generic_args: Option<Punctuated<GenericArgument, Comma>>,
    where_clause: Option<WhereClause>,
    thing_type: Type,
}

impl Parse for ManagerOfThingInfo {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let manager_ty: Type = input.parse()?;
        if input.peek(Token![<]) {
            input.parse::<Token![<]>()?;
        }
        let manager_generics_ident: Ident = input.parse()?;
        if input.peek(Token![>]) {
            input.parse::<Token![>]>()?;
        }
        input.parse::<Token![for]>()?;
        let thing_type: Type = input.parse()?;
        todo!()
    }
}
