use std::ops::{Index, IndexMut};

use crate::pixel::Pixel;

// 可变行引用结构体
pub struct ImageRowMut<'a, P: Pixel> {
    data: &'a mut [P],
    width: usize,
}

impl<'a, P: Pixel> ImageRowMut<'a, P> {
    pub fn new(data: &'a mut [P], width: usize)->Self{
        Self { data, width }
    }

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