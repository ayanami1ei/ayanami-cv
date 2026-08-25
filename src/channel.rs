use crate::pixel::Pixel;

pub trait Channel<P:Pixel>{
    const CHANNEL: usize;
}