use super::*;
use std::ffi::*;

/* ================================================================================================================= */
/*     FFI                                                                                                           */
/* ================================================================================================================= */

type CHAR = c_char;
type DWORD = c_ulong;
#[allow(non_camel_case_types)]
type DWORD_PTR = ULONG_PTR;
type LPCWAVEFORMATEX = *const WAVEFORMATEX;
type LPSTR = *const CHAR;
type LPHWAVEOUT = *mut HWAVEOUT;
type LPWAVEHDR = *mut WAVEHDR;
type HWAVEOUT = *const c_void;
type MMRESULT = UINT;
type UINT = c_uint;
#[allow(non_camel_case_types)]
type ULONG_PTR = usize;
type WORD = c_ushort;

const CALLBACK_NULL: DWORD = 0x00000000;
const MMSYSERR_NOERROR: MMRESULT = 0;
const WAVE_FORMAT_PCM: WORD = 1;
const WAVE_MAPPER: UINT = 0xffffffff;
const WHDR_DONE: DWORD = 0x00000001;

#[repr(C)]
#[allow(non_snake_case)]
struct WAVEFORMATEX {
    wFormatTag: WORD,
    nChannels: WORD,
    nSamplesPerSec: DWORD,
    nAvgBytesPerSec: DWORD,
    nBlockAlign: WORD,
    wBitsPerSample: WORD,
    cbSize: WORD,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct WAVEHDR {
    lpData: LPSTR,
    dwBufferLength: DWORD,
    dwBytesRecorded: DWORD,
    dwUser: DWORD_PTR,
    dwFlags: DWORD,
    dwLoops: DWORD,
    lpNext: *mut WAVEHDR,
    reserved: DWORD_PTR,
}

#[link(name = "winmm")]
extern "stdcall" {
    fn waveOutClose(hwo: HWAVEOUT) -> MMRESULT;
    fn waveOutOpen(
        phwo: LPHWAVEOUT,
        uDeviceID: UINT,
        pwfx: LPCWAVEFORMATEX,
        dwCallback: DWORD_PTR,
        dwInstance: DWORD_PTR,
        fdwOpen: DWORD,
    ) -> MMRESULT;
    fn waveOutPrepareHeader(hwo: HWAVEOUT, pwh: LPWAVEHDR, cbwh: UINT) -> MMRESULT;
    fn waveOutUnprepareHeader(hwo: HWAVEOUT, pwh: LPWAVEHDR, cbwh: UINT) -> MMRESULT;
    fn waveOutReset(hwo: HWAVEOUT) -> MMRESULT;
    fn waveOutWrite(hwo: HWAVEOUT, pwh: LPWAVEHDR, cbwh: UINT) -> MMRESULT;
}

/* ================================================================================================================= */
/*     Constants                                                                                                     */
/* ================================================================================================================= */

const SIZEOF_HEADER: usize = std::mem::size_of::<WAVEHDR>();
const SIZEOF_SHORT: usize = std::mem::size_of::<i16>();
const FORMAT: WAVEFORMATEX = WAVEFORMATEX {
    wFormatTag: WAVE_FORMAT_PCM,
    nChannels: 1,
    nSamplesPerSec: SAMPLE_RATE as DWORD,
    nAvgBytesPerSec: (SAMPLE_RATE * SIZEOF_SHORT) as DWORD,
    nBlockAlign: SIZEOF_SHORT as WORD,
    wBitsPerSample: 8 * SIZEOF_SHORT as WORD,
    cbSize: 0,
};

/* ================================================================================================================= */
/*     Players                                                                                                       */
/* ================================================================================================================= */

pub type AudioPlayer = HWAVEOUT;

impl super::PlayerImpl for AudioPlayer {
    fn new() -> Result<Self, String> {
        let mut wave_out = std::ptr::null();
        let res = unsafe { waveOutOpen(&mut wave_out, WAVE_MAPPER, &FORMAT, 0, 0, CALLBACK_NULL) };
        if res != MMSYSERR_NOERROR || wave_out == std::ptr::null() {
            Err(format!("failed to create a player : {}", res))
        } else {
            Ok(wave_out)
        }
    }

    fn reset(&self) -> Result<(), String> {
        let res = unsafe { waveOutReset(*self) };
        if res != MMSYSERR_NOERROR {
            Err(format!("failed to reset a player : {}", res))
        } else {
            Ok(())
        }
    }
    
    fn close(self) -> Result<(), String> {
        let res = unsafe { waveOutClose(self) };
        if res != MMSYSERR_NOERROR {
            Err(format!("failed to close a player : {}", res))
        } else {
            Ok(())
        }
    }
}

/* ================================================================================================================= */
/*     AudioHandle                                                                                                   */
/* ================================================================================================================= */

pub struct AudioHandle {
    player: AudioPlayer,
    header: WAVEHDR,
}

impl super::AudioHandleImpl<AudioPlayer> for AudioHandle {
    fn new(player: &AudioPlayer, buffer: &WaveBuffer) -> Result<Self, String> {
        let mut header = WAVEHDR {
            lpData: buffer.as_ptr() as LPSTR,
            dwBufferLength: (buffer.len() * SIZEOF_SHORT) as DWORD,
            dwBytesRecorded: 0,
            dwUser: 0,
            dwFlags: 0,
            dwLoops: 0,
            lpNext: std::ptr::null_mut(),
            reserved: 0,
        };
        let res = unsafe { waveOutPrepareHeader(*player, &mut header, SIZEOF_HEADER as UINT) };
        if res != MMSYSERR_NOERROR {
            Err(format!("failed to prepare wave audio to play it : {}", res))
        } else {
            Ok(AudioHandle {
                player: *player,
                header,
            })
        }
    }

    fn play(&mut self) -> Result<(), String> {
        let res = unsafe { waveOutWrite(self.player, &mut self.header, SIZEOF_HEADER as UINT) };
        if res != MMSYSERR_NOERROR {
            Err(format!("failed to play wave audio : {}", res))
        } else {
            Ok(())
        }
    }

    fn is_playing(&self) -> bool {
        (self.header.dwFlags & WHDR_DONE) == 0
    }

    fn close(mut self) -> Result<(), String> {
        let res =
            unsafe { waveOutUnprepareHeader(self.player, &mut self.header, SIZEOF_HEADER as UINT) };
        if res != MMSYSERR_NOERROR {
            return Err(format!("failed to call waveOutUnprepareHeader : {}", res));
        }
        Ok(())
    }
}
