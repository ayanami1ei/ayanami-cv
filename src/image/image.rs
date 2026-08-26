use crate::color_space::ColorSpace;
use crate::image::iter::pixel_iter::ImagePixelIter;
use crate::image::iter::pixel_iter_mut::ImagePixelIterMut;
use crate::image::iter::row_iter::ImageRowIter;
use crate::image::iter::row_iter_mut::ImageRowIterMut;

pub struct Image<C: ColorSpace> {
    weight: usize,
    height: usize,
    data: Vec<C::PixelType>,
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

    pub fn new_from_vec(weight: usize, height: usize, vec: Vec<u8>) -> Self {
        Self {
            weight,
            height,
            data: bytemuck::allocation::cast_vec(vec),
        }
    }

    pub fn width(&self) -> usize {
        self.weight
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn data(&self) -> &[u8] {
        bytemuck::cast_slice(&self.data)
    }

    pub fn data_mut(&mut self) -> &mut [u8] {
        bytemuck::cast_slice_mut(&mut self.data)
    }

    pub fn row_iter(&self)->ImageRowIter<'_, C>{
        ImageRowIter::new(&self.data, self.height, self.width())
    }

    pub fn row_iter_mut(&mut self)->ImageRowIterMut<'_, C>{
        ImageRowIterMut::new(self.data.as_mut_ptr(), self.height, self.width())
    }

    pub fn pixel_iter(&self)->ImagePixelIter<'_, C>{
        ImagePixelIter::new(&self.data, self.height*self.weight)
    }

    pub fn pixel_iter_mut(&mut self)->ImagePixelIterMut<'_, C>{
        ImagePixelIterMut::new(self.data.as_mut_ptr(), self.height*self.weight)
    }

    pub fn at(&self, index:(usize, usize))->&C::PixelType{
        &self.data[index.0*self.height+index.1] 
    }

    pub fn at_mut(&mut self, index:(usize, usize))->&mut C::PixelType{
        &mut self.data[index.0*self.height+index.1] 
    }
}
