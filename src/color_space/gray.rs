use crate::{color_space::ColorSpace, pixel::Pixel};

/// 灰度通道类型
pub struct Gray;
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GrayPixel(pub u8);
impl Pixel for GrayPixel{}

impl ColorSpace for Gray {
    const CHANNEL: usize = 1;
    type PixelType = GrayPixel;
}