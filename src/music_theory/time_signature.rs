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
use std::fmt::{Display, Formatter, Result as FmtResult};

/// A time signature's top number (i.e. the count)
#[derive(Clone, Copy)]
pub struct TimeSignatureTop(i32);

impl TimeSignatureTop {
    #[allow(dead_code)]
    pub fn as_number(&self) -> i32 {
        self.0
    }

    pub fn basis(&self) -> i32 {
        match self.0 {
            6 | 9 | 12 => 3,
            _ => 1,
        }
    }

    pub fn is_accented(&self, index: i32) -> bool {
        if index == 0 {
            true
        } else {
            match self.0 {
                4 => index == 2,
                6 => index == 3,
                9 => index == 3 || index == 6,
                12 => index == 3 || index == 6 || index == 9,
                _ => false,
            }
        }
    }
}

impl TryFrom<i32> for TimeSignatureTop {
    type Error = Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value >= 1 {
            Ok(Self(value))
        } else {
            Err(Error::InvalidTimeSignatureTop)
        }
    }
}

impl Display for TimeSignatureTop {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

/// A time signature's bottom number (i.e. the note value or quantum)
#[derive(Clone, Copy)]
pub struct TimeSignatureBottom(i32);

impl TimeSignatureBottom {
    pub fn as_number(&self) -> i32 {
        self.0
    }
}

impl TryFrom<i32> for TimeSignatureBottom {
    type Error = Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            2 | 4 | 8 | 16 => Ok(Self(value)),
            _ => Err(Error::InvalidTimeSignatureBottom),
        }
    }
}

impl Display for TimeSignatureBottom {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}
