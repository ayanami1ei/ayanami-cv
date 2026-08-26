use crate::{Image, color_space::ColorSpace, image::image_row_mut::ImageRowMut};

pub struct ImageRowIterMut<'a, C: ColorSpace> {
    pixels: *mut C::PixelType,
    width: usize,
    height: usize,
    index: usize,
    _marker: std::marker::PhantomData<&'a mut C::PixelType>,
}

impl<'a, C: ColorSpace> ImageRowIterMut<'a, C> {
    pub fn new(image: &'a mut Image<C>) -> Self {
        let height = image.height();
        let width = image.width();
        let pixels = image.pixel_mut().as_mut_ptr();

        Self {
            pixels,
            width,
            height,
            index: 0,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<'a, C: ColorSpace> Iterator for ImageRowIterMut<'a, C> {
    type Item = ImageRowMut<'a, C::PixelType>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.height {
            return None;
        }

        let start = self.index * self.width;

        let row = unsafe {
            std::slice::from_raw_parts_mut(
                self.pixels.add(start),
                self.width,
            )
        };

        self.index += 1;

        Some(ImageRowMut::new(row, self.width))
    }
}