use filter_derive::filter;

use crate::{
    ImageViewLike, ImageViewMutLike,
    algorithm::neighborhood::{NeighborhoodAlgorithm, WindowLike, Error, neighborhood},
    image::color_space::ColorSpace,
};

#[filter]
pub struct MeanFilter {
    sums: Vec<usize>,
    last_index: usize,
    initialized: bool,
}

impl MeanFilter {
    pub fn new() -> Self {
        Self {
            sums: Vec::new(),
            last_index: 0,
            initialized: false,
        }
    }
}

impl NeighborhoodAlgorithm for MeanFilter {
    fn reset(&mut self) {
        self.sums.clear();
        self.last_index = 0;
        self.initialized = false;
    }

    fn process<C: ColorSpace, W: WindowLike<C>>(&mut self, window: &W) -> C::PixelType {
        let half = (W::SIZE / 2) as i32;

        // 用 bytemuck 把任意 PixelType 按字节切片，逐通道滚动求和，与色彩空间无关
        if !self.initialized || window.index() != self.last_index + 1 {
            self.sums.resize(C::CHANNEL, 0);
            self.sums.fill(0);
            for x in -half..=half {
                for y in -half..=half {
                    let p = window.at(x, y);
                    let px = bytemuck::bytes_of(&p);
                    for k in 0..C::CHANNEL {
                        self.sums[k] += px[k] as usize;
                    }
                }
            }
        } else {
            for x in -half..=half {
                let l = window.at(x, -(half + 1));
                let r = window.at(x, half);
                let left = bytemuck::bytes_of(&l);
                let right = bytemuck::bytes_of(&r);
                for k in 0..C::CHANNEL {
                    self.sums[k] -= left[k] as usize;
                    self.sums[k] += right[k] as usize;
                }
            }
        }

        self.last_index = window.index();
        self.initialized = true;

        let mean = (0..C::CHANNEL)
            .map(|k| (self.sums[k] / (W::SIZE * W::SIZE)) as u8)
            .collect::<Vec<_>>();
        bytemuck::cast_slice::<u8, C::PixelType>(&mean)[0]
    }
}