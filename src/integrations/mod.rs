#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod osx;
#[cfg(target_os = "windows")]
mod windows;

mod shared_api;
pub use self::shared_api::*;

#[cfg(target_os = "linux")]
pub use self::linux::*;
#[cfg(target_os = "macos")]
pub use self::osx::*;
#[cfg(target_os = "windows")]
pub use self::windows::*;
