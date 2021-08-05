//! subsl extends the slices you know and love with some additional
//! functionality.
//!
//! For example, usually, you can't split a byte slice on a subslice;
//! with subsl, you can!
//!
//! # Examples
//!
//! ```rust
//! use subsl::SubslSplitter;
//!
//! let http_get: &[u8] = &*b"GET / HTTP/1.0\r\n\r\nsome data in the body";
//! let sep = b"\r\n\r\n";
//! let mut iter = http_get.subsl_split(sep);
//! let headers = iter.next().unwrap();
//! let body = iter.next().unwrap();
//!
//! assert_eq!(headers, b"GET / HTTP/1.0");
//! assert_eq!(body, b"some data in the body");
//! ```

use std::iter::Iterator;

pub trait SubslSplitter<'a, T: PartialEq> {
    fn subsl_split(&'a self, pat: &'a [T]) -> SubslSplit<'a, T>;
}

impl<'a, T> SubslSplitter<'a, T> for &'a [T]
where
    T: PartialEq,
{
    fn subsl_split(&'a self, pat: &'a [T]) -> SubslSplit<'a, T> {
        SubslSplit::new(self, pat)
    }
}

pub struct SubslSplit<'a, T: PartialEq> {
    start: usize,
    end: usize,
    ndl: &'a [T],
    hay: &'a [T],
}

impl<'a, T: PartialEq> SubslSplit<'a, T> {
    pub fn new(haystack: &'a [T], needle: &'a [T]) -> Self {
        Self {
            start: 0,
            end: 0,
            ndl: needle,
            hay: haystack,
        }
    }
}

impl<'a, T: PartialEq> Iterator for SubslSplit<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        // No more input
        if self.end >= self.hay.len() && self.start >= self.end {
            return None;
        }

        // Empty slice case, just return the whole input
        // and exhaust the rest of the state in the iterator.
        if self.ndl.is_empty() {
            self.end = self.hay.len();
            self.start = self.end;
            return Some(self.hay);
        }

        let len = self.hay.len();

        // Scan for the needle
        while self.end < len {
            // Found it, let's emit it
            if self.hay[self.end..].starts_with(self.ndl) {
                let it = &self.hay[self.start..self.end];
                self.end += self.ndl.len();
                self.start = self.end;
                return Some(it);
            }

            self.end += 1;
        }

        // If we are here, we did not find a needle this time
        // around. Emit the rest of the input.
        let it = &self.hay[self.start..self.end];
        self.start = self.end;
        Some(it)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    // FIXME: find a less gross way to represent slices of byte string literals
    // for these test cases.

    #[rstest]
    #[case(b"hello", b"test", &[&b"hello"[..]])]
    #[case(b"hello", b"ll", &[&b"he"[..], &b"o"[..]])]
    #[case(b"lll", b"l", &[&b""[..], &b""[..], &b""[..]])]
    #[case(b"hi", b"", &[&b"hi"[..]])]
    #[case(b"hi", b"hi", &[&b""[..]])]
    #[case(b"", b"hi", &[])]
    fn subsl_test(#[case] input: &[u8], #[case] split: &[u8], #[case] expected: &[&[u8]]) {
        let actual = input.subsl_split(split).collect::<Vec<&[u8]>>();
        assert_eq!(&actual[..], &expected[..]);
    }
}
