use crate::{color_space::ColorSpace, pixel::Pixel};

/// RGB通道类型
pub struct Rgb;
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
}