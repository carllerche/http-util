use http::header::HeaderValue;

use std::ops::{Index, Range, RangeFrom, RangeTo, RangeFull};

/// A slice of a `HeaderValue`.
#[derive(Debug, Eq, PartialEq)]
pub struct HdrVal {
    inner: [u8],
}

#[derive(Debug)]
pub struct InvalidHdrVal {
    _p: (),
}

impl HdrVal {
    /// Convert a `HdrVal` from a `&str`.
    pub fn from_str(src: &str) -> Result<&HdrVal, InvalidHdrVal> {
        // TODO: Don't make this as crappy
        match HeaderValue::from_str(src) {
            Ok(_) => Ok(unsafe { from_slice(src.as_bytes()) }),
            Err(_) => {
                Err(InvalidHdrVal {
                    _p: (),
                })
            }
        }
    }

    /// Returns true if the `HdrVal` has a length of zero bytes.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Returns the length of this `HdrVal`.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Converts a `HdrVal` to a byte slice.
    pub fn as_bytes(&self) -> &[u8] {
        &self.inner[..]
    }
}

impl AsRef<HdrVal> for HdrVal {
    fn as_ref(&self) -> &HdrVal {
        self
    }
}

impl AsRef<HdrVal> for HeaderValue {
    fn as_ref(&self) -> &HdrVal {
        let s = self.as_bytes();
        unsafe { from_slice(s) }
    }
}

impl Index<Range<usize>> for HdrVal {
    type Output = HdrVal;

    fn index(&self, index: Range<usize>) -> &HdrVal {
        let s = self.inner.index(index);
        unsafe { from_slice(s) }
    }
}

impl Index<RangeFrom<usize>> for HdrVal {
    type Output = HdrVal;

    fn index(&self, index: RangeFrom<usize>) -> &HdrVal {
        let s = self.inner.index(index);
        unsafe { from_slice(s) }
    }
}

impl Index<RangeTo<usize>> for HdrVal {
    type Output = HdrVal;

    fn index(&self, index: RangeTo<usize>) -> &HdrVal {
        let s = self.inner.index(index);
        unsafe { from_slice(s) }
    }
}

impl Index<RangeFull> for HdrVal {
    type Output = HdrVal;

    fn index(&self, index: RangeFull) -> &HdrVal {
        let s = self.inner.index(index);
        unsafe { from_slice(s) }
    }
}

impl PartialEq<str> for HdrVal {
    fn eq(&self, other: &str) -> bool {
        self.inner.eq(other.as_bytes())
    }
}

unsafe fn from_slice(s: &[u8]) -> &HdrVal {
    ::std::mem::transmute(s)
}
