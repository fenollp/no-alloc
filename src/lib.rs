use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, ItemFn};

#[proc_macro_attribute]
pub fn no_alloc(attr: TokenStream, item: TokenStream) -> TokenStream {
    assert!(attr.is_empty());

    // dbg!(item);
    // // panic!();

    // TokenStream {
    //     Ident {
    //         ident: "assert_no_alloc",
    //     },
    //     Group {
    //         delimiter: Parenthesis,
    //         stream: TokenStream [
    //             Punct {
    //                 ch: '|',
    //                 spacing: Joint,
    //             },
    //             Punct {
    //                 ch: '|',
    //                 spacing: Alone,
    //             },
    //             Group {
    //                 delimiter: Brace,
    //                 stream: item,
    //             },
    //         ],
    //     },
    // }

    let mut function = parse_macro_input!(item as ItemFn);
    let stmts = function.block.stmts;
    function.block = Box::new(parse_quote!({
        quote!(assert_no_alloc(|| {
            // let __result = (move || #ret {
            //     #move_self
            //     #(
            //         let #arg_pat = #arg_val;
            //     )*
                #(#stmts)*
            // })()
        }))
    }));

    TokenStream::from(quote!(#function))
}
