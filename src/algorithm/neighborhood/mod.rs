use std::marker::PhantomData;

use crate::{
    ImageViewLike, ImageViewMutLike, algorithm::neighborhood::error::Error,
    image::color_space::ColorSpace,
};

pub mod error;
pub mod filters;

pub trait WindowLike<C: ColorSpace> {
    const SIZE: usize;
    fn set_index(&mut self, index: usize);
    fn index(&self) -> usize;
    fn at(&self, x: i32, y: i32) -> C::PixelType;
}

pub struct InteriorWindow<'a, C: ColorSpace, I: ImageViewLike<C>, const SIZE: usize> {
    data: &'a I,
    index: usize,
    pixels: Vec<&'a [C::PixelType]>,
    _mark: PhantomData<C>,
}

impl<'a, C: ColorSpace, I: ImageViewLike<C>, const SIZE: usize> InteriorWindow<'a, C, I, SIZE> {
    fn new(image: &'a I) -> Self {
        Self {
            data: image,
            index: 0,
            pixels: image.pixel(),
            _mark: PhantomData,
        }
    }
}

impl<'a, C: ColorSpace, I: ImageViewLike<C>, const SIZE: usize> WindowLike<C>
    for InteriorWindow<'a, C, I, SIZE>
{
    const SIZE: usize = SIZE;
    fn set_index(&mut self, index: usize) {
        self.index = index;
    }
    fn index(&self) -> usize {
        self.index
    }

    fn at(&self, x: i32, y: i32) -> C::PixelType {
        let w = self.data.width();
        let i = self.index / w;
        let j = self.index % w;
        self.pixels[(i as isize + x as isize) as usize][(j as isize + y as isize) as usize]
    }
}

pub struct BorderWindow<'a, C: ColorSpace, I: ImageViewLike<C>, const SIZE: usize> {
    data: &'a I,
    index: usize,
    pixels: Vec<&'a [C::PixelType]>,
    _mark: PhantomData<C>,
}

impl<'a, C: ColorSpace, I: ImageViewLike<C>, const SIZE: usize> BorderWindow<'a, C, I, SIZE> {
    fn new(image: &'a I) -> Self {
        Self {
            data: image,
            index: 0,
            pixels: image.pixel(),
            _mark: PhantomData,
        }
    }
}

impl<'a, C: ColorSpace, I: ImageViewLike<C>, const SIZE: usize> WindowLike<C>
    for BorderWindow<'a, C, I, SIZE>
{
    const SIZE: usize = SIZE;
    fn set_index(&mut self, index: usize) {
        self.index = index;
    }
    fn index(&self) -> usize {
        self.index
    }

    fn at(&self, x: i32, y: i32) -> C::PixelType {
        let w = self.data.width();
        let h = self.data.height();
        let i = self.index / w;
        let j = self.index % w;
        let ni = i as isize + x as isize;
        let nj = j as isize + y as isize;
        if ni < 0 || ni as usize >= h || nj < 0 || nj as usize >= w {
            return C::PixelType::default();
        }
        self.pixels[ni as usize][nj as usize]
    }
}

pub trait NeighborhoodAlgorithm{
    fn reset(&mut self);
    fn process<C:ColorSpace, W: WindowLike<C>>(&mut self, window: &W)->C::PixelType;
}

pub fn neighborhood<
    'a,
    C:ColorSpace,
    I: ImageViewLike<C> + 'a,
    IMut: ImageViewMutLike<C>,
    const SIZE: usize,
    A: NeighborhoodAlgorithm,
>(
    src: &'a I,
    dst: &mut IMut,
    algor:&mut A,
) -> Result<(), Error> {
    if SIZE % 2 == 0 {
        return Err(Error::WindowSizeMustBeOdd);
    }

    algor.reset();

    let r: usize = SIZE / 2;

    let width = src.width();
    let height = src.height();

    // 内部
    let mut inner_win = InteriorWindow::<C, I, SIZE>::new(src);

    for i in r..height - r {
        for j in r..width - r {
            inner_win.set_index(i * width + j);

            let p = dst.at_mut((i, j));
            *p = algor.process(&inner_win);
        }
    }

    // 边界
    let mut border_win = BorderWindow::<C, I, SIZE>::new(src);

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
