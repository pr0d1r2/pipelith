//! The `## caches` section: rows by kind, lookups, the built-in
//! fallback and every refusal (T40, V82, V83).

use crate::cache::{
    CACHES, COLUMNS, CacheKind, CacheValue, Caches, Push, SECTION, builtin,
};
use crate::registry::{Schema, parse};

const FILE: &str = ".lablith";

const KEY: &str = "example-cache-1:\
    NfWjbhgAj41byXhCKiaE+av3Vnphm1fTezHXEGsiQIM=";

/// The caches of a registry file with `rows` under the caches header,
/// which starts on line 4, so the first row is line 5.
fn caches(rows: &str) -> Result<Caches, String> {
    let text = format!("format 1\n\n## caches\nname|kind|url|key|push\n{rows}");
    let schema = Schema {
        sections: &[CACHES],
    };
    let doc = parse(FILE, &text, &schema).map_err(|e| e.to_string())?;
    Caches::from_document(FILE, &doc).map_err(|e| e.to_string())
}

fn refused(rows: &str) -> String {
    caches(rows).err().unwrap_or_default()
}

fn lit(text: &str) -> CacheValue {
    CacheValue::Literal(text.to_owned())
}

fn secret(name: &str) -> CacheValue {
    CacheValue::Secret(name.to_owned())
}

#[test]
fn the_section_is_caches() {
    assert_eq!(SECTION, "caches");
    assert_eq!(COLUMNS, ["name", "kind", "url", "key", "push"]);
    assert_eq!(CACHES.name, SECTION);
    assert_eq!(CACHES.header(), "name|kind|url|key|push");
    assert_eq!((CACHES.required, CACHES.key), (false, Some(0)));
}

#[test]
fn without_rows_only_the_builtin_caches_are_found() -> Result<(), String> {
    let schema = Schema {
        sections: &[CACHES],
    };
    let doc = parse(FILE, "format 1\n", &schema).map_err(|e| e.to_string())?;
    let found = Caches::from_document(FILE, &doc).map_err(|e| e.to_string())?;
    assert_eq!(found, Caches::default());
    assert_eq!(found.find("pr0d1r2"), builtin("pr0d1r2"));
    assert_eq!(found.find("example-cache"), None);
    assert_eq!(found.names(), ["pr0d1r2"]);
    assert!(found.is_empty() && !found.has_row("pr0d1r2"));
    Ok(())
}

#[test]
fn an_https_row_with_literals() -> Result<(), String> {
    let found = caches(&format!(
        "example-cache|https|https://cache.example.org|{KEY}|-\n"
    ))?;
    let cache = found.find("example-cache").ok_or("not found")?;
    assert_eq!(cache.kind(), CacheKind::Https);
    assert_eq!(cache.url(), &lit("https://cache.example.org"));
    assert_eq!(cache.key(), &lit(KEY));
    assert_eq!(cache.push(), &Push::ReadOnly);
    assert_eq!(found.names(), ["example-cache", "pr0d1r2"]);
    Ok(())
}

#[test]
fn a_row_is_a_row_and_a_builtin_name_is_not() -> Result<(), String> {
    let found = caches(&format!(
        "example-cache|https|https://cache.example.org|{KEY}|-\n"
    ))?;
    assert!(!found.is_empty() && found.has_row("example-cache"));
    assert!(!found.has_row("pr0d1r2"));
    Ok(())
}

#[test]
fn an_https_row_with_secrets() -> Result<(), String> {
    let found = caches(
        "hidden|https|secret:CACHE_URL|secret:CACHE_KEY|secret:CACHE_NETRC\n",
    )?;
    let cache = found.find("hidden").ok_or("not found")?;
    assert_eq!(cache.url(), &secret("CACHE_URL"));
    assert_eq!(cache.key(), &secret("CACHE_KEY"));
    assert_eq!(cache.push(), &Push::Secret("CACHE_NETRC".to_owned()));
    assert_eq!(cache.read_secrets(), ["CACHE_URL", "CACHE_KEY"]);
    Ok(())
}

const CACHIX_ROWS: &str = "pr0d1r2|cachix|-|-|secret:OWN_CACHIX_TOKEN
example-cache|cachix|https://example-cache.cachix.org|secret:EXAMPLE_KEY|-
";

#[test]
fn a_cachix_row_fills_the_builtin_url_and_key() -> Result<(), String> {
    let found = caches(CACHIX_ROWS)?;
    let own = found.find("pr0d1r2").ok_or("not found")?;
    let builtin = builtin("pr0d1r2").ok_or("no built-in")?;
    assert_eq!((own.url(), own.key()), (builtin.url(), builtin.key()));
    assert_eq!(own.push(), &Push::Secret("OWN_CACHIX_TOKEN".to_owned()));
    assert_eq!(own.nix_config(), builtin.nix_config());
    assert_eq!(found.names(), ["pr0d1r2", "example-cache"]);
    Ok(())
}

#[test]
fn a_cachix_row_takes_its_url_and_a_secret_key() -> Result<(), String> {
    let found = caches(CACHIX_ROWS)?;
    let other = found.find("example-cache").ok_or("not found")?;
    assert_eq!(other.url(), &lit("https://example-cache.cachix.org"));
    assert_eq!(other.key(), &secret("EXAMPLE_KEY"));
    assert!(!other.can_push());
    Ok(())
}

