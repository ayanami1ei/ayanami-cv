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
        &self.data[index]
    }
}

impl<'a> Index<usize> for MatRowMut<'a> {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<'a> IndexMut<usize> for MatRowMut<'a> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
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
            let width=self.width();
            Some(MatRowMut {
                data: &mut self.data[start..end],
                width: width,
                row_index: row,
            })
        } else {
            None
        }
    }
    
    // 通过行和列访问像素值（类似二维数组语法）
    pub fn pixel(&self, x: usize, y: usize) -> Option<&u8> {
        if x < self.width() && y < self.height() {
            let index = y * self.width() + x;
            self.data.get(index)
        } else {
            None
        }
    }

    // 通过行和列访问像素值（可变引用）
    pub fn pixel_mut(&mut self, x: usize, y: usize) -> Option<&mut u8> {
        if x < self.width() && y < self.height() {
            let index = y * self.width() + x;
            self.data.get_mut(index)
        } else {
            None
        }
    }
}

// 为了支持类似二维数组的访问，我们需要实现一个更实用的方法
// 由于Rust的借用规则限制，我们不能直接实现Index和IndexMut
// 但我们可以提供一个更便捷的访问方式
impl<T, C: Channel> Mat<T, C> 
where 
    T: Copy + Into<usize> + std::ops::Mul<Output = T> + From<u8> + PartialEq,
{
    /// 获取指定坐标的像素值（通过行和列）
    pub fn get_pixel_by_coords(&self, x: usize, y: usize) -> Option<u8> {
        if x < self.width() && y < self.height() {
            let index = y * self.width() + x;
            Some(self.data[index])
        } else {
            None
        }
    }

    /// 设置指定坐标的像素值（通过行和列）
    pub fn set_pixel_by_coords(&mut self, x: usize, y: usize, value: u8) -> bool {
        if x < self.width() && y < self.height() {
            let index = y * self.width() + x;
            self.data[index] = value;
            true
        } else {
            false
        }
    }
    
    /// 转换为灰度图像（如果当前是彩色图像）
    pub fn to_grayscale(&self) -> Mat<usize, crate::color_space::GrayChannel> {
        let mut gray_mat = Mat::<usize, crate::color_space::GrayChannel>::new(self.width(), self.height());
        
        for y in 0..self.height() {
            for x in 0..self.width() {
                if let Some(pixel) = self.pixel(x, y) {
                    // 简单的灰度转换：取像素值的平均
                    let gray_value = *pixel;
                    gray_mat.set_pixel_by_coords(x, y, gray_value);
                }
            }
        }
        
        gray_mat
    }
}
