//! A row checked for a kind: default and listed systems, cache, args
//! (T14, V31, V52, V53); a cache from `## caches` (T40).

use crate::cache::{CACHES, CacheKind, Caches, builtin};
use crate::kind::{
    Cache, Checked, Extras, KindSpec, RawRow, Row, TIMEOUT, WORKFLOWS, lookup,
};
use crate::registry::{Schema, parse};
use crate::system::System;

const FILE: &str = ".lablith";

const ROWS: &str = "format 1

# Rows checked against a probe kind, one shape or refusal per row.

## workflows
id|kind|systems|cache|args
plain|probe|-|-|-
picked|probe|aarch64-darwin, x86_64-linux|pr0d1r2|timeout=45
intel|probe|x86_64-darwin|-|-
riscv|probe|riscv64-linux|-|-
twice|probe|x86_64-linux, x86_64-linux|-|-
stranger|probe|-|nix-community|-
extra|probe|-|-|colour=blue
slow|probe|-|-|timeout=400
";

/// A kind that takes `timeout` and defaults to one system.
const PROBE: KindSpec = KindSpec {
    name: "probe",
    schema: &[TIMEOUT],
    default_systems: &[System::Aarch64Linux],
    secrets: &[],
};

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
    Row::check(&PROBE, &row, &Extras::default()).map_err(|e| e.to_string())
}

fn refused(id: &str) -> String {
    checked(id).err().unwrap_or_default()
}

#[test]
fn a_dash_row_takes_the_kind_defaults() -> Result<(), String> {
    let row = checked("plain")?;
    assert_eq!((row.id.as_str(), row.line), ("plain", 7));
    assert_eq!(row.systems, [System::Aarch64Linux]);
    assert_eq!(row.cache, None);
    assert_eq!(row.args.get("timeout"), Some("30"));
    Ok(())
}

#[test]
fn listed_systems_keep_their_order() -> Result<(), String> {
    let row = checked("picked")?;
    assert_eq!(row.systems, [System::Aarch64Darwin, System::X86_64Linux]);
    let cache = Cache::parse(8, "pr0d1r2").map_err(|e| e.to_string())?;
    assert_eq!(row.cache, Some(cache));
    assert_eq!(row.args.get("timeout"), Some("45"));
    Ok(())
}

#[test]
fn intel_mac_is_a_ci_system() -> Result<(), String> {
    assert_eq!(checked("intel")?.systems, [System::X86_64Darwin]);
    Ok(())
}

#[test]
fn an_unknown_system_is_refused() {
    let error = refused("riscv");
    assert!(
        error.starts_with("line 10: unknown system `riscv64-linux`"),
        "{error}"
    );
}

#[test]
fn a_repeated_system_is_refused() {
    assert_eq!(
        refused("twice"),
        "line 11: system `x86_64-linux` is listed twice (keep one \
         `x86_64-linux`)"
    );
}

#[test]
fn an_unknown_cache_is_refused() {
    let error = refused("stranger");
    assert!(
        error.starts_with("line 12: cache `nix-community`"),
        "{error}"
    );
}

#[test]
fn an_unknown_arg_is_refused() {
    let error = refused("extra");
    assert!(
        error.starts_with("line 13: kind `probe` has no arg `colour`"),
        "{error}"
    );
}

#[test]
fn a_timeout_out_of_range_is_refused() {
    let error = refused("slow");
    assert!(error.starts_with("line 14: timeout `400`"), "{error}");
}

#[test]
fn a_catalogue_kind_checks_its_own_row() -> Result<(), String> {
    let gate = lookup("gate").ok_or("no gate")?;
    let row = Row::check(gate, &raw("plain")?, &Extras::default());
    let row = row.map_err(|e| e.to_string())?;
    assert_eq!(row.systems, gate.default_systems);
    let command = row.args.get("command");
    assert_eq!(
        command,
        Some("nix develop --command hk check --all --check")
    );
    Ok(())
}

const CACHED: &str = "format 1

## caches
name|kind|url|key|push
example-cache|https|https://cache.example.org|secret:CACHE_KEY|-

## workflows
id|kind|systems|cache|args
mirror|probe|-|example-cache|-
missing|probe|-|other-cache|-
builtin|probe|-|pr0d1r2|-
bare|probe|-|-|-
";

/// The row `id` of [`CACHED`], checked with its `## caches` rows.
fn checked_with(id: &str) -> Result<Checked, String> {
    let schema = Schema {
        sections: &[CACHES, WORKFLOWS],
    };
    let doc = parse(FILE, CACHED, &schema).map_err(|e| e.to_string())?;
    let caches =
        Caches::from_document(FILE, &doc).map_err(|e| e.to_string())?;
    let row = doc
        .section("workflows")
        .iter()
        .find(|r| r.cells.first().map(|c| &*c.text) == Some(id));
    let row = RawRow::from_cells(FILE, row.ok_or("no such row")?);
    let row = row.map_err(|e| e.to_string())?;
    Row::check_with(&PROBE, &row, &Extras::default(), &caches)
        .map_err(|e| e.to_string())
}

#[test]
fn a_row_names_a_caches_row() -> Result<(), String> {
    let checked = checked_with("mirror")?;
    assert_eq!(checked.row.cache, None);
    let cache = checked.cache.ok_or("no cache")?;
    assert_eq!(cache.name(), "example-cache");
    assert_eq!(cache.kind(), CacheKind::Https);
    Ok(())
}

#[test]
fn a_row_naming_a_missing_cache_is_refused() {
    assert_eq!(
        checked_with("missing").err().as_deref(),
        Some(
            "line 10: cache `other-cache` is in neither `## caches` nor the \
             built-in table (add a `## caches` row named `other-cache`, use \
             one of: example-cache, pr0d1r2, or `-` for no cache)"
        )
    );
}

#[test]
fn the_builtin_cache_still_works_beside_rows() -> Result<(), String> {
    let checked = checked_with("builtin")?;
    let old = Cache::parse(11, "pr0d1r2").map_err(|e| e.to_string())?;
    assert_eq!(checked.row.cache, Some(old));
    assert_eq!(checked.cache, builtin("pr0d1r2"));
    Ok(())
}

#[test]
fn a_dash_cache_trusts_no_cache_beside_rows() -> Result<(), String> {
    let checked = checked_with("bare")?;
    assert_eq!((checked.row.cache, checked.cache), (None, None));
    Ok(())
}

#[test]
fn check_is_check_with_and_no_caches() -> Result<(), String> {
    let row = raw("picked")?;
    let extras = Extras::default();
    let old = Row::check(&PROBE, &row, &extras).map_err(|e| e.to_string())?;
    let new = Row::check_with(&PROBE, &row, &extras, &Caches::default());
    let new = new.map_err(|e| e.to_string())?;
    assert_eq!(new.row, old);
    assert_eq!(new.cache, builtin("pr0d1r2"));
    Ok(())
}
