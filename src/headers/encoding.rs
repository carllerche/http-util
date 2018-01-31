use header::{self, HdrVal};
use headers::parse::CommaDelimited;

use std::fmt;

/// A value to represent an encoding used in `Transfer-Encoding`
/// or `Accept-Encoding` header.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Encoding<T> {
    kind: Kind<T>,
}

/// Represents one or more encodings
#[derive(Clone, Debug)]
pub struct Encodings<T> {
    kind: Kind<T>,
}

/// Iterate encodings
pub struct Iter<'a> {
    // TODO: unbox
    inner: Option<Kind<Box<Iterator<Item = Encoding<&'a HdrVal>> + 'a>>>,
}

/// Encoding kinds
#[derive(Clone, PartialEq, Eq, Debug)]
enum Kind<T> {
    Chunked,
    Brotli,
    Gzip,
    Deflate,
    Compress,
    Identity,
    Trailers,
    Ext(T),
}

// ===== impl Encoding =====

impl<T> Encoding<T> {
    /// Chunked encoding
    pub const CHUNKED: Encoding<T> = Encoding {
        kind: Kind::Chunked,
    };

    /// Convert an `Encoding` from the provided bytes.
    pub fn parse(src: T) -> Encoding<T>
    where T: AsRef<HdrVal>,
    {
        use self::Kind::*;

        match src.as_ref().as_bytes() {
            b"chunked" => Chunked.into(),
            b"br" => Brotli.into(),
            b"deflate" => Deflate.into(),
            b"gzip" => Gzip.into(),
            b"compress" => Compress.into(),
            b"identity" => Identity.into(),
            b"trailers" => Trailers.into(),
            _ => Ext(src).into(),
        }
    }
}

impl<T: fmt::Display> fmt::Display for Encoding<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use self::Kind::*;

        match self.kind {
            Chunked => fmt.write_str("chunked"),
            Brotli => fmt.write_str("br"),
            Gzip => fmt.write_str("gzip"),
            Deflate => fmt.write_str("deflate"),
            Compress => fmt.write_str("compress"),
            Identity => fmt.write_str("identity"),
            Trailers => fmt.write_str("trailers"),
            Ext(ref s) => s.fmt(fmt),
        }
    }
}

impl<T> From<Kind<T>> for Encoding<T> {
    fn from(kind: Kind<T>) -> Self {
        Encoding { kind }
    }
}

// ===== impl Encodings =====

impl<T> Encodings<T>
where T: header::Encoded,
{
    /// Create a new `Encodings`
    pub fn new(inner: T) -> Self {
        Encodings { kind: Kind::Ext(inner) }
    }

    pub fn is_empty(&self) -> bool {
        use self::Kind::*;

        match self.kind {
            Ext(ref ext) => ext.is_empty(),
            _ => false,
        }
    }

    pub fn iter(&self) -> Iter {
        use self::Kind::*;

        let kind = match self.kind {
            Chunked => Chunked,
            Brotli => Brotli,
            Gzip => Gzip,
            Deflate => Deflate,
            Compress => Compress,
            Identity => Identity,
            Trailers => Trailers,
            Ext(ref ext) => {
                let inner = ext.iter()
                    .flat_map(CommaDelimited::new)
                    .map(Encoding::parse);

                Ext(Box::new(inner) as Box<Iterator<Item = Encoding<&HdrVal>>>)
            }
        };

        Iter { inner: Some(kind) }
    }
}

// ===== impl Iter =====

impl<'a> Iterator for Iter<'a> {
    type Item = Encoding<&'a HdrVal>;

    fn next(&mut self) -> Option<Self::Item> {
        use self::Kind::*;

        match self.inner {
            Some(Chunked) => {
                self.inner = None;
                Some(Encoding { kind: Chunked })
            }
            Some(Brotli) => {
                self.inner = None;
                Some(Encoding { kind: Brotli })
            }
            Some(Gzip) => {
                self.inner = None;
                Some(Encoding { kind: Gzip })
            }
            Some(Deflate) => {
                self.inner = None;
                Some(Encoding { kind: Deflate })
            }
            Some(Compress) => {
                self.inner = None;
                Some(Encoding { kind: Compress })
            }
            Some(Identity) => {
                self.inner = None;
                Some(Encoding { kind: Identity })
            }
            Some(Trailers) => {
                self.inner = None;
                Some(Encoding { kind: Trailers })
            }
            Some(Ext(ref mut ext)) => {
                ext.next()
            }
            None => {
                None
            }
        }
    }
}
