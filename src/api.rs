#[cfg(target_os = "linux")]
pub(super) mod linux;
#[cfg(target_os = "windows")]
pub(super) mod windows;

#[cfg(target_os = "linux")]
pub(super) use linux::*;
#[cfg(target_os = "windows")]
pub(super) use windows::*;

use super::*;

pub(super) trait AudioPlayerImpl: Sized {
    fn new() -> Result<Self, String>;
    /// A method to reset sound buffer.
    /// If there is data in its queue, it will all be cleared.
    fn reset(&self) -> Result<(), String>;
    fn close(self) -> Result<(), String>;
}

pub(super) trait AudioHandleImpl<P: AudioPlayerImpl>: Sized {
    /// A constructor.
    /// It binds a wave buffer with a player.
    /// When calling `play` method, the player plays the wave buffer.
    fn new(player: &P, buffer: &WaveBuffer) -> Result<Self, String>;
    /// A method to play wave sound.
    /// If an audio player is currently playing a sound, any new sound data will be enqueued into its sound buffer.
    fn play(&self) -> Result<(), String>;
    fn is_playing(&self) -> bool;
    fn close(self) -> Result<(), String>;
}
