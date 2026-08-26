use crate::Image;
use crate::color_space::ColorSpace;
use crate::image::image_row::ImageRow;

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

    // 获取行引用
    pub fn row(&self, row: usize) -> ImageRow<'_, C::PixelType> {
        if row < self.height() {
            let start = row * self.width();
            let end = start + self.width();
            ImageRow::new(
                &self.data[start..end],
                self.width(),
            )
        } else {
            panic!("range out of bound")
        }
    }
}