#[test]
fn a_repeated_name_is_refused() {
    let error = refused(&format!(
        "example-cache|https|https://a.example.org|{KEY}|-\n\
         example-cache|https|https://b.example.org|{KEY}|-\n"
    ));
    assert!(error.starts_with(".lablith:6:1: "), "{error}");
    assert!(error.contains("example-cache"), "{error}");
}

#[test]
fn an_unknown_kind_is_refused() {
    assert_eq!(
        refused("example-cache|http|https://cache.example.org|-|-\n"),
        ".lablith:5:15: cache kind `http` is unknown (use one of: cachix, \
         https)"
    );
}

#[test]
fn an_empty_name_or_kind_is_refused() {
    assert!(refused("-|https|-|-|-\n").contains("`name` is empty"));
    assert!(refused("x|-|-|-|-\n").contains("`kind` is empty"));
}

#[test]
fn a_name_that_is_not_a_cache_name_is_refused() {
    assert_eq!(
        refused("-x|https|https://cache.example.org|secret:K|-\n"),
        ".lablith:5:1: cache name `-x` does not fit kind `https` (use \
         letters, digits, `.`, `_`, `-`; no leading `-`)"
    );
}

#[test]
fn a_name_that_is_not_a_cachix_name_is_refused() {
    let want = "does not fit kind `cachix` (a cachix name is lower-case \
                letters, digits and `-`, not starting with `-`)";
    for name in ["Example", "a_b", "-a", "a.b"] {
        let error = refused(&format!("{name}|cachix|-|secret:K|-\n"));
        assert!(error.ends_with(want), "{error}");
    }
}

/// The message for a bad secret name, after the name.
const NOT_A_SECRET: &str = "is not a secret name (write `secret:NAME`: \
    letters, digits and `_`, not starting with a digit)";

#[test]
fn a_bad_secret_name_is_refused_in_every_cell() {
    for (row, column) in [
        ("x|https|secret:1URL|secret:K|-", 9),
        ("x|https|secret:U|secret:A-B|-", 18),
        ("x|https|secret:U|secret:K|secret:", 27),
    ] {
        let error = refused(&format!("{row}\n"));
        let at = format!(".lablith:5:{column}: secret `");
        assert!(error.starts_with(&at), "{error}");
        assert!(error.ends_with(NOT_A_SECRET), "{error}");
    }
}

#[test]
fn a_literal_url_must_be_https() {
    assert_eq!(
        refused(&format!("x|https|http://cache.example.org|{KEY}|-\n")),
        ".lablith:5:9: url `http://cache.example.org` is not an https url \
         (write `https://<host>...`, or `secret:NAME` to keep it out of the \
         file)"
    );
}

#[test]
fn a_literal_url_with_a_credential_is_refused_unechoed() {
    let error = refused(&format!(
        "x|https|https://user:hunter2@cache.example.org|{KEY}|-\n"
    ));
    assert_eq!(
        error,
        ".lablith:5:9: url holds a credential before `@`, so it is refused \
         and not echoed (put the credential in the push netrc secret, or \
         write the url as `secret:NAME`)"
    );
    assert!(!error.contains("hunter2"), "{error}");
}

#[test]
fn a_literal_url_without_a_host_is_refused() {
    let error = refused(&format!("x|https|https://:8443|{KEY}|-\n"));
    assert!(error.contains("is not an https url"), "{error}");
}

#[test]
fn a_malformed_literal_key_is_refused() {
    assert_eq!(
        refused("x|https|https://cache.example.org|example-cache-1:abc|-\n"),
        ".lablith:5:35: key `example-cache-1:abc` is not a nix public key \
         (write `<keyname>:<base64 of 32 bytes>`, or `secret:NAME`)"
    );
    let error = refused("x|cachix|-|x.cachix.org-1:abc|-\n");
    assert!(error.contains("is not a nix public key"), "{error}");
}

#[test]
fn a_literal_push_is_refused() {
    assert_eq!(
        refused(&format!(
            "x|https|https://cache.example.org|{KEY}|machine x login y\n"
        )),
        format!(
            ".lablith:5:{}: `push` is a literal: a push credential is never \
             written in the config file (write `secret:NAME`, or `-` for \
             read-only)",
            36 + KEY.len()
        )
    );
}

#[test]
fn an_https_cache_needs_url_and_key() {
    assert_eq!(
        refused(&format!("x|https|-|{KEY}|-\n")),
        ".lablith:5:9: an https cache needs its `url` (write the url, or \
         `secret:NAME`)"
    );
    assert_eq!(
        refused("x|https|https://cache.example.org|-|-\n"),
        ".lablith:5:35: an https cache needs its `key` (write the key, or \
         `secret:NAME`)"
    );
}

#[test]
fn a_cachix_url_is_its_names() {
    for url in ["https://other.cachix.org", "secret:CACHE_URL"] {
        assert_eq!(
            refused(&format!("pr0d1r2|cachix|{url}|-|-\n")),
            ".lablith:5:16: a cachix cache's url is \
             `https://pr0d1r2.cachix.org` (write `-` or that url)"
        );
    }
}

#[test]
fn a_cachix_key_is_needed_unless_built_in() {
    assert_eq!(
        refused("example-cache|cachix|-|-|-\n"),
        ".lablith:5:24: cachix cache `example-cache` has no built-in public \
         key (write its key, `<name>.cachix.org-1:<base64>`, or \
         `secret:NAME`)"
    );
}
