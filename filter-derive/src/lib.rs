use proc_macro::TokenStream;
use quote::{quote};
use syn::{ItemStruct,  parse_macro_input};

#[proc_macro_attribute]
pub fn filter(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let name = input.ident.clone(); // 结构体名

    // 生成实现代码
    let expanded = quote! {
        #input
        impl #name {
            pub fn filter<C:ColorSpace, I: ImageViewLike<C>, IMut: ImageViewMutLike<C>, const SIZE: usize>(
                &mut self,
                src: &I,
                dst: &mut IMut,
            ) -> Result<(), Error> {
                neighborhood::<C, I, IMut, SIZE, #name>(src, dst, self)
            }
        }
    };

    TokenStream::from(expanded)
}
