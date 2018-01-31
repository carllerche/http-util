use header::Encoded;
use headers::encoding::{Encodings, Encoding};

use http::HeaderMap;
use http::header::{self, HeaderValue};

pub struct TransferEncoding<T> {
    encodings: Encodings<T>,
}

pub struct Error {
    _p: (),
}

// ===== impl TransferEncoding =====

impl<T> TransferEncoding<T>
where T: Encoded,
{
    pub fn encodings(&self) -> &Encodings<T> {
        &self.encodings
    }

    /// Returns true if there are no transfer encodings
    pub fn is_empty(&self) -> bool {
        self.encodings.is_empty()
    }

    /// Returns `true` if the transfer encoding value contains `chunked`
    pub fn is_chunked(&self) -> bool {
        self.encodings().iter()
            .any(|e| e == Encoding::CHUNKED)
    }
}

impl<'a> TransferEncoding<header::GetAll<'a, HeaderValue>> {
    pub fn get(map: &'a HeaderMap) -> Self {
        let encodings = Encodings::new(map.get_all(header::TRANSFER_ENCODING));
        TransferEncoding { encodings }
    }
}
