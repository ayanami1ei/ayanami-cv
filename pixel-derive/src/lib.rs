use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Ident, ItemStruct, Token, parse::Parser, parse_macro_input, punctuated::Punctuated};

#[proc_macro_attribute]
pub fn color_space(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let name = input.ident; // 结构体名
    let pixel_name = format_ident!("{name}Pixel");

    //let attr_str = parse_macro_input!(attr as Ident).to_string();
    //let re = Regex::new(r"[\s*,\s*]").unwrap();
    let pixel_names: Vec<Ident> = Punctuated::<Ident, Token![,]>::parse_terminated
        .parse(attr)
        .unwrap()
        .into_iter()
        .collect();
    let channel = pixel_names.len();

    // 生成实现代码
    let expanded = quote! {
        pub struct #name;
        impl crate::color_space::ColorSpace for #name {
            const CHANNEL: usize = #channel;
            type PixelType = #pixel_name;
        }

        #[repr(C)]
        #[derive(Debug, Clone, Copy, Default, bytemuck::Pod, bytemuck::Zeroable)]
        pub struct #pixel_name{
            #(
                #pixel_names: u8,
            )*
        }

        impl crate::color_space::Pixel for #pixel_name{}
    };

    TokenStream::from(expanded)
}
