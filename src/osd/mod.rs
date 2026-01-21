//
// macOS
//
#[cfg(target_os = "macos")]
mod macos;
use anyhow::Result;
//#[cfg(target_os = "macos")]
pub use macos::MacOsd as Osd;

//
// X11
//
#[cfg(target_os = "linux")]
mod x11;
#[cfg(target_os = "linux")]
pub use x11::X11Osd as Osd;

pub trait OsdTrait: Sized {
    fn new() -> Result<Self>;
    fn display<S>(&self, text: S) -> Result<()>
    where
        S: AsRef<str>;
}
