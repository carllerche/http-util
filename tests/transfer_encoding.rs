extern crate http;
extern crate http_util;

use http::*;
use http_util::headers::TransferEncoding;

#[test]
fn empty_map() {
    let map = HeaderMap::new();

    let v = TransferEncoding::get(&map);
    assert!(v.is_empty());
    assert!(!v.is_chunked());
}

#[test]
fn chunked() {
    let mut map = HeaderMap::new();
    map.insert("transfer-encoding", "chunked".parse().unwrap());

    let v = TransferEncoding::get(&map);
    assert!(!v.is_empty());
    assert!(v.is_chunked());
}

#[test]
fn brotli() {
    let mut map = HeaderMap::new();
    map.insert("transfer-encoding", "brotli".parse().unwrap());

    let v = TransferEncoding::get(&map);
    assert!(!v.is_empty());
    assert!(!v.is_chunked());
}

#[test]
fn comma_brotli_chunked() {
    let mut map = HeaderMap::new();
    map.insert("transfer-encoding", "brotli, chunked".parse().unwrap());

    let v = TransferEncoding::get(&map);
    assert!(!v.is_empty());
    assert!(v.is_chunked());
}

#[test]
fn comma_chunked_brotli() {
    let mut map = HeaderMap::new();
    map.insert("transfer-encoding", "chunked, brotli".parse().unwrap());

    let v = TransferEncoding::get(&map);
    assert!(!v.is_empty());
    assert!(v.is_chunked());
}

#[test]
fn multi_brotli_chunked() {
    let mut map = HeaderMap::new();
    map.append("transfer-encoding", "brotli".parse().unwrap());
    map.append("transfer-encoding", "chunked".parse().unwrap());

    let v = TransferEncoding::get(&map);
    assert!(!v.is_empty());
    assert!(v.is_chunked());
}

#[test]
fn multi_chunked_brotli() {
    let mut map = HeaderMap::new();
    map.append("transfer-encoding", "chunked".parse().unwrap());
    map.append("transfer-encoding", "brotli".parse().unwrap());

    let v = TransferEncoding::get(&map);
    assert!(!v.is_empty());
    assert!(v.is_chunked());
}
