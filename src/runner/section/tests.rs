//! The `## runners` section: rows, labels, refusals, the forge default
//! and the trust guard (T37, V80, V81).

use crate::registry::{Schema, parse};
use crate::runner::{COLUMNS, RUNNERS, Runner, Runners, SECTION};
use crate::system::{ALL, System, SystemError};

const FILE: &str = ".lablith";

/// The runners of a registry file with `rows` under the runners header.
fn runners(rows: &str) -> Result<Runners, String> {
    let text = format!("format 1\n\n## runners\nsystem|labels\n{rows}");
    let schema = Schema {
        sections: &[RUNNERS],
    };
    let doc = parse(FILE, &text, &schema).map_err(|e| e.to_string())?;
    Runners::from_document(FILE, &doc).map_err(|e| e.to_string())
}

fn refused(rows: &str) -> String {
    runners(rows).err().unwrap_or_default()
}

fn labels(names: &[&str]) -> Vec<String> {
    names.iter().map(|&n| n.to_owned()).collect()
}

#[test]
fn the_section_is_runners() {
    assert_eq!(SECTION, "runners");
    assert_eq!(COLUMNS, ["system", "labels"]);
    assert_eq!(RUNNERS.name, SECTION);
    assert_eq!(RUNNERS.header(), "system|labels");
    assert_eq!((RUNNERS.required, RUNNERS.key), (false, Some(0)));
}

#[test]
fn a_row_names_the_runner_over_the_default() -> Result<(), String> {
    let found = runners("x86_64-darwin|self-hosted, macOS, X64\n")?;
    let want = labels(&["self-hosted", "macOS", "X64"]);
    for has_default in [true, false] {
        let runner = found.pick(System::X86_64Darwin, has_default);
        assert_eq!(runner, Ok(Runner::Labels(&want)), "{has_default}");
    }
    Ok(())
}

#[test]
fn a_system_without_a_row_takes_the_forge_default() -> Result<(), String> {
    let found = runners("x86_64-darwin|self-hosted\n")?;
    let runner = found.pick(System::Aarch64Linux, true);
    assert_eq!(runner, Ok(Runner::Default));
    Ok(())
}

#[test]
fn neither_row_nor_default_is_no_runner() -> Result<(), String> {
    let found = runners("aarch64-linux|own-arm\n")?;
    let runner = found.pick(System::X86_64Darwin, false);
    assert_eq!(runner, Err(SystemError::NoRunner(System::X86_64Darwin)));
    Ok(())
}

#[test]
fn a_file_without_the_section_has_no_rows() -> Result<(), String> {
    let schema = Schema {
        sections: &[RUNNERS],
    };
    let doc = parse(FILE, "format 1\n", &schema).map_err(|e| e.to_string())?;
    let found =
        Runners::from_document(FILE, &doc).map_err(|e| e.to_string())?;
    assert_eq!(found, Runners::default());
    assert_eq!(found.pick(System::X86_64Darwin, true), Ok(Runner::Default));
    Ok(())
}

#[test]
fn every_system_may_have_a_row() -> Result<(), String> {
    let found = runners(
        "x86_64-linux|a\naarch64-linux|b\naarch64-darwin|c\nx86_64-darwin|d\n",
    )?;
    let names = ["a", "b", "c", "d"].map(|n| labels(&[n]));
    for (system, want) in ALL.iter().zip(&names) {
        let runner = found.pick(*system, false);
        assert_eq!(runner, Ok(Runner::Labels(want)), "{system}");
    }
    Ok(())
}

#[test]
fn an_unknown_system_is_refused_where_it_stands() {
    assert_eq!(
        refused("riscv64-linux|own\n"),
        ".lablith:5:1: unknown system `riscv64-linux`; known: x86_64-linux, \
         aarch64-linux, aarch64-darwin, x86_64-darwin (write a nix system \
         from the list)"
    );
}

#[test]
fn a_repeated_system_is_refused() {
    let error = refused("x86_64-darwin|a\nx86_64-darwin|b\n");
    assert!(error.starts_with(".lablith:6:1: "), "{error}");
    assert!(error.contains("x86_64-darwin"), "{error}");
}

#[test]
fn empty_labels_are_refused() {
    assert_eq!(
        refused("x86_64-darwin|-\n"),
        ".lablith:5:15: `labels` is empty (fill `labels`; `-` is only for \
         optional cells)"
    );
}

#[test]
fn a_label_that_is_not_a_plain_word_is_refused() {
    for label in ["mac os", "-x", "mac$os", "ma,c\\|os"] {
        let error = refused(&format!("x86_64-darwin|self-hosted, {label}\n"));
        assert!(error.starts_with(".lablith:5:15: label `"), "{error}");
        assert!(
            error.ends_with(
                "is not a runner label (use letters, digits, `.`, `_`, \
                 `-`; no leading `-`)"
            ),
            "{error}"
        );
    }
}

#[test]
fn an_empty_label_is_refused_by_the_list_reader() {
    let error = refused("x86_64-darwin|a,,b\n");
    assert!(error.contains("`labels` has an empty item"), "{error}");
}

#[test]
fn own_runners_are_trusted_only() {
    let own = labels(&["self-hosted"]);
    assert!(Runner::Labels(&own).trusted_only());
    assert!(!Runner::Default.trusted_only());
}
