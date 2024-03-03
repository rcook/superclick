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
pub enum Channel {
    Left,
    Right,
    Both,
}

pub struct Click {
    pub channel: Channel,
    pub frequency: f32,
    pub length: f64,
}

impl Click {
    /// Body Beat Pulse Solo accent (high-intensity) click
    pub const ACCENT: Click = Click {
        channel: Channel::Right,
        frequency: 400f32,
        length: 0.125f64,
    };

    /// Body Beat Pulse Solo subaccent (medium-intensity) click
    pub const SUBACCENT: Click = Click {
        channel: Channel::Left,
        frequency: 800f32,
        length: 0.125f64,
    };

    /// Body Beat Pulse Solo normal (low-intensity) click
    pub const NORMAL: Click = Click {
        channel: Channel::Both,
        frequency: 1_600f32,
        length: 0.125f64,
    };
}
