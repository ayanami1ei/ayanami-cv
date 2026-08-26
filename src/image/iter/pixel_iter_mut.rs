use crate::color_space::ColorSpace;

pub struct ImagePixelIterMut<'a, C: ColorSpace> {
    pixels: *mut C::PixelType,
    len: usize,
    index: usize,
    _marker: std::marker::PhantomData<&'a mut C::PixelType>,
}

impl<'a, C: ColorSpace> ImagePixelIterMut<'a, C> {
    pub fn new(pixels: *mut C::PixelType, len: usize) -> Self {
        Self {
            pixels,
            len,
            index: 0,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<'a, C: ColorSpace> Iterator for ImagePixelIterMut<'a, C> {
    type Item = &'a mut C::PixelType;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.len {
            return None;
        }

        let ptr = unsafe { self.pixels.add(self.index) };

        self.index += 1;

        Some(unsafe { &mut *ptr })
    }
}
