pub mod gray;
pub mod rgb;

use crate::{pixel::Pixel};

pub trait ColorSpace {
    const CHANNEL: usize;
    type PixelType: Pixel;
}





