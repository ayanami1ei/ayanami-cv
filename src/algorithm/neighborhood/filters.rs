use filter_derive::filter;

use crate::{
    Gray, GrayPixel, ImageViewLike, ImageViewMutLike,
    algorithm::neighborhood::{NeighborhoodAlgorithm, WindowLike, error::Error, neighborhood},
};

#[filter]
pub struct MeanFilter;

impl NeighborhoodAlgorithm for MeanFilter {
    fn process<W: WindowLike<Gray>>(&mut self, window: &W) -> GrayPixel {
        let range = (W::SIZE / 2) as i32;
        let mut sum = 0u64;
        for i in -range..=range {
            for j in -range..=range {
                sum += window.at(i, j).gray as u64;
            }
        }
        GrayPixel {
            gray: (sum / (W::SIZE * W::SIZE) as u64) as u8,
        }
    }
}
