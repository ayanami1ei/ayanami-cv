use crate::pixel::Pixel;
use pixel_derive::color_space;

pub trait ColorSpace {
    const CHANNEL: usize;
    type PixelType: Pixel;
}

/// RGB通道类型
#[color_space(r, g, b)]
pub struct Rgb;

/// 灰度通道类型
#[color_space(gray)]
pub struct Gray;
