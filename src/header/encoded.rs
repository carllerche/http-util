use super::HdrVal;
use http::header::{self, GetAll, HeaderValue};

use std::option;

/// An encoded header value.
///
/// A single header value can be represented in multiple fields. In this case,
/// the value is equivalent to a comma separated list of all the fields.
pub trait Encoded: for<'a> Iterable<'a> {
    /// Returns true if the encoded header value is empty.
    fn is_empty(&self) -> bool {
        self.iter().next().is_none()
    }
}

/// Iterate header values
pub trait Iterable<'a> {
    type Iter: Iterator<Item = &'a HdrVal>;

    fn iter(&'a self) -> Self::Iter;
}

pub struct ValueIter<'a> {
    inner: header::ValueIter<'a, HeaderValue>,
}

// ==== impl HeaderValue =====

impl Encoded for HeaderValue {
}

impl<'a> Iterable<'a> for HeaderValue {
    type Iter = option::IntoIter<&'a HdrVal>;

    fn iter(&'a self) -> Self::Iter {
        Some(self.as_ref()).into_iter()
    }
}

// ==== impl &HeaderValue =====

impl<'a> Encoded for &'a HeaderValue {
}

impl<'a, 'b> Iterable<'a> for &'b HeaderValue {
    type Iter = option::IntoIter<&'a HdrVal>;

    fn iter(&'a self) -> Self::Iter {
        Some(self.as_ref()).into_iter()
    }
}

// ===== impl GetAll =====

impl<'a> Encoded for GetAll<'a, HeaderValue> {
}

impl<'a, 'b> Iterable<'a> for GetAll<'b, HeaderValue> {
    type Iter = ValueIter<'a>;

    fn iter(&'a self) -> Self::Iter {
        let inner = GetAll::iter(self);
        ValueIter { inner }
    }
}

// ===== impl ValueIter =====

impl<'a> Iterator for ValueIter<'a> {
    type Item = &'a HdrVal;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(AsRef::as_ref)
    }
}
