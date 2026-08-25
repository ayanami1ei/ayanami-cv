use std::ops::{Index, IndexMut};

use crate::color_space::ColorSpace;
use crate::pixel::Pixel;

pub struct Image<C: ColorSpace> {
    weight: usize,
    height: usize,
    data: Vec<C::PixelType>,
}

// 行引用结构体
pub struct ImageRow<'a, P: Pixel> {
    data: &'a [P],
    width: usize,
}

impl<'a, P: Pixel> ImageRow<'a, P> {
    pub fn len(&self) -> usize {
        self.width
    }
}

impl<'a, P: Pixel> Index<usize> for ImageRow<'a, P> {
    type Output = P;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.width {
            panic!("range out of bound")
        }
        &self.data[index]
    }
}

// 可变行引用结构体
pub struct ImageRowMut<'a, P: Pixel> {
    data: &'a mut [P],
    width: usize,
}

impl<'a, P: Pixel> ImageRowMut<'a, P> {
    pub fn len(&self) -> usize {
        self.width
    }
}

impl<'a, P: Pixel> Index<usize> for ImageRowMut<'a, P> {
    type Output = P;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.width {
            panic!("range out of bound")
        }
        &self.data[index]
    }
}

impl<'a, P: Pixel> IndexMut<usize> for ImageRowMut<'a, P> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.width {
            panic!("range out of bound")
        }
        &mut self.data[index]
    }
}

impl<C: ColorSpace> Image<C> {
    pub fn new(weight: usize, height: usize) -> Self {
        let size = weight * height;
        let data = vec![C::PixelType::default(); size.into()];
        Image {
            weight,
            height,
            data,
        }
    }

    pub fn width(&self) -> usize {
        self.weight.into()
    }

    pub fn height(&self) -> usize {
        self.height.into()
    }

    pub fn data(&self) -> &[u8] {
        bytemuck::cast_slice(&self.data)
    }

    pub fn data_mut(&mut self) -> &mut [C::PixelType] {
        bytemuck::cast_slice_mut(&mut self.data)
    }

    // 获取行引用
    pub fn row(&self, row: usize) -> ImageRow<'_, C::PixelType> {
        if row < self.height() {
            let start = row * self.width();
            let end = start + self.width();
            ImageRow {
                data: &self.data[start..end],
                width: self.width(),
            }
        } else {
            panic!("range out of bound")
        }
    }

    // 获取可变行引用
    pub fn row_mut(&mut self, row: usize) -> ImageRowMut<'_, C::PixelType> {
        if row < self.height() {
            let start = row * self.width();
            let end = start + self.width();
            let width = self.width();
            ImageRowMut {
                data: &mut self.data[start..end],
                width: width,
            }
        } else {
            panic!("range out of bound")
        }
    }
}
