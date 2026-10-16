//! Cache name to public key and nix config (T14, V53).

use crate::kind::{Cache, KindError};

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
