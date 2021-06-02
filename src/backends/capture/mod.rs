#[cfg(feature = "input-v4l")]
mod v4l2;
#[cfg(feature = "input-v4l")]
pub use v4l2::V4LCaptureDevice;
#[cfg(feature = "input-uvc")]
mod uvc_backend;
#[cfg(feature = "input-uvc")]
pub use uvc_backend::UVCCaptureDevice;
#[cfg(feature = "input-opencv")]
mod opencv_backend;
#[cfg(feature = "input-opencv")]
pub use opencv_backend::OpenCvCaptureDevice;