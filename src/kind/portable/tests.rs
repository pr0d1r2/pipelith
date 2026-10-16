//! Forge extras and the portability check: forge-only kind, forge-only
//! key, clean row (T15, V33).

use crate::kind::{
    ArgDefault, ArgSpec, Extras, Finding, KindSpec, RawRow, TIMEOUT, portable,
    resolve,
};
use crate::registry::Arg;

/// A forge-only kind.
const PAGES: KindSpec = KindSpec {
    name: "pages",
    schema: &[TIMEOUT],
    default_systems: &[],
    secrets: &[],
};

/// A forge-only key, plus `timeout`, which every kind already takes.
const KEYS: [ArgSpec; 2] = [
    ArgSpec {
        key: "runner",
        required: false,
        default: ArgDefault::None,
    },
    TIMEOUT,
];

const EXTRAS: Extras<'static> = Extras {
    kinds: &[PAGES],
    keys: &KEYS,
};

fn row(kind: &str, keys: &[&str]) -> RawRow {
    let arg = |k: &&str| Arg {
        key: (*k).to_owned(),
        value: "1".to_owned(),
    };
    RawRow {
        line: 6,
        id: "ci".to_owned(),
        kind: kind.to_owned(),
        args: keys.iter().map(arg).collect(),
        ..RawRow::default()
    }
}

fn kind(line: usize, kind: &str) -> Finding {
    Finding::Kind {
        line,
        kind: kind.to_owned(),
    }
}

fn key(line: usize, key: &str) -> Finding {
    Finding::Key {
        line,
        key: key.to_owned(),
    }
}

#[test]
fn resolve_finds_catalogue_then_forge_kinds() {
    let gate = resolve(&row("gate", &[]), &EXTRAS).map(|k| k.name);
    assert_eq!(gate, Ok("gate"));
    assert_eq!(resolve(&row("pages", &[]), &EXTRAS), Ok(&PAGES));
    assert_eq!(EXTRAS.kind("pages"), Some(&PAGES));
    assert_eq!(EXTRAS.kind("gate"), None);
}

#[test]
fn an_unknown_kind_is_refused_listing_the_kinds() {
    let error =
        resolve(&row("deploy", &[]), &EXTRAS).map_err(|e| e.to_string());
    assert_eq!(
        error,
        Err(
            "line 6: unknown kind `deploy` (use one of `gate`, `build`, \
             `update-pins`, `upstream`, `pages`)"
                .to_owned()
        )
    );
    let bare = resolve(&row("pages", &[]), &Extras::default()).err();
    assert_eq!(bare.map(|e| e.line), Some(6));
}

#[test]
fn a_clean_row_is_portable() {
    assert_eq!(portable(&row("gate", &["timeout"]), &EXTRAS), []);
    assert_eq!(
        portable(&row("build", &["package"]), &Extras::default()),
        []
    );
}

#[test]
fn a_forge_only_kind_is_one_finding() {
    let findings = portable(&row("pages", &["timeout"]), &EXTRAS);
    assert_eq!(findings, [kind(6, "pages")]);
}

#[test]
fn a_forge_only_key_is_one_finding_each() {
    let findings = portable(&row("gate", &["runner", "timeout"]), &EXTRAS);
    assert_eq!(findings, [key(6, "runner")]);
    let both = portable(&row("pages", &["runner"]), &EXTRAS);
    assert_eq!(both, [kind(6, "pages"), key(6, "runner")]);
}

#[test]
fn unknown_kinds_and_keys_are_errors_not_findings() {
    assert_eq!(portable(&row("deploy", &["colour"]), &EXTRAS), []);
}

#[test]
fn a_finding_names_line_and_what() {
    assert_eq!(
        kind(6, "pages").to_string(),
        "line 6: kind `pages` is forge-only; the other forge refuses this row"
    );
    assert_eq!(
        key(7, "runner").to_string(),
        "line 7: arg `runner` is forge-only; the other forge refuses this row"
    );
}
