use crate::{channel::Channel, pixel::Pixel};

/// 灰度通道类型
pub struct GrayChannel;
#[derive(Debug, Clone, Copy, Default)]
pub struct GrayPixel(pub u8);
impl Pixel for GrayPixel{}

impl Channel<GrayPixel> for GrayChannel {
    const CHANNEL: usize = 1;
}

/// RGB通道类型
pub struct RgbChannel;
#[derive(Debug, Clone, Copy, Default)]
pub struct RgbPixel{
    pub r:u8,
    pub g:u8,
    pub b:u8
}
impl Pixel for RgbPixel{}

impl Channel<RgbPixel> for RgbChannel {
    const CHANNEL: usize = 3;
}
