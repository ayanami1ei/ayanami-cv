use crate::image::{color_space::ColorSpace, iter::{
        pixel_iter::ImagePixelIter, pixel_iter_mut::ImagePixelIterMut, row_iter::ImageRowIter,
        row_iter_mut::ImageRowIterMut,
    }};

pub mod image;
pub mod iter;
pub mod view;
pub mod pixel;
pub mod color_space;

pub trait ImageViewLike<C: ColorSpace> {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn pixel<'a>(&'a self) -> Vec<&'a [C::PixelType]>;

    fn row_iter(&self) -> ImageRowIter<'_, C>;
    fn pixel_iter<'a>(&'a self) -> impl Iterator<Item = &'a C::PixelType>
    where
        C: 'a,
    {
        self.row_iter().flat_map(|row| row)
    }
    fn at(&self, index: (usize, usize)) -> &C::PixelType;
}

pub trait ImageViewMutLike<C: ColorSpace>: ImageViewLike<C> {
    fn pixel_mut<'a>(&'a mut self) -> *mut C::PixelType;

    fn row_iter_mut(&mut self) -> ImageRowIterMut<'_, C>;
    fn pixel_iter_mut(&mut self) -> ImagePixelIterMut<'_, C>;
    fn at_mut(&mut self, index: (usize, usize)) -> &mut C::PixelType;
}
