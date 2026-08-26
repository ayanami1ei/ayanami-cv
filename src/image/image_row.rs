use std::ops::Index;

use crate::pixel::Pixel;

// 行引用结构体
pub struct ImageRow<'a, P: Pixel> {
    data: &'a [P],
    width: usize,
}

impl<'a, P: Pixel> ImageRow<'a, P> {
    pub fn new(data: &'a [P], width: usize)->Self{
        Self { data, width }
    }

    pub fn len(&self) -> usize {
        self.width
    }

    pub fn data(&self)->&'a [u8]{
        bytemuck::cast_slice(self.data)
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