use crate::{color_space::ColorSpace};

pub struct ImagePixelIter<'a, C: ColorSpace> {
    pixels: &'a [C::PixelType],
    len: usize,
    index: usize,
}

impl<'a, C: ColorSpace> ImagePixelIter<'a, C> {
    pub fn new(pixels: &'a [C::PixelType], len:usize) -> Self {
        Self {
            pixels,
            len,
            index: 0,
        }
    }
}

impl<'a, C: ColorSpace> Iterator for ImagePixelIter<'a, C> {
    type Item = &'a C::PixelType;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.len {
            return None;
        }
        let res = Some(&self.pixels[self.index]);
        self.index += 1;
        res
    }
}
