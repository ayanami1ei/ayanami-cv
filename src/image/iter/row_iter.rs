use crate::{
    color_space::ColorSpace,
    image::{ImageViewLike, image_row::ImageRow},
};

pub struct ImageRowIter<'a, C: ColorSpace> {
    pixels: &'a [C::PixelType],
    height: usize,
    width: usize,
    index: usize,
}

impl<'a, C: ColorSpace> ImageRowIter<'a, C> {
    pub fn new<I: ImageViewLike<C>>(image: &'a I) -> Self {
        Self {
            pixels: image.pixel(),
            height: image.height(),
            width: image.width(),
            index: 0,
        }
    }
}

impl<'a, C: ColorSpace> Iterator for ImageRowIter<'a, C> {
    type Item = ImageRow<'a, C>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.height {
            return None;
        }

        let start = self.index * self.width;
        let end = start + self.width;
        let res = &self.pixels[start..end];
        self.index += 1;
        Some(ImageRow::new(res, self.width))
    }
}
