//!
//! This library supplements `hook` _macro attribute_ to hook error conversion
//! (please use the re-exported macro from `error_hook` via feature `attribute`).
//!

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Arm, ItemFn};

/// Hooks `From` trait and executes specified action.
#[proc_macro_attribute]
pub fn hook(args: TokenStream, input: TokenStream) -> TokenStream {
    let Arm { pat, body, .. } = parse_macro_input!(args as Arm);
    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = parse_macro_input!(input as ItemFn);

    if sig.asyncness.is_some() {
        quote! {
            #(#attrs)*
            #vis
            #sig
            {
                use error_hook::SecretTraitDoNotUseOrYouWillBeFired;

                (move || async move #block)()
                .await
                .into_ghost(|#pat| #body)
                .map_err(Into::into)
            }
        }
        .into()
    } else {
        quote! {
            #(#attrs)*
            #vis
            #sig
            {
                use error_hook::SecretTraitDoNotUseOrYouWillBeFired;

                (move || -> Result<_, _> #block)()
                .into_ghost(|#pat| #body)
                .map_err(Into::into)
            }
        }
        .into()
    }
}
