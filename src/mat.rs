use std::marker::PhantomData;
use std::ops::{Index, IndexMut};

use crate::channel::Channel;

pub struct Mat<T, C: Channel> {
    weight: T,
    height: T,
    data: Vec<u8>,
    _phantom: PhantomData<C>,
}

// 行引用结构体
pub struct MatRow<'a> {
    data: &'a [u8],
    width: usize,
    row_index: usize,
}

// 可变行引用结构体
pub struct MatRowMut<'a> {
    data: &'a mut [u8],
    width: usize,
    row_index: usize,
}

impl<'a> Index<usize> for MatRow<'a> {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[self.row_index * self.width + index]
    }
}

impl<'a> IndexMut<usize> for MatRowMut<'a> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let offset = self.row_index * self.width + index;
        &mut self.data[offset]
    }
}

impl<T, C: Channel> Mat<T, C> 
where 
    T: Copy + Into<usize> + std::ops::Mul<Output = T> + From<u8> + PartialEq,
{
    pub fn new(weight: T, height: T) -> Self {
        let size = weight * height;
        let data = vec![0u8; size.into()];
        Mat {
            weight,
            height,
            data,
            _phantom: PhantomData,
        }
    }

    pub fn width(&self) -> usize {
        self.weight.into()
    }

    pub fn height(&self) -> usize {
        self.height.into()
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Option<&u8> {
        if x < self.width() && y < self.height() {
            let index = y * self.width() + x;
            self.data.get(index)
        } else {
            None
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, value: u8) -> bool {
        if x < self.width() && y < self.height() {
            let index = y * self.width() + x;
            self.data[index] = value;
            true
        } else {
            false
        }
    }

    pub fn fill(&mut self, value: u8) {
        self.data.fill(value);
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }
    
    // 获取行引用
    pub fn row(&self, row: usize) -> Option<MatRow<'_>> {
        if row < self.height() {
            let start = row * self.width();
            let end = start + self.width();
            Some(MatRow {
                data: &self.data[start..end],
                width: self.width(),
                row_index: row,
            })
        } else {
            None
        }
    }
    
    // 获取可变行引用
    pub fn row_mut(&mut self, row: usize) -> Option<MatRowMut<'_>> {
        if row < self.height() {
            let start = row * self.width();
            let end = start + self.width();
            Some(MatRowMut {
                data: &mut self.data[start..end],
                width: self.width(),
                row_index: row,
            })
        } else {
            None
        }
    }
}

impl<T, C: Channel> Index<usize> for Mat<T, C>
where 
    T: Copy + Into<usize> + std::ops::Mul<Output = T> + From<u8> + PartialEq,
{
    type Output = MatRow<'_>;

    fn index(&self, index: usize) -> &Self::Output {
        // 这里需要返回一个临时值，实际实现会更复杂
        panic!("Index trait not fully implemented for Mat");
    }
}

impl<T, C: Channel> IndexMut<usize> for Mat<T, C>
where 
    T: Copy + Into<usize> + std::ops::Mul<Output = T> + From<u8> + PartialEq,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        // 这里需要返回一个临时值，实际实现会更复杂
        panic!("IndexMut trait not fully implemented for Mat");
    }
}
