use crate::channel::Channel;

/// 灰度通道类型
pub struct GrayChannel;

impl Channel for GrayChannel {
    const CHANNEL: usize = 1;
}

/// RGB通道类型
pub struct RgbChannel;

impl Channel for RgbChannel {
    const CHANNEL: usize = 3;
}

/// HSV通道类型
pub struct HsvChannel;

impl Channel for HsvChannel {
    const CHANNEL: usize = 3;
}
