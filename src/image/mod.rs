use crate::{
    color_space::ColorSpace,
    image::iter::{
        pixel_iter::ImagePixelIter, pixel_iter_mut::ImagePixelIterMut, row_iter::ImageRowIter,
        row_iter_mut::ImageRowIterMut,
    },
};

pub mod image;
pub mod image_row;
pub mod image_row_mut;
pub mod image_view;
pub mod image_view_mut;
pub mod iter;

pub trait ImageViewLike<C: ColorSpace> {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn pixel<'a>(&'a self)->&'a [C::PixelType];


    fn data(&self) -> &[u8];
    fn row_iter(&self) -> ImageRowIter<'_, C>;
    fn pixel_iter(&self) -> ImagePixelIter<'_, C>;
    fn at(&self, index: (usize, usize)) -> &C::PixelType;
}

pub trait ImageViewMutLike<C: ColorSpace>: ImageViewLike<C> {
    fn pixel_mut<'a>(&'a mut self)->* mut C::PixelType;

    fn data_mut(&mut self) -> &mut [u8];
    fn row_iter_mut(&mut self) -> ImageRowIterMut<'_, C>;
    fn pixel_iter_mut(&mut self) -> ImagePixelIterMut<'_, C>;
    fn at_mut(&mut self, index: (usize, usize)) -> &mut C::PixelType;
}
