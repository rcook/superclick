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
use std::sync::{Arc, Mutex};

pub type DisplayDataRef = Arc<Mutex<DisplayData>>;

#[derive(Default)]
pub struct DisplayData {
    pub sample_rate: f32,
    pub min_buffer_size: Option<u32>,
    pub max_buffer_size: u32,
    pub samples: usize,
    pub tempo: f64,
    pub bar_number: i32,
    pub bar_start_pos_beats: f64,
    pub pos_beats: f64,
    pub time_sig_numerator: i32,
    pub time_sig_denominator: i32,
}

impl DisplayData {
    pub fn new() -> DisplayDataRef {
        Arc::new(Mutex::new(Self::default()))
    }
}
