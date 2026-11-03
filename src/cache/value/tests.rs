//! Cell values and the literal shapes (T40, V83).

use super::{CacheValue, https_url, public_key, secret_name};

const KEY: &str = "example-cache-1:\
    NfWjbhgAj41byXhCKiaE+av3Vnphm1fTezHXEGsiQIM=";

#[test]
fn a_secret_prefix_names_a_secret() {
    let value = CacheValue::read("secret:CACHE_URL");
    assert_eq!(value, CacheValue::Secret("CACHE_URL".to_owned()));
    assert_eq!(value.secret(), Some("CACHE_URL"));
    assert_eq!(value.literal(), None);
}

#[test]
fn anything_else_is_a_literal() {
    for text in ["https://cache.example.org", "Secret:X", "secretX"] {
        let value = CacheValue::read(text);
        assert_eq!(value, CacheValue::Literal(text.to_owned()));
        assert_eq!(value.literal(), Some(text));
        assert_eq!(value.secret(), None);
    }
}

#[test]
fn secret_names_are_words_not_starting_with_a_digit() {
    for name in ["CACHE_URL", "_x", "a1"] {
        assert!(secret_name(name), "{name}");
    }
    for name in ["", "1ST", "CACHE-URL", "A B", "a.b", "\u{c9}"] {
        assert!(!secret_name(name), "{name}");
    }
}

const GOOD_URLS: [&str; 3] = [
    "https://cache.example.org",
    "https://cache.example.org/",
    "https://cache.example.org:8443/nix?priority=40",
];

const BAD_URLS: [&str; 7] = [
    "http://cache.example.org",
    "https://",
    "https:///path",
    "cache.example.org",
    "https://cache.example.org two",
    "https://cache.example.org\n",
    "HTTPS://cache.example.org",
];

const BAD_KEYS: [&str; 10] = [
    "",
    "example-cache-1",
    ":NfWjbhgAj41byXhCKiaE+av3Vnphm1fTezHXEGsiQIM=",
    "a b:NfWjbhgAj41byXhCKiaE+av3Vnphm1fTezHXEGsiQIM=",
    "x:NfWjbhgAj41byXhCKiaE+av3Vnphm1fTezHXEGsiQIM",
    "x:NfWjbhgAj41byXhCKiaE+av3Vnphm1fTezHXEGsiQIN=",
    "x:NfWjbhgAj41byXhCKiaE+av3Vnphm1fTezHXEGsiQI=",
    "x:NfWjbhgAj41byXhCKiaE+av3Vnphm1fTezHXEGsi-M=",
    "x:NfWjbhgAj41byXhCKiaE+av3Vnphm1fTezHXEGsiQIM==",
    "x:y:NfWjbhgAj41byXhCKiaE+av3Vnphm1fTezHXEGsiQIM=",
];

#[test]
fn a_literal_url_is_https_with_a_host() {
    assert_eq!(GOOD_URLS.map(https_url), [true; 3]);
    assert_eq!(BAD_URLS.map(https_url), [false; 7]);
}

#[test]
fn a_literal_key_is_a_name_and_an_ed25519_key() {
    assert!(public_key(KEY));
    assert!(public_key(
        "cache.nixos.org-1:6NCHdD59X431o0gWypbMrAURkbJ16ZPMQFGspcDShjY="
    ));
    assert_eq!(BAD_KEYS.map(public_key), [false; 10]);
}
