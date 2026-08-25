use crate::{channel::Channel, pixel::Pixel};

/// 灰度通道类型
pub struct GrayChannel;
#[derive(Debug, Clone, Copy, Default)]
pub struct GrayPixel(pub u8);
impl Pixel for GrayPixel{
    fn to_u8(&self)->Vec<u8> {
        let mut vec=Vec::new();
        vec.push(self.0);
        vec
    }

    fn to_u8_with(&self, vec:&mut Vec<u8>) {
        vec.push(self.0);
    }
}

impl Channel for GrayChannel {
    const CHANNEL: usize = 1;
    type PixelType = GrayPixel;
}

/// RGB通道类型
pub struct RgbChannel;
#[derive(Debug, Clone, Copy, Default)]
pub struct RgbPixel{
    pub r:u8,
    pub g:u8,
    pub b:u8
}
impl Pixel for RgbPixel{
    fn to_u8(&self)->Vec<u8> {
        let mut vec=Vec::new();
        vec.push(self.r);
        vec.push(self.g);
        vec.push(self.b);
        vec
    }

    fn to_u8_with(&self, vec:&mut Vec<u8>) {
        vec.push(self.r);
        vec.push(self.g);
        vec.push(self.b);
    }
}

impl Channel for RgbChannel {
    const CHANNEL: usize = 3;
    type PixelType = RgbPixel;
}
