use crate::Image;
use crate::color_space::ColorSpace;
use crate::image::iter::pixel_iter::ImagePixelIter;
use crate::image::iter::row_iter::ImageRowIter;

pub struct ImageView<'a, C: ColorSpace> {
    weight: usize,
    height: usize,
    data: &'a [C::PixelType],
}

impl<'a, C: ColorSpace> ImageView<'a, C> {
    pub fn new(weight: usize, height: usize, vec:&'a [u8])->Self{
        Self { weight, height, data: bytemuck::cast_slice(vec) }
    }

    pub fn new_from_image(image:&'a Image<C>)->Self{
        Self{
            weight: image.width(),
            height: image.height(),
            data: bytemuck::cast_slice(image.data()),
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

    pub fn row_iter(&self)->ImageRowIter<'_, C>{
        ImageRowIter::new(&self.data, self.height, self.width())
    }

    pub fn pixel_iter(&self)->ImagePixelIter<'_, C>{
        ImagePixelIter::new(&self.data, self.height*self.weight)
    }

    pub fn at(&self, index:(usize, usize))->&C::PixelType{
        &self.data[index.0*self.height+index.1] 
    }
}
