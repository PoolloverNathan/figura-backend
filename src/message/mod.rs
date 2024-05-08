mod c2s;
mod s2c;

use std::fmt::*;
use std::ops::RangeInclusive;

pub enum MessageLoadError {
    BadEnum(&'static str, RangeInclusive<usize>, usize),
    BadLength(&'static str, usize, bool, usize),
}
impl Display for MessageLoadError {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        match self {
            Self::BadEnum(f, r, c) => write!(
                fmt,
                "value {f} must be {} to {} inclusive, got {c}",
                r.start(),
                r.end()
            ),
            Self::BadLength(f, n, e, c) => write!(
                fmt,
                "buffer wrong size for {f} — must be {} {n} bytes, got c",
                if *e { "exactly" } else { "at least" }
            ),
        }
    }
}
