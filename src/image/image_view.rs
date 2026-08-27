use crate::Image;
use crate::color_space::ColorSpace;
use crate::image::ImageViewLike;
use crate::image::iter::row_iter::ImageRowIter;

pub struct ImageView<'a, C: ColorSpace> {
    width: usize,
    height: usize,
    data: &'a [C::PixelType],
}

impl<'a, C: ColorSpace> ImageViewLike<C> for ImageView<'a, C> {
    fn width(&self) -> usize {
        self.width
    }
    fn height(&self) -> usize {
        self.height
    }
    fn pixel<'b>(&'b self) -> &'b [C::PixelType] {
        self.data
    }
    fn row_iter(&self) -> ImageRowIter<'_, C> {
        ImageRowIter::new(self)
    }
    fn at(&self, index: (usize, usize)) -> &C::PixelType {
        &self.data[index.0 * self.height + index.1]
    }
}

impl<'a, C: ColorSpace> ImageView<'a, C> {
    pub fn new(width: usize, height: usize, vec: &'a [u8]) -> Self {
        Self {
            width,
            height,
            data: bytemuck::cast_slice(vec),
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
