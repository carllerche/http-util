use header::HdrVal;

/// Parses a comma delimited string into each individual value.
pub struct CommaDelimited<'a> {
    inner: &'a HdrVal,
}

impl<'a> CommaDelimited<'a> {
    /// Return a new comma delimited parser
    pub fn new(inner: &'a HdrVal) -> Self {
        CommaDelimited {
            inner,
        }
    }
}

impl<'a> Iterator for CommaDelimited<'a> {
    type Item = &'a HdrVal;

    fn next(&mut self) -> Option<Self::Item> {
        let mut begin = 0;
        let mut end = 0;

        for i in 0..self.inner.len() {
            match self.inner.as_bytes()[i] {
                b' ' => {
                    if begin == end {
                        begin = i + 1;
                        end = i + 1;
                    }
                }
                b',' => {
                    if begin < end {
                        let ret = &self.inner[begin..end];
                        self.inner = &self.inner[(i + 1)..];

                        return Some(ret);
                    } else {
                        begin = i;
                        end = i;
                    }
                }
                _ => {
                    end += 1;
                }
            }
        }

        if end > begin {
            let ret = &self.inner[begin..end];
            self.inner = &self.inner[self.inner.len()..];

            return Some(ret);
        }

        None
    }
}

#[test]
fn test_parse_empty() {
    let v = HdrVal::from_str("").unwrap();
    let res: Vec<_> = CommaDelimited::new(v).collect();

    assert!(res.is_empty());
}

#[test]
fn test_parse_one_no_space() {
    let v = HdrVal::from_str("foo").unwrap();
    let res: Vec<_> = CommaDelimited::new(v).collect();

    assert_eq!(res, &["foo"]);
}

#[test]
fn test_parse_one_spaces() {
    let v = HdrVal::from_str(" foo   ").unwrap();
    let res: Vec<_> = CommaDelimited::new(v).collect();

    assert_eq!(res, &["foo"]);
}

#[test]
fn test_trailing_comma() {
    let v = HdrVal::from_str(" foo ,").unwrap();
    let res: Vec<_> = CommaDelimited::new(v).collect();

    assert_eq!(res, &["foo"]);
}

#[test]
fn test_two() {
    for s in &["foo,bar", "foo, bar", "foo ,bar"] {
        let v = HdrVal::from_str(s).unwrap();
        let res: Vec<_> = CommaDelimited::new(v).collect();

        assert_eq!(res, &["foo", "bar"]);
    }
}
