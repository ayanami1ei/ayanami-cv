use crate::{channel::Channel, pixel::Pixel};

/// 灰度通道类型
pub struct GrayChannel;
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GrayPixel(pub u8);
impl Pixel for GrayPixel{}

impl Channel for GrayChannel {
    const CHANNEL: usize = 1;
    type PixelType = GrayPixel;
}

/// RGB通道类型
pub struct RgbChannel;
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RgbPixel{
    pub r:u8,
    pub g:u8,
    pub b:u8
}
impl Pixel for RgbPixel{}

impl Channel for RgbChannel {
    const CHANNEL: usize = 3;
    type PixelType = RgbPixel;
}
