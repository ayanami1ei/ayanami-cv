use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("window size must be odd")]
    WindowSizeMustBeOdd,
    #[error("image size not suitable")]
    ImageSizeNotSuitable,
}
