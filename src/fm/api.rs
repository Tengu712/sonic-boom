#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "linux")]
pub use linux::*;
#[cfg(target_os = "windows")]
pub use windows::*;

use super::*;

pub trait PlayerImpl: Sized {
    fn new() -> Result<Self, String>;
    /// A method to reset sound buffer.
    /// If there is data in its queue, it will all be cleared.
    fn reset(&self) -> Result<(), String>;
    fn close(self) -> Result<(), String>;
}

pub trait AudioHandleImpl<P: PlayerImpl>: Sized {
    fn new(player: &P, buffer: &WaveBuffer) -> Result<Self, String>;
    /// A method to play wave sound.
    /// If an audio player is currently playing a sound, any new sound data will be enqueued into its sound buffer.
    fn play(&mut self) -> Result<(), String>;
    fn is_playing(&self) -> bool;
    fn close(self) -> Result<(), String>;
}
