use std::marker::PhantomData;
use std::ops::{Index, IndexMut};

use crate::channel::Channel;
use crate::pixel::Pixel;

pub struct Image<T, P:Pixel, C: Channel<P>> {
    weight: T,
    height: T,
    data: Vec<P>,
    _phantom: PhantomData<C>,
}

// 行引用结构体
pub struct ImageRow<'a, P:Pixel> {
    data: &'a [P],
    width: usize,
}

impl<'a, P:Pixel> Index<usize> for ImageRow<'a, P> {
    type Output = P;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.width {
            panic!("range out of bound")
        }
        &self.data[index]
    }
}

// 可变行引用结构体
pub struct ImageRowMut<'a, P:Pixel> {
    data: &'a mut [P],
    width: usize,
}

impl<'a, P:Pixel> Index<usize> for ImageRowMut<'a, P> {
    type Output = P;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.width {
            panic!("range out of bound")
        }
        &self.data[index]
    }
}

impl<'a, P:Pixel> IndexMut<usize> for ImageRowMut<'a, P> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.width {
            panic!("range out of bound")
        }
        &mut self.data[index]
    }
}

impl<T, P:Pixel, C: Channel<P>> Image<T, P, C>
where
    T: Copy + Into<usize> + std::ops::Mul<Output = T> + From<u8> + PartialEq,
{
    pub fn new(weight: T, height: T) -> Self {
        let size = weight * height;
        let data = vec![P::default(); size.into()];
        Image {
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

    pub fn data(&self) -> &[P] {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut [P] {
        &mut self.data
    }
}

impl<'a, T, P:Pixel, C: Channel<P>> Index<u8> for Image<T, P, C> where
    T: Copy + Into<usize> + std::ops::Mul<Output = T> + From<u8> + PartialEq
{
    type Output=ImageRow<'a, P>;

    fn index(&self, index: u8) -> &Self::Output {
        return 
    }
}
