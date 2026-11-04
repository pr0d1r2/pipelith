//! A cache: kinds, accessors, nix settings and the built-in table (T40,
//! V82, V84, V85).

use super::{
    BinaryCache, CacheKind, Push, builtin, builtin_key, builtin_names,
};
use crate::cache::CacheValue;

/// The nix config of actlith's ci.yml for the `pr0d1r2` cache.
const CI_CONFIG: &str = "extra-substituters = https://pr0d1r2.cachix.org\n\
    extra-trusted-public-keys = pr0d1r2.cachix.org-1:\
    NfWjbhgAj41byXhCKiaE+av3Vnphm1fTezHXEGsiQIM=\n";

const KEY: &str = "example-cache-1:\
    NfWjbhgAj41byXhCKiaE+av3Vnphm1fTezHXEGsiQIM=";

fn lit(text: &str) -> CacheValue {
    CacheValue::Literal(text.to_owned())
}

fn secret(name: &str) -> CacheValue {
    CacheValue::Secret(name.to_owned())
}

/// An https cache `example-cache` with these parts.
fn https(url: CacheValue, key: CacheValue, push: Push) -> BinaryCache {
    BinaryCache::new("example-cache", CacheKind::Https, url, key).pushed(push)
}

#[test]
fn kinds_are_cachix_and_https() {
    let names: Vec<&str> = CacheKind::ALL.iter().map(|k| k.name()).collect();
    assert_eq!(names, ["cachix", "https"]);
    for kind in CacheKind::ALL {
        assert_eq!(CacheKind::parse(kind.name()), Some(*kind));
    }
}

#[test]
fn every_kind_is_a_slice_a_later_kind_keeps() {
    let all: &'static [CacheKind] = CacheKind::ALL;
    assert_eq!(all, [CacheKind::Cachix, CacheKind::Https]);
}

#[test]
fn other_kind_names_are_unknown() {
    let parsed = ["http", "Cachix", "s3", ""].map(CacheKind::parse);
    assert_eq!(parsed, [None; 4]);
}

#[test]
fn the_builtin_cache_keeps_its_bytes() -> Result<(), String> {
    let cache = builtin("pr0d1r2").ok_or("no built-in")?;
    assert_eq!(cache.name(), "pr0d1r2");
    assert_eq!(cache.kind(), CacheKind::Cachix);
    assert_eq!(cache.url(), &lit("https://pr0d1r2.cachix.org"));
    assert_eq!(cache.nix_config().as_deref(), Some(CI_CONFIG));
    Ok(())
}

#[test]
fn the_builtin_cache_pushes_with_the_forge_secret() -> Result<(), String> {
    let cache = builtin("pr0d1r2").ok_or("no built-in")?;
    let key =
        "pr0d1r2.cachix.org-1:NfWjbhgAj41byXhCKiaE+av3Vnphm1fTezHXEGsiQIM=";
    assert_eq!(cache.key(), &lit(key));
    assert_eq!(cache.push(), &Push::Forge);
    assert!(cache.can_push());
    assert!(cache.read_secrets().is_empty());
    Ok(())
}

#[test]
fn only_known_names_are_built_in() {
    assert_eq!(builtin_names(), ["pr0d1r2"]);
    for name in ["nix-community", "pr0d1r", "pr0d1r2.cachix.org-1:", ""] {
        assert_eq!((builtin(name), builtin_key(name)), (None, None), "{name}");
    }
}

#[test]
fn a_literal_https_cache_has_a_literal_config() {
    let url = lit("https://cache.example.org");
    let cache = https(url, lit(KEY), Push::ReadOnly);
    assert!(!cache.can_push());
    assert!(cache.read_secrets().is_empty());
    let want = "extra-substituters = https://cache.example.org\n\
         extra-trusted-public-keys = example-cache-1:\
         NfWjbhgAj41byXhCKiaE+av3Vnphm1fTezHXEGsiQIM=\n";
    assert_eq!(cache.nix_config().as_deref(), Some(want));
}

/// A url held by a secret, then a key held by one, each with the
/// secret name reading needs.
fn secret_parts() -> [(CacheValue, CacheValue, &'static str); 2] {
    [
        (secret("EXAMPLE_URL"), lit(KEY), "EXAMPLE_URL"),
        (
            lit("https://cache.example.org"),
            secret("EXAMPLE_KEY"),
            "EXAMPLE_KEY",
        ),
    ]
}

#[test]
fn a_secret_url_or_key_has_no_literal_config() {
    for (url, key, name) in secret_parts() {
        let cache = https(url, key, Push::ReadOnly);
        assert_eq!(cache.read_secrets(), [name]);
        assert_eq!(cache.nix_config(), None);
    }
}

#[test]
fn a_secret_url_or_key_still_has_its_settings() {
    for (url, key, _) in secret_parts() {
        let cache = https(url.clone(), key.clone(), Push::ReadOnly);
        let want = [
            ("extra-substituters", &url),
            ("extra-trusted-public-keys", &key),
        ];
        assert_eq!(cache.nix_settings(), want);
    }
}

#[test]
fn a_push_secret_takes_a_push() {
    let push = Push::Secret("EXAMPLE_NETRC".to_owned());
    let cache = https(lit("https://cache.example.org"), lit(KEY), push.clone());
    assert!(cache.can_push());
    assert_eq!(cache.push(), &push);
}

#[test]
fn a_read_netrc_is_a_read_secret_and_blocks_the_literal_config() {
    let url = lit("https://cache.example.org");
    let cache = https(url, lit(KEY), Push::ReadOnly)
        .read_with(Some(secret("CACHE_READ_NETRC")));
    assert_eq!(cache.read_netrc(), Some(&secret("CACHE_READ_NETRC")));
    assert_eq!(cache.read_secrets(), ["CACHE_READ_NETRC"]);
    assert_eq!(cache.nix_config(), None);
    assert_eq!(cache.nix_settings()[0].1, &lit("https://cache.example.org"));
}

#[test]
fn without_a_read_netrc_nothing_changes() {
    let cache =
        https(lit("https://cache.example.org"), lit(KEY), Push::ReadOnly);
    assert_eq!(cache.read_netrc(), None);
    assert!(cache.nix_config().is_some());
    assert_eq!(
        builtin("pr0d1r2").and_then(|c| c.read_netrc().cloned()),
        None
    );
}
