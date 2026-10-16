use super::{Cargo, Facts, render, stale_prose};
use crate::facts::Nixpkgs;

fn nixpkgs() -> Nixpkgs {
    let (series, rev, day) = ("26.05", "f5c082a", "2026-09-25");
    Nixpkgs {
        series: series.into(),
        rev: rev.into(),
        day: day.into(),
    }
}

fn cargo() -> Cargo {
    Cargo {
        slug: "o/r".into(),
        license: "MIT".into(),
        edition: "2024".into(),
        msrv: "1.95".into(),
        dependencies: 0,
    }
}

fn facts() -> Facts {
    Facts {
        cargo: cargo(),
        floor: "97.74".into(),
        steps: (39, 41),
        spec: (28, 2, 1),
        nixpkgs: nixpkgs(),
        platforms: vec![("arm", "macos")],
    }
}

/// Alt text and URL come from the same value, escaped the shields.io way.
#[test]
fn the_block_renders_every_group() {
    let block = render(&facts());
    for want in [
        "[![CI](https://github.com/o/r/actions/workflows/ci.yml/badge.svg)](https://github.com/o/r/actions/workflows/ci.yml)\n",
        "[![direct dependencies 0](https://img.shields.io/badge/direct_dependencies-0-brightgreen)](docs/THIRD-PARTY-NOTICES.md)\n",
        "[![gate steps 39 commit / 41 push](https://img.shields.io/badge/gate_steps-39_commit_%2F_41_push-6E4AFF)](hk.pkl)\n",
        "[![coverage floor 97.74%](https://img.shields.io/badge/coverage_floor-%E2%89%A597.74%25-brightgreen)](.coverage)\n",
        "[![invariants 28](https://img.shields.io/badge/invariants-28-6E4AFF)](SPEC.md)\n",
        "[![nixpkgs 26.05 (2026-09-25 - f5c082a)](https://img.shields.io/badge/nixpkgs-26.05_(2026--09--25_--_f5c082a)-5277C3?logo=nixos&logoColor=white)](flake.lock)\n",
        "[![arm macos](https://img.shields.io/badge/macos-5277C3?logo=arm&logoColor=white)](.github/workflows/ci.yml)\n\n[![built with Claude Code]",
        "[![built with SDD](https://img.shields.io/badge/built_with-spec--driven_development-D97757)](SPEC.md)\n<!-- END badges -->\n",
    ] {
        assert!(block.contains(want), "missing {want}");
    }
}

/// Unpublished: no badge points at crates.io or docs.rs (T2).
#[test]
fn no_badge_claims_a_publication() {
    let block = render(&facts());
    assert!(!block.contains("crates.io") && !block.contains("docs.rs"));
    assert!(!block.contains('{'), "{block}");
}

/// An empty or zero fact refuses the render, naming it.
#[test]
fn an_empty_fact_is_refused() {
    assert!(facts().checked().is_ok());
    let empty = Facts {
        steps: (0, 41),
        ..facts()
    };
    assert_eq!(
        empty.checked().err().as_deref(),
        Some("hk.pkl steps read empty or zero; fix the source")
    );
}

/// A Cargo.toml fact that reads empty is refused the same way.
#[test]
fn an_empty_cargo_fact_is_refused() {
    let slug = String::new();
    let cargo = Cargo { slug, ..cargo() };
    let why = Facts { cargo, ..facts() }.checked().err();
    let why = why.unwrap_or_default();
    assert!(why.starts_with("Cargo.toml repository"), "{why}");
}

/// The disclaimer's numbers must be the facts', across wrapped lines.
#[test]
fn stale_prose_is_named() {
    let good = "**28\ninvariants** and **2 bugs**, a floor of 97.74% here";
    assert!(stale_prose(good, &facts()).is_empty());
    let stale = stale_prose(&good.replace("28", "9"), &facts());
    assert_eq!(stale, ["**28 invariants**"]);
}
