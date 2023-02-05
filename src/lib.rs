use proc_macro::TokenStream;

use quote::{quote, TokenStreamExt};
use syn::{parse_macro_input, ExprClosure, ItemFn};

#[proc_macro_attribute]
pub fn precond(args: TokenStream, input: TokenStream) -> TokenStream {
    // We expect the precondition to be provided via Rust closure syntax.
    let precond_fn = parse_macro_input!(args as ExprClosure);
    // We expect the target to always be a Rust function.
    let wrapped_fn = parse_macro_input!(input as ItemFn);

    // Parse the arguments of the wrapped function.
    let mut arg_symbols = Vec::new();
    let wrapped_fn_sig = wrapped_fn.sig;
    for fn_arg in wrapped_fn_sig.inputs.iter() {
        if let syn::FnArg::Typed(t) = fn_arg {
            if let syn::Pat::Ident(p) = &*t.pat {
                arg_symbols.push(p.ident.clone());
            }
        }
    }

    // Check and assert argument and return type symmetries/invariants.
    let precond_expr = precond_fn.body;
    let alen = arg_symbols.len();
    assert!(alen > 0);
    assert_eq!(alen, precond_fn.inputs.len());
    // If the precondition is a simple tuple expression, check arity match.
    // Other valid expressions (e.g., a function pointer) are not checked.
    if let syn::Expr::Tuple(t) = &*precond_expr {
        assert_eq!(alen, t.elems.len());
    }

    // Build the precondition expression to be inlined at the beginning of the
    // wrapped function.
    let mut body = proc_macro2::TokenStream::new();
    body.append_all(&wrapped_fn.block.stmts);
    let mut let_binding_str = String::from("let ");
    if alen > 1 {
        let_binding_str.push('(');
        for arg in &arg_symbols[..alen - 1] {
            let_binding_str.push_str(&format!("{arg}, "));
        }
        let_binding_str.push_str(&format!("{})", arg_symbols[alen - 1]));
    } else {
        let_binding_str.push_str(&format!("{}", arg_symbols[0]));
    }
    let let_binding: proc_macro2::TokenStream = let_binding_str.parse().unwrap();

    // Emit the wrapped function with inlined precondition code.
    quote! {
        #wrapped_fn_sig {
            #let_binding = #precond_expr;
            #body
        }
    }
    .into()
}
