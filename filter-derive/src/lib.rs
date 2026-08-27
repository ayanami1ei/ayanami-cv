use proc_macro::TokenStream;
use quote::{quote};
use syn::{ItemStruct,  parse_macro_input};

#[proc_macro_attribute]
pub fn filter(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let name = input.ident; // 结构体名

    // 生成实现代码
    let expanded = quote! {
        #[derive(Debug, Clone, Copy)]
        pub struct #name;
        impl #name {
            pub fn filter<I: ImageViewLike<Gray>, IMut: ImageViewMutLike<Gray>, const SIZE: usize>(
                &self,
                src: &I,
                dst: &mut IMut,
            ) -> Result<(), Error> {
                neighborhood::<I, IMut, SIZE, #name>(src, dst, *self)
            }
        }
    };

    TokenStream::from(expanded)
}
