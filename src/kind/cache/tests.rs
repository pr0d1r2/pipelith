//! Cache name to public key and nix config (T14, V53); a `## caches`
//! row beside the built-in table (T40, `src/cache:V82`).

use super::resolve;
use crate::cache::{BinaryCache, CACHES, Caches, Push, builtin};
use crate::kind::{Cache, KindError};
use crate::registry::{Schema, parse};

/// The nix config of actlith's ci.yml for the `pr0d1r2` cache.
const CI_CONFIG: &str = "extra-substituters = https://pr0d1r2.cachix.org\n\
    extra-trusted-public-keys = pr0d1r2.cachix.org-1:\
    NfWjbhgAj41byXhCKiaE+av3Vnphm1fTezHXEGsiQIM=\n";

#[test]
fn a_known_cache_gives_its_key_and_config() -> Result<(), KindError> {
    let cache = Cache::parse(3, "pr0d1r2")?;
    assert_eq!(cache.name(), "pr0d1r2");
    assert_eq!(
        cache.key(),
        "pr0d1r2.cachix.org-1:NfWjbhgAj41byXhCKiaE+av3Vnphm1fTezHXEGsiQIM="
    );
    assert_eq!(cache.nix_config(), CI_CONFIG);
    Ok(())
}

#[test]
fn an_unknown_cache_is_refused_naming_it() {
    for name in ["nix-community", "pr0d1r", "pr0d1r2.cachix.org-1:"] {
        let error = Cache::parse(3, name).err().map(|e| e.to_string());
        let want = format!(
            "line 3: cache `{name}` has no known public key (use one of: \
             pr0d1r2, or `-` for no cache)"
        );
        assert_eq!(error, Some(want));
    }
}

/// The caches of a `## caches` section holding `rows`.
fn caches(rows: &str) -> Result<Caches, String> {
    let text = format!("format 1\n\n## caches\nname|kind|url|key|push\n{rows}");
    let schema = Schema {
        sections: &[CACHES],
    };
    let doc = parse(".lablith", &text, &schema).map_err(|e| e.to_string())?;
    Caches::from_document(".lablith", &doc).map_err(|e| e.to_string())
}

#[test]
fn a_caches_row_is_only_a_binary_cache() -> Result<(), String> {
    let found = caches("hidden|https|secret:U|secret:K|-\n")?;
    let (old, new) = resolve(9, "hidden", &found).map_err(|e| e.to_string())?;
    assert_eq!(old, None);
    assert_eq!(new.as_ref().map(BinaryCache::name), Some("hidden"));
    Ok(())
}

#[test]
fn a_builtin_cache_is_both_types() -> Result<(), String> {
    let found = caches("hidden|https|secret:U|secret:K|-\n")?;
    let (old, new) =
        resolve(9, "pr0d1r2", &found).map_err(|e| e.to_string())?;
    assert_eq!(
        old,
        Some(Cache::parse(9, "pr0d1r2").map_err(|e| e.to_string())?)
    );
    assert_eq!(new, builtin("pr0d1r2"));
    Ok(())
}

#[test]
fn a_row_shadowing_a_builtin_leaves_the_old_type_empty() -> Result<(), String> {
    let found = caches("pr0d1r2|cachix|-|-|-\n")?;
    let (old, new) =
        resolve(9, "pr0d1r2", &found).map_err(|e| e.to_string())?;
    assert_eq!(old, None);
    assert_eq!(new.map(|c| c.push().clone()), Some(Push::ReadOnly));
    Ok(())
}

#[test]
fn a_missing_cache_names_the_caches_fix() -> Result<(), String> {
    let found = caches("hidden|https|secret:U|secret:K|-\n")?;
    let error = resolve(9, "other", &found).err().map(|e| e.to_string());
    assert_eq!(
        error.as_deref(),
        Some(
            "line 9: cache `other` is in neither `## caches` nor the \
             built-in table (add a `## caches` row named `other`, use one \
             of: hidden, pr0d1r2, or `-` for no cache)"
        )
    );
    Ok(())
}

#[test]
fn without_rows_the_error_is_the_old_one() {
    let error = resolve(9, "other", &Caches::default()).err();
    assert_eq!(error, Cache::parse(9, "other").err());
}
