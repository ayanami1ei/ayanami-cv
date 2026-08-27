use crate::Image;
use crate::color_space::ColorSpace;
use crate::image::ImageViewLike;
use crate::image::iter::row_iter::ImageRowIter;

pub struct ImageView<'a, C: ColorSpace> {
    width: usize,
    height: usize,
    data: Vec<&'a [C::PixelType]>,
}

impl<'a, C: ColorSpace> ImageViewLike<C> for ImageView<'a, C> {
    fn width(&self) -> usize {
        self.width
    }
    fn height(&self) -> usize {
        self.height
    }
    fn pixel<'b>(&'b self) -> Vec<&'b [<C as ColorSpace>::PixelType]> {
        self.data.clone()
    }
    fn row_iter(&self) -> ImageRowIter<'_, C> {
        ImageRowIter::new(self)
    }
    fn at(&self, index: (usize, usize)) -> &C::PixelType {
        &self.data[index.0][index.1]
    }
}

impl<'a, C: ColorSpace> ImageView<'a, C> {
    pub fn new(width: usize, height: usize, vec: &'a [u8]) -> Self {
        let mut data=Vec::with_capacity(height);
        for i in 0..height{
            data.push(bytemuck::cast_slice(&vec[i*height..i*height+width]))
        }
        Self {
            width,
            height,
            data,
        }
    }

    pub fn new_from_image(image: &'a Image<C>) -> Self {
        Self {
            width: image.width(),
            height: image.height(),
            data: image.pixel(),
        }
    }
}
