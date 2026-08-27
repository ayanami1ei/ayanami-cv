use crate::{
    Gray, GrayPixel, ImageViewLike, ImageViewMutLike,
    algorithm::neighborhood::{NeighborhoodAlgorithm, WindowLike, error::Error, neighborhood},
};

#[derive(Debug, Clone, Copy)]
pub struct MeanFilter;
impl NeighborhoodAlgorithm for MeanFilter {
    fn process<W: WindowLike<Gray>>(&self, window: &W) -> GrayPixel {
        let range = (W::SIZE / 2) as i32;
        let mut sum = 0u64;
        for i in -range..=range {
            for j in -range..=range {
                sum += window.at(i, j).gray as u64;
            }
        }
        GrayPixel { gray: (sum / (W::SIZE * W::SIZE) as u64) as u8 }
    }
}
impl MeanFilter {
    pub fn filter<I: ImageViewLike<Gray>, IMut: ImageViewMutLike<Gray>, const SIZE: usize>(
        &self,
        src: &I,
        dst: &mut IMut,
    ) -> Result<(), Error> {
        neighborhood::<I, IMut, SIZE, MeanFilter>(src, dst, *self)
    }
}
