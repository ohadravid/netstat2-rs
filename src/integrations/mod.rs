#[cfg(target_os = "linux")]
mod linux;
#[cfg(any(target_os = "macos", target_os = "ios"))]
mod osx;
#[cfg(target_os = "windows")]
mod windows;

mod shared_api;
pub use self::shared_api::*;

#[cfg(target_os = "linux")]
pub use self::linux::*;
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub use self::osx::*;
#[cfg(target_os = "windows")]
pub use self::windows::*;
