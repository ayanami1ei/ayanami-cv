use crate::color_space::ColorSpace;
use crate::image::iter::pixel_iter_mut::ImagePixelIterMut;
use crate::image::iter::row_iter_mut::ImageRowIterMut;
use crate::image::{ImageViewLike, ImageViewMutLike};
use crate::image::iter::row_iter::ImageRowIter;
use crate::point::Point;

pub struct RoiMut<'a, C: ColorSpace> {
    left_up: Point,
    right_down: Point,
    data: &'a mut [C::PixelType],
}

impl<'a, C: ColorSpace> ImageViewLike<C> for RoiMut<'a, C> {
    fn width(&self) -> usize {
        self.right_down.x - self.left_up.x
    }
    fn height(&self) -> usize {
        self.right_down.y - self.left_up.y
    }
    fn pixel<'b>(&'b self) -> Vec<&'b [<C as ColorSpace>::PixelType]> {
        let mut res=Vec::with_capacity(self.height());
        for i in 0..self.height(){
            res.push(&self.data[i*self.height()..i*self.height()+self.width()])
        }

        res
    }
    fn row_iter(&self) -> ImageRowIter<'_, C> {
        ImageRowIter::new(self)
    }
    fn at(&self, index: (usize, usize)) -> &C::PixelType {
        &self.data[index.0 * self.height() + index.1]
    }
}

impl<'a, C: ColorSpace> ImageViewMutLike<C> for RoiMut<'a, C> {
    fn pixel_mut<'b>(&'b mut self) -> *mut <C as ColorSpace>::PixelType {
        self.data.as_mut_ptr()
    }
    fn row_iter_mut(&mut self) -> ImageRowIterMut<'_, C> {
        ImageRowIterMut::new(self)
    }
    fn pixel_iter_mut(&mut self) -> ImagePixelIterMut<'_, C> {
        ImagePixelIterMut::new(self.data.as_mut_ptr(), self.height() * self.width())
    }
    fn at_mut(&mut self, index: (usize, usize)) -> &mut C::PixelType {
        &mut self.data[index.0 * self.height() + index.1]
    }
}

impl<'a, C: ColorSpace> RoiMut<'a, C> {
    pub fn new_from_image<I: ImageViewMutLike<C>>(
        image: &'a mut I,
        left_up: Point,
        right_down: Point,
    ) -> Self {
        let data = unsafe {
            std::slice::from_raw_parts_mut(image.pixel_mut(), image.width() * image.height())
        };
        Self {
            left_up,
            right_down,
            data,
        }
    }

    pub fn set_image<I: ImageViewLike<C>>(_image:&'a I){
        todo!()
    }
}
