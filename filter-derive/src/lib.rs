use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Ident, ItemStruct, Token, parse::Parser, parse_macro_input, punctuated::Punctuated};

#[proc_macro_attribute]
pub fn filter(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let name = input.ident; // 结构体名
    let pixel_name = format_ident!("{name}Pixel");

    // 生成实现代码
    let expanded = quote! {
        pub struct #name;
        impl #name {
            pub fn filter(){
                
            }
        }
    };

    TokenStream::from(expanded)
}
