use http::HeaderMap;
use http::header::{self, HeaderValue};

/// `Connection` header, defined in
/// [RFC7230](http://tools.ietf.org/html/rfc7230#section-6.1)
///
/// The `Connection` header field allows the sender to indicate desired control
/// options for the current connection.  In order to avoid confusing downstream
/// recipients, a proxy or gateway MUST remove or replace any received
/// connection options before forwarding the message.
///
/// # ABNF
/// ```plain
/// Connection        = 1#connection-option
/// connection-option = token
///
/// # Example values
/// * `close`
/// * `keep-alive`
/// * `upgrade`
/// ```
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Connection<T> {
    value: Value<T>,
}

#[derive(Debug)]
pub struct Error {
    _p: (),
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum Value<T> {
    KeepAlive,
    Close,
    Upgrade,
    Other(T),
}

const KEEP_ALIVE: &'static [u8] = b"keep-alive";
const CLOSE: &'static [u8] = b"close";
const UPGRADE: &'static [u8] = b"upgrade";

// ===== impl Connection =====

impl<T> Connection<T> {
    pub fn close() -> Self {
        Connection { value: Value::Close }
    }

    pub fn keep_alive() -> Self {
        Connection { value: Value::KeepAlive }
    }

    pub fn upgrade() -> Self {
        Connection { value: Value::Upgrade }
    }

    pub fn other(val: T) -> Self {
        Connection { value: Value::Other(val) }
    }

    pub fn is_close(&self) -> bool {
        match self.value {
            Value::Close => true,
            _ => false,
        }
    }

    pub fn is_keep_alive(&self) -> bool {
        match self.value {
            Value::KeepAlive => true,
            _ => false,
        }
    }

    pub fn is_upgrade(&self) -> bool {
        match self.value {
            Value::Upgrade => true,
            _ => false,
        }
    }
}

impl<'a> Connection<&'a HeaderValue> {
    pub fn get(src: &'a HeaderMap) -> Result<Option<Self>, Error> {
        match src.get(header::CONNECTION) {
            Some(v) => {
                match v.as_bytes() {
                    KEEP_ALIVE => Ok(Some(Self::keep_alive())),
                    CLOSE => Ok(Some(Self::close())),
                    UPGRADE => Ok(Some(Self::upgrade())),
                    _ => Ok(Some(Self::other(v))),
                }
            }
            None => Ok(None),
        }
    }
}
