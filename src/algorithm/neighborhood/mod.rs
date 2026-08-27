use std::marker::PhantomData;

use crate::{
    Gray, GrayPixel, ImageViewLike, ImageViewMutLike, algorithm::neighborhood::error::Error,
    image::color_space::ColorSpace,
};

pub mod error;

pub trait WindowLike<C: ColorSpace> {
    fn set_index(&mut self, index: usize);
    fn at(&self, x: i32, y: i32) -> C::PixelType;
}

pub struct InteriorWindow<'a, C: ColorSpace, I: ImageViewLike<C>, const SIZE: usize> {
    data: &'a I,
    index: usize,
    _mark: PhantomData<C>,
}

impl<'a, C: ColorSpace, I: ImageViewLike<C>, const SIZE: usize> InteriorWindow<'a, C, I, SIZE> {
    fn new(image: &'a I) -> Self {
        Self {
            data: image,
            index: 0,
            _mark: PhantomData,
        }
    }
}

impl<'a, C: ColorSpace, I: ImageViewLike<C>, const SIZE: usize> WindowLike<C>
    for InteriorWindow<'a, C, I, SIZE>
{
    fn set_index(&mut self, index: usize) {
        self.index = index;
    }

    fn at(&self, x: i32, y: i32) -> C::PixelType {
        self.data.pixel()[self.index + x as usize][self.index + y as usize]
    }
}

pub struct BorderWindow<'a, C: ColorSpace, I: ImageViewLike<C>, const SIZE: usize> {
    data: &'a I,
    index: usize,
    _mark: PhantomData<C>,
}

impl<'a, C: ColorSpace, I: ImageViewLike<C>, const SIZE: usize> BorderWindow<'a, C, I, SIZE> {
    fn new(image: &'a I) -> Self {
        Self {
            data: image,
            index: 0,
            _mark: PhantomData,
        }
    }
}

impl<'a, C: ColorSpace, I: ImageViewLike<C>, const SIZE: usize> WindowLike<C>
    for BorderWindow<'a, C, I, SIZE>
{
    fn set_index(&mut self, index: usize) {
        self.index = index;
    }

    fn at(&self, x: i32, y: i32) -> C::PixelType {
        if x < 0 || x as usize >= self.data.width() || y < 0 || y as usize >= self.data.height() {
            return C::PixelType::default();
        }
        self.data.pixel()[self.index + x as usize][self.index + y as usize]
    }
}

pub trait NeighborhoodAlgorithm{
    fn process<W: WindowLike<Gray>>(&self, window: &W)->GrayPixel;
}

pub fn neighborhood<
    'a,
    I: ImageViewLike<Gray> + 'a,
    IMut: ImageViewMutLike<Gray>,
    const SIZE: usize,
    A: NeighborhoodAlgorithm,
>(
    src: &'a I,
    dst: &mut IMut,
    algor: A,
) -> Result<(), Error> {
    if SIZE % 2 == 0 {
        return Err(Error::WindowSizeMustBeOdd);
    }

    let r: usize = SIZE / 2;

    let width = src.width();
    let height = src.height();

    // 内部
    let mut inner_win = InteriorWindow::<Gray, I, SIZE>::new(src);

    for i in r..height - r {
        for j in r..width - r {
            inner_win.set_index(i * width + j);

            let p = dst.at_mut((i, j));
            *p = algor.process(&inner_win);
        }
    }

    // 边界
    let mut border_win = BorderWindow::<Gray, I, SIZE>::new(src);

    // top
    for i in 0..r {
        for j in 0..width {
            border_win.set_index(i * width + j);

            let p = dst.at_mut((i, j));
            *p = algor.process(&border_win);
        }
    }

    // bottom
    for i in height - r..height {
        for j in 0..width {
            border_win.set_index(i * width + j);

            let p = dst.at_mut((i, j));
            *p = algor.process(&border_win);
        }
    }

    // left
    for i in r..height - r {
        for j in 0..r {
            border_win.set_index(i * width + j);

            let p = dst.at_mut((i, j));
            *p = algor.process(&border_win);
        }
    }

    // right
    for i in r..height - r {
        for j in width - r..width {
            border_win.set_index(i * width + j);

            let p = dst.at_mut((i, j));
            *p = algor.process(&border_win);
        }
    }

    Ok(())
}
