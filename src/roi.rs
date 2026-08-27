use crate::color_space::ColorSpace;
use crate::image::ImageViewLike;
use crate::image::iter::row_iter::ImageRowIter;
use crate::point::Point;

pub struct Roi<'a, C: ColorSpace> {
    left_up: Point,
    right_down: Point,
    data: Vec<&'a [C::PixelType]>,
}

impl<'a, C: ColorSpace> ImageViewLike<C> for Roi<'a, C> {
    fn width(&self) -> usize {
        self.right_down.x - self.left_up.x
    }
    fn height(&self) -> usize {
        self.right_down.y - self.left_up.y
    }
    fn pixel<'b>(&'b self) -> Vec<&'b [<C as ColorSpace>::PixelType]> {
        self.data.clone()
    }
    fn row_iter(&self) -> ImageRowIter<'_, C> {
        ImageRowIter::new(self)
    }
    fn at(&self, index: (usize, usize)) -> &C::PixelType {
        &self.data[index.0 * self.height()][index.1]
    }
}

impl<'a, C: ColorSpace> Roi<'a, C> {
    pub fn new_from_image<I: ImageViewLike<C>>(
        image: &'a I,
        left_up: Point,
        right_down: Point,
    ) -> Self {
        Self {
            left_up,
            right_down,
            data:image.pixel(),
        }
    }

    pub fn set_image<I: ImageViewLike<C>>(_image:&'a I){
        todo!()
    }
}
