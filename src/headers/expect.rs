use http::HeaderMap;
use http::header::{self, HeaderValue};

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Expect<T> {
    _p: ::std::marker::PhantomData<T>,
}

#[derive(Debug)]
pub struct Error {
    _p: (),
}

const CONTINUE: &'static [u8] = b"100-continue";

impl<T> Expect<T> {
    pub fn is_continue(&self) -> bool {
        // Always true
        true
    }
}

impl Expect<HeaderValue> {
    pub fn get(map: &HeaderMap) -> Result<Option<Self>, Error> {
        match map.get(header::EXPECT) {
            Some(ref v) if v.as_bytes().eq_ignore_ascii_case(CONTINUE) => {
                Ok(Some(Expect::default()))
            }
            Some(_) => {
                Err(Error { _p: () })
            }
            None => Ok(None),
        }
    }
}

impl<T> Default for Expect<T> {
    fn default() -> Self {
        Expect { _p: ::std::marker::PhantomData }
    }
}
