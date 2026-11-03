//! actlith's build-time row rules, checked by `Row::check` (T32, V75,
//! V76): `build` has a cache, `package` is a flake attribute name,
//! `cron` is five fields, `token` is a secret name, a bump kind runs on
//! one system, `build`, `script` and `branch-prefix` are plain words;
//! `build`'s cache takes a push (T40).

use super::{attribute, five_fields, plain_word, secret_name};
use crate::cache::{CACHES, Caches};
use crate::kind::{Checked, Extras, RawRow, Row, WORKFLOWS, resolve};
use crate::registry::{Schema, parse};

const FILE: &str = ".lablith";

const ROWS: &str = "format 1

# Rows the catalogue kinds check, one pass or refusal per row.

## workflows
id|kind|systems|cache|args
fine|build|-|pr0d1r2|package=nix-hk_2
nocache|build|-|-|package=hk
spliced|build|-|pr0d1r2|package=a;b
digit|build|-|pr0d1r2|package=1x
gate|gate|-|-|-
pins|update-pins|-|-|-
named|update-pins|-|-|token=APP_TOKEN
dashed|update-pins|-|-|token=A-B
names|update-pins|-|-|cron=*/15 6-8 1 JAN MON-FRI
short|update-pins|-|-|cron=50 6 * *
long|upstream|-|-|script=bump.sh, cron=0 50 6 * * *
odd|upstream|-|-|script=bump.sh, cron=50 6 * * ?
two|upstream|x86_64-linux, aarch64-linux|-|script=bump.sh
target|update-pins|-|-|build=.#hk
path|upstream|-|-|script=scripts/bump-hk.sh, branch-prefix=up/v#1+x:y@z_
spaced|update-pins|-|-|build=a b
flag|upstream|-|-|script=-rf
subst|upstream|-|-|script=bump.sh, branch-prefix=$(x)
";

fn raw(id: &str) -> Result<RawRow, String> {
    let schema = Schema {
        sections: &[WORKFLOWS],
    };
    let doc = parse(FILE, ROWS, &schema).map_err(|e| e.to_string())?;
    let rows = doc.section("workflows");
    let row = rows
        .iter()
        .find(|r| r.cells.first().map(|c| &*c.text) == Some(id));
    let row = row.ok_or("no such row")?;
    RawRow::from_cells(FILE, row).map_err(|e| e.to_string())
}

fn checked(id: &str) -> Result<Row, String> {
    let row = raw(id)?;
    let extras = Extras::default();
    let kind = resolve(&row, &extras).map_err(|e| e.to_string())?;
    Row::check(kind, &row, &extras).map_err(|e| e.to_string())
}

fn refused(id: &str) -> String {
    checked(id).err().unwrap_or_default()
}

#[test]
fn rows_inside_the_rules_pass() -> Result<(), String> {
    for id in ["fine", "gate", "pins", "named", "names", "target", "path"] {
        checked(id)?;
    }
    Ok(())
}

#[test]
fn a_build_row_without_a_cache_is_refused() {
    assert_eq!(
        refused("nocache"),
        "line 8: kind `build` needs a cache: it pushes there and \
         verify-cache asks it for every built path (name a cachix cache \
         in the cache column)"
    );
}

#[test]
fn a_package_that_is_no_attribute_name_is_refused() {
    assert_eq!(
        refused("spliced"),
        "line 9: package `a;b` is not a flake attribute name (use \
         letters, digits, `_` and `-`, starting with a letter or `_`)"
    );
    let error = refused("digit");
    assert!(error.starts_with("line 10: package `1x`"), "{error}");
}

#[test]
fn a_token_that_is_no_secret_name_is_refused() {
    assert_eq!(
        refused("dashed"),
        "line 14: token `A-B` is not a secret name (write \
         `token=<SECRET>`: letters, digits and `_`, not starting with a \
         digit)"
    );
}

#[test]
fn four_or_six_cron_fields_are_refused() {
    assert_eq!(
        refused("short"),
        "line 16: cron `50 6 * *` is not five cron fields (write \
         `cron=<minute> <hour> <day> <month> <weekday>`, e.g. \
         `cron=50 6 * * *`)"
    );
    let error = refused("long");
    assert!(error.starts_with("line 17: cron `0 50 6 * * *`"), "{error}");
}

#[test]
fn a_cron_field_with_another_character_is_refused() {
    let error = refused("odd");
    assert!(error.starts_with("line 18: cron `50 6 * * ?`"), "{error}");
}

