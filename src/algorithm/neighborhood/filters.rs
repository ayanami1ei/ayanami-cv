use filter_derive::filter;

use crate::{
    Gray, GrayPixel, ImageViewLike, ImageViewMutLike,
    algorithm::neighborhood::{NeighborhoodAlgorithm, WindowLike, error::Error, neighborhood},
};

#[filter]
pub struct MeanFilter {
    sum: usize,
    last_index: usize,
    initialized: bool,
}

impl MeanFilter {
    pub fn new() -> Self {
        Self {
            sum: 0,
            last_index: 0,
            initialized: false,
        }
    }
}

impl NeighborhoodAlgorithm for MeanFilter {
    fn reset(&mut self) {
        self.sum = 0;
        self.last_index = 0;
        self.initialized = false;
    }

    fn process<W: WindowLike<Gray>>(&mut self, window: &W) -> GrayPixel {
        let r = (W::SIZE / 2) as i32;

        if !self.initialized || window.index() != self.last_index + 1 {
            // 初始值 & 换行：窗口垂直或跳跃移动，无法增量更新，整体重新求和
            self.sum = 0;
            for x in -r..=r {
                for y in -r..=r {
                    self.sum += window.at(x, y).gray as usize;
                }
            }
        } else {
            // 向右平移一步：减去滑出窗口的左列，加上滑入的右列
            for x in -r..=r {
                self.sum -= window.at(x, -(r + 1)).gray as usize;
                self.sum += window.at(x, r).gray as usize;
            }
        }

        self.last_index = window.index();
        self.initialized = true;

        GrayPixel {
            gray: (self.sum / (W::SIZE * W::SIZE)) as u8,
        }
    }
}