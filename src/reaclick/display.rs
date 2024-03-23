// Copyright (c) 2024 Richard Cook
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//
use crate::error::Error;
use crate::music_theory::{TimeSignatureBottom, TimeSignatureTop};
use atomic_float::AtomicF64;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicIsize, Ordering};

pub struct Playhead {
    pub tempo: f64,
    pub bar_number: i32,
    pub bar_start_pos_crotchets: f64,
    pub pos_crotchets: f64,
    pub time_signature_top: TimeSignatureTop,
    pub time_signature_bottom: TimeSignatureBottom,
}

#[derive(Default)]
pub struct Display {
    error_code: AtomicIsize,
    is_playing: AtomicBool,
    tempo: AtomicF64,
    bar_number: AtomicI32,
    bar_start_pos_crotchets: AtomicF64,
    pos_crotchets: AtomicF64,
    time_signature_top: AtomicI32,
    time_signature_bottom: AtomicI32,
}

impl Display {
    pub fn error_code(&self) -> isize {
        self.error_code.load(Ordering::Relaxed)
    }

    pub fn is_playing(&self) -> bool {
        self.is_playing.load(Ordering::Relaxed)
    }

    pub fn tempo(&self) -> f64 {
        self.tempo.load(Ordering::Relaxed)
    }

    pub fn bar_number(&self) -> i32 {
        self.bar_number.load(Ordering::Relaxed)
    }

    pub fn bar_start_pos_crotchets(&self) -> f64 {
        self.bar_start_pos_crotchets.load(Ordering::Relaxed)
    }

    pub fn pos_crotchets(&self) -> f64 {
        self.pos_crotchets.load(Ordering::Relaxed)
    }

    pub fn time_signature_top(&self) -> Option<TimeSignatureTop> {
        self.time_signature_top
            .load(Ordering::Relaxed)
            .try_into()
            .ok()
    }

    pub fn time_signature_bottom(&self) -> Option<TimeSignatureBottom> {
        self.time_signature_bottom
            .load(Ordering::Relaxed)
            .try_into()
            .ok()
    }

    pub fn update(&self, error: Option<Error>, playhead: &Option<Playhead>) {
        if let Some(error) = error {
            self.error_code.store(error as isize, Ordering::Relaxed);
        } else {
            self.error_code.store(isize::default(), Ordering::Relaxed);
        }

        if let Some(ref playhead) = playhead {
            self.is_playing.store(true, Ordering::Relaxed);
            self.tempo.store(playhead.tempo, Ordering::Relaxed);
            self.bar_number
                .store(playhead.bar_number, Ordering::Relaxed);
            self.bar_start_pos_crotchets
                .store(playhead.bar_start_pos_crotchets, Ordering::Relaxed);
            self.pos_crotchets
                .store(playhead.pos_crotchets, Ordering::Relaxed);
            self.time_signature_top
                .store(playhead.time_signature_top.as_number(), Ordering::Relaxed);
            self.time_signature_bottom.store(
                playhead.time_signature_bottom.as_number(),
                Ordering::Relaxed,
            );
        } else {
            self.is_playing.store(false, Ordering::Relaxed);
        }
    }
}
