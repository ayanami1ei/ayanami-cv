use std::marker::PhantomData;
use std::ops::{Index, IndexMut};

use crate::channel::Channel;

pub struct MatGray<T> {
    weight: T,
    height: T,
    data: Vec<u8>,
    _phantom: PhantomData<GrayChannel>,
}

// 灰度通道类型
pub struct GrayChannel;

impl Channel for GrayChannel {
    const CHANNEL: usize = 1;
}

// 行引用结构体
pub struct MatGrayRow<'a> {
    data: &'a [u8],
    width: usize,
    row_index: usize,
}

// 可变行引用结构体
pub struct MatGrayRowMut<'a> {
    data: &'a mut [u8],
    width: usize,
    row_index: usize,
}

impl<'a> Index<usize> for MatGrayRow<'a> {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<'a> Index<usize> for MatGrayRowMut<'a> {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<'a> IndexMut<usize> for MatGrayRowMut<'a> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T> MatGray<T> 
where 
    T: Copy + Into<usize> + std::ops::Mul<Output = T> + From<u8> + PartialEq,
{
    pub fn new(weight: T, height: T) -> Self {
        let size = weight * height;
        let data = vec![0u8; size.into()];
        MatGray {
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
    pub fn row(&self, row: usize) -> Option<MatGrayRow<'_>> {
        if row < self.height() {
            let start = row * self.width();
            let end = start + self.width();
            Some(MatGrayRow {
                data: &self.data[start..end],
                width: self.width(),
                row_index: row,
            })
        } else {
            None
        }
    }
    
    // 获取可变行引用
    pub fn row_mut(&mut self, row: usize) -> Option<MatGrayRowMut<'_>> {
        if row < self.height() {
            let start = row * self.width();
            let end = start + self.width();
            let width=self.width();
            Some(MatGrayRowMut {
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

impl<T> MatGray<T> 
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
}
