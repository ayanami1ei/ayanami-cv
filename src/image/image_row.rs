use crate::color_space::ColorSpace;
use crate::image::ImagePixelIter;
use std::ops::Index;

// 行引用结构体
pub struct ImageRow<'a, C: ColorSpace> {
    data: &'a [C::PixelType],
    width: usize,
}

impl<'a, C: ColorSpace + 'a> IntoIterator for ImageRow<'a, C> {
    type Item = &'a C::PixelType;
    type IntoIter = std::slice::Iter<'a, C::PixelType>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<'a, C: ColorSpace> ImageRow<'a, C> {
    pub fn new(data: &'a [C::PixelType], width: usize) -> Self {
        Self { data, width }
    }

    pub fn len(&self) -> usize {
        self.width
    }

    pub fn data(&self) -> &'a [u8] {
        bytemuck::cast_slice(self.data)
    }

    pub fn pixel_iter(&self) -> ImagePixelIter<'_, C> {
        ImagePixelIter::new(&self.data, self.width)
    }
}

impl<'a, C: ColorSpace> Index<usize> for ImageRow<'a, C> {
    type Output = C::PixelType;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.width {
            panic!("range out of bound")
        }
        &self.data[index]
    }
}
