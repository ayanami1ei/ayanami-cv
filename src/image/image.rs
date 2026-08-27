use crate::color_space::ColorSpace;
use crate::image::iter::pixel_iter_mut::ImagePixelIterMut;
use crate::image::iter::row_iter::ImageRowIter;
use crate::image::iter::row_iter_mut::ImageRowIterMut;
use crate::image::{ImageViewLike, ImageViewMutLike};

pub struct Image<C: ColorSpace> {
    width: usize,
    height: usize,
    data: Vec<C::PixelType>,
}

impl<C: ColorSpace> ImageViewLike<C> for Image<C> {
    fn width(&self) -> usize {
        self.width
    }
    fn height(&self) -> usize {
        self.height
    }
    fn pixel<'a>(&'a self) -> &'a [<C as ColorSpace>::PixelType] {
        &self.data
    }
    fn row_iter(&self) -> ImageRowIter<'_, C> {
        ImageRowIter::new(self)
    }
    fn at(&self, index: (usize, usize)) -> &C::PixelType {
        &self.data[index.0 * self.height + index.1]
    }
}

impl<C: ColorSpace> ImageViewMutLike<C> for Image<C> {
    fn pixel_mut<'a>(&'a mut self) -> *mut <C as ColorSpace>::PixelType {
        self.data.as_mut_ptr()
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

impl<C: ColorSpace> Image<C> {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        let data = vec![C::PixelType::default(); size.into()];
        Image {
            width,
            height,
            data,
        }
    }

    pub fn new_from_vec(width: usize, height: usize, vec: Vec<u8>) -> Self {
        Self {
            width,
            height,
            data: bytemuck::allocation::cast_vec(vec),
        }
    }
}
