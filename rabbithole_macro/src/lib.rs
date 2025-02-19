use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Block, ItemFn, Local, Stmt};

#[proc_macro_attribute]
pub fn rabbithole(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let attrs = &input.attrs;
    let vis = &input.vis;
    let sig = &input.sig;
    let block = &input.block;

    let transformed_block = transform_block(block);

    let output = quote! {
        #(#attrs)*
        #vis #sig
        #transformed_block
    };

    output.into()
}

fn transform_local_stmt(local: &Local) -> proc_macro2::TokenStream {
    let attrs = &local.attrs;
    let let_token = &local.let_token;
    let pat = &local.pat;
    let semi = &local.semi_token;

    if let Some(init) = &local.init {
        let eq = &init.eq_token;
        let rhs_expr = &init.expr;

        quote! {
            #(#attrs)*
            #let_token #pat #eq dbg!(#rhs_expr) #semi
        }
    } else {
        quote! {
            #(#attrs)*
            #let_token #pat #semi
        }
    }
}

fn transform_block(block: &Block) -> proc_macro2::TokenStream {
    let mut stmts_iter = block.stmts.iter().peekable();
    let mut transformed_stmts = Vec::new();

    while let Some(stmt) = stmts_iter.next() {
        let transformed = match stmt {
            Stmt::Local(local) => transform_local_stmt(local),
            _ => {
                quote! { #stmt }
            }
        };

        transformed_stmts.push(transformed);
    }

    quote! {
        {
            #(#transformed_stmts)*
        }
    }
}
