use http::HeaderMap;
use http::header;

/// An HTTP content length
pub struct ContentLength {
    val: u64,
}

pub struct Error {
    _p: (),
}

impl ContentLength {
    pub fn get(map: &HeaderMap) -> Result<Option<Self>, Error> {
        match map.get(header::CONTENT_LENGTH) {
            Some(_) => unimplemented!(),
            None => Ok(None),
        }
    }
}

impl From<ContentLength> for u64 {
    fn from(src: ContentLength) -> Self {
        src.val
    }
}
