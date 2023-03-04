extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Arm, ItemFn};

#[proc_macro_attribute]
pub fn hook(args: TokenStream, input: TokenStream) -> TokenStream {
    let Arm { pat, body, .. } = parse_macro_input!(args as Arm);
    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = parse_macro_input!(input as ItemFn);

    quote! {
        #(#attrs)*
        #vis
        #sig
        {
            use error_hook::ResultExt;

            (move || -> Result<_, _> {
                #block
            })()
            .into_ghost(|#pat| {
                #body
            })
            .map_err(Into::into)
        }
    }
    .into()
}
