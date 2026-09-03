use pixel_derive::color_space;

use crate::image::pixel::Pixel;

pub trait ColorSpace {
    const CHANNEL: usize;
    type PixelType: Pixel;
}

/// RGB通道类型
#[color_space(r, g, b)]
pub struct Rgb;

/// RGBA通道类型（带透明度）
#[color_space(r, g, b, a)]
pub struct Rgba;

/// 灰度通道类型
#[color_space(gray)]
pub struct Gray;
