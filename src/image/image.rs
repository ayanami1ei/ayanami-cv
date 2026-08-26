use crate::color_space::ColorSpace;
use crate::image::image_row::ImageRow;
use crate::image::image_row_mut::ImageRowMut;

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

    pub fn new_from_vec(weight: usize, height: usize, vec: &'_ Vec<u8>) -> Self {
        Self {
            weight,
            height,
            data: bytemuck::cast_slice(vec).to_vec(),
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

    // 获取行引用
    pub fn row(&self, row: usize) -> ImageRow<'_, C::PixelType> {
        if row < self.height() {
            let start = row * self.width();
            let end = start + self.width();
            ImageRow::new(&self.data[start..end], self.width())
        } else {
            panic!("range out of bound")
        }
    }

    // 获取可变行引用
    pub fn row_mut(&mut self, row: usize) -> ImageRowMut<'_, C::PixelType> {
        if row < self.height() {
            let start = row * self.width();
            let end = start + self.width();
            let width = self.width();
            ImageRowMut::new(&mut self.data[start..end], width)
        } else {
            panic!("range out of bound")
        }
    }
}
