use pixel_derive::color_space;

/// RGB通道类型
#[color_space(r,g,b)]
pub struct Rgb;
/*pub struct Rgb;
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RgbPixel{
    pub r:u8,
    pub g:u8,
    pub b:u8
}
impl Pixel for RgbPixel{}

impl ColorSpace for Rgb {
    const CHANNEL: usize = 3;
    type PixelType = RgbPixel;
}*/
