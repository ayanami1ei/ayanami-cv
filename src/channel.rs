use crate::pixel::Pixel;

pub trait Channel{
    const CHANNEL: usize;
    type PixelType:Pixel;
}