use crate::Image;
use crate::color_space::ColorSpace;
use crate::image::{ImageViewLike, ImageViewMutLike};
use crate::image::iter::pixel_iter::ImagePixelIter;
use crate::image::iter::pixel_iter_mut::ImagePixelIterMut;
use crate::image::iter::row_iter::ImageRowIter;
use crate::image::iter::row_iter_mut::ImageRowIterMut;

pub struct ImageViewMut<'a, C: ColorSpace> {
    width: usize,
    height: usize,
    data: &'a mut [C::PixelType],
}

impl<'a, C: ColorSpace> ImageViewLike<C> for ImageViewMut<'a, C> {
    fn width(&self) -> usize {
        self.width
    }
    fn height(&self) -> usize {
        self.height
    }
    fn pixel<'b>(&'b self)->&'b [<C as ColorSpace>::PixelType] {
        self.data
    }
    fn data(&self) -> &[u8] {
        bytemuck::cast_slice(&self.data)
    }
    fn row_iter(&self) -> ImageRowIter<'_, C> {
        ImageRowIter::new(self)
    }
    fn pixel_iter(&self) -> ImagePixelIter<'_, C> {
        ImagePixelIter::new(&self.data, self.height * self.width)
    }
    fn at(&self, index: (usize, usize)) -> &C::PixelType {
        &self.data[index.0 * self.height + index.1]
    }
}

impl<'a, C: ColorSpace> ImageViewMutLike<C> for ImageViewMut<'a, C> {
    fn pixel_mut<'b>(&'b mut self)->* mut <C as ColorSpace>::PixelType {
        self.data.as_mut_ptr()
    }
    fn data_mut(&mut self) -> &mut [u8] {
        bytemuck::cast_slice_mut(&mut self.data)
    }
    fn row_iter_mut(&mut self) -> ImageRowIterMut<'_, C> {
        ImageRowIterMut::new(self)
    }
    fn pixel_iter_mut(&mut self) -> ImagePixelIterMut<'_, C> {
        ImagePixelIterMut::new(self.data.as_mut_ptr(), self.height * self.width)
    }
    fn at_mut(&mut self, index: (usize, usize)) -> &mut C::PixelType {
        &mut self.data[index.0 * self.height + index.1]
    }
}

impl<'a, C: ColorSpace> ImageViewMut<'a, C> {
    pub fn new(width: usize, height: usize, vec: &'a mut [u8]) -> Self {
        Self {
            width,
            height,
            data: bytemuck::cast_slice_mut(vec),
        }
    }

    pub fn new_from_image(image: &'a mut Image<C>) -> Self {
        Self {
            width: image.width(),
            height: image.height(),
            data: bytemuck::cast_slice_mut(image.data_mut()),
        }
    }
}
