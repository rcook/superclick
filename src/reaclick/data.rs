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
use anyhow::{bail, Result};
use std::sync::{Arc, Mutex};

pub type DisplayDataRef = Arc<Mutex<DisplayData>>;

#[derive(Clone, Copy)]
pub struct TimeSigCount(i32);

impl TimeSigCount {
    pub fn new(number: i32) -> Result<Self> {
        match number {
            3 | 4 | 6 => Ok(Self(number)),
            _ => bail!("invalid time signature numerator {}", number),
        }
    }

    pub fn as_number(&self) -> i32 {
        self.0
    }

    pub fn note_value(&self) -> f64 {
        match self.0 {
            3 => 1f64,
            4 => 1f64,
            6 => 0.5f64,
            _ => 1f64, /* TBD */
        }
    }

    pub fn is_subaccent(&self, note_index: i32) -> bool {
        match self.0 {
            3 => false,
            4 => note_index == 1 || note_index == 3,
            6 => note_index == 3,
            _ => false, /* TBD */
        }
    }
}

#[derive(Clone, Copy)]
pub struct TimeSigValue(i32);

impl TimeSigValue {
    pub fn new(number: i32) -> Result<Self> {
        match number {
            2 | 4 | 8 | 16 => Ok(Self(number)),
            _ => bail!("invalid time signature denominator {}", number),
        }
    }

    pub fn as_number(&self) -> i32 {
        self.0
    }
}
pub struct Playhead {
    pub tempo: f64,
    pub bar_number: i32,
    pub bar_start_pos_crotchets: f64,
    pub pos_crotchets: f64,
    pub time_sig_numerator: TimeSigCount,
    pub time_sig_denominator: TimeSigValue,
}

#[derive(Default)]
pub struct DisplayData {
    pub sample_rate: f32,
    pub min_buffer_size: Option<u32>,
    pub max_buffer_size: u32,
    pub samples: usize,
    pub channels: usize,
    pub playhead: Option<Playhead>,
}

impl DisplayData {
    pub fn new() -> DisplayDataRef {
        Arc::new(Mutex::new(Self::default()))
    }
}