#[test]
fn a_bump_kind_on_two_systems_is_refused() {
    assert_eq!(
        refused("two"),
        "line 19: a bump kind runs one job, 2 systems given (list one \
         system, or `-` for x86_64-linux)"
    );
}

#[test]
fn only_attribute_names_that_are_shell_words_pass() {
    for name in ["hk", "nix-hk", "_x1", "a-b_c9"] {
        assert!(attribute(name), "{name}");
    }
    for name in ["", "1x", "-x", "a b", "a;b", "$(x)", "a'b", "a.b", "a\"b"] {
        assert!(!attribute(name), "{name}");
    }
}

#[test]
fn only_secret_names_pass() {
    for name in ["CACHIX_AUTH_TOKEN", "_X", "bot_token2"] {
        assert!(secret_name(name), "{name}");
    }
    for name in ["", "2X", "A-B", "A }} B", "A.B", "A B"] {
        assert!(!secret_name(name), "{name}");
    }
}

#[test]
fn five_fields_of_numbers_names_ranges_and_steps_pass() {
    for cron in ["50 6 * * *", "*/15 6-8 1 JAN MON-FRI", "0  5   * * 1"] {
        assert!(five_fields(cron), "{cron}");
    }
    for cron in ["", "50 6 * *", "0 50 6 * * *", "50 6 * * ?", "a,b * * * *"] {
        assert!(!five_fields(cron), "{cron}");
    }
}

#[test]
fn a_build_that_is_no_plain_word_is_refused() {
    assert_eq!(
        refused("spaced"),
        "line 22: arg `build` is `a b`, not a plain word (write `build=` \
         with letters, digits and `._/#+:@-`)"
    );
}

#[test]
fn a_script_or_prefix_that_is_no_plain_word_is_refused() {
    let error = refused("flag");
    let want = "line 23: arg `script` is `-rf`, not a plain word";
    assert!(error.starts_with(want), "{error}");
    let error = refused("subst");
    let want = "line 24: arg `branch-prefix` is `$(x)`, not a plain word";
    assert!(error.starts_with(want), "{error}");
}

#[test]
fn only_plain_words_pass() {
    for word in [".#hk", "scripts/bump-hk.sh", "upstream/", "a+b:c@d#e"] {
        assert!(plain_word(word), "{word}");
    }
    for word in ["", "-", "-rf", "a b", "$(x)", "a;b", "a'b", "a\"b", "a,b"] {
        assert!(!plain_word(word), "{word}");
    }
}

/// The row `id` of `text`, checked with its `## caches` rows.
fn checked_with_caches(text: &str, id: &str) -> Result<Checked, String> {
    let schema = Schema {
        sections: &[CACHES, WORKFLOWS],
    };
    let doc = parse(FILE, text, &schema).map_err(|e| e.to_string())?;
    let caches =
        Caches::from_document(FILE, &doc).map_err(|e| e.to_string())?;
    let rows = doc.section("workflows");
    let row = rows
        .iter()
        .find(|r| r.cells.first().map(|c| &*c.text) == Some(id));
    let row = RawRow::from_cells(FILE, row.ok_or("no such row")?);
    let row = row.map_err(|e| e.to_string())?;
    let extras = Extras::default();
    let kind = resolve(&row, &extras).map_err(|e| e.to_string())?;
    Row::check_with(kind, &row, &extras, &caches).map_err(|e| e.to_string())
}

const CACHED: &str = "format 1

## caches
name|kind|url|key|push
mirror|https|https://cache.example.org|secret:CACHE_KEY|-
pushy|https|secret:CACHE_URL|secret:CACHE_KEY|secret:CACHE_NETRC

## workflows
id|kind|systems|cache|args
ro|build|-|mirror|package=hk
rw|build|-|pushy|package=hk
";

#[test]
fn a_build_row_with_a_read_only_cache_is_refused() {
    let error = checked_with_caches(CACHED, "ro").err();
    assert_eq!(
        error.as_deref(),
        Some(
            "line 10: kind `build` pushes to its cache, and cache `mirror` \
             is read-only (give its `## caches` row a `push` secret)"
        )
    );
}

#[test]
fn a_build_row_with_a_push_secret_passes() -> Result<(), String> {
    let checked = checked_with_caches(CACHED, "rw")?;
    assert_eq!(checked.row.cache, None);
    let cache = checked.cache.ok_or("no cache")?;
    assert_eq!(cache.name(), "pushy");
    assert!(cache.can_push());
    Ok(())
}
