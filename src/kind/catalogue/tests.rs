//! The catalogue: `docs/KINDS.md` parity (`.:V11`), actlith's values
//! (V34), secret purposes (V32) (T14).

use crate::kind::{
    ArgDefault, ArgSpec, Args, CATALOGUE, Cache, KindSpec, Purpose, Row,
    TIMEOUT, lookup,
};
use crate::system::{DEFAULT_SYSTEMS, System};

const KINDS_MD: &str = include_str!("../../../docs/KINDS.md");

/// The kinds `docs/KINDS.md` documents: its ``## `name` `` headings.
fn documented() -> Vec<&'static str> {
    KINDS_MD
        .lines()
        .filter_map(|l| l.strip_prefix("## `"))
        .filter_map(|l| l.strip_suffix('`'))
        .collect()
}

fn kind(name: &str) -> Result<&'static KindSpec, String> {
    lookup(name).ok_or(format!("no kind `{name}`"))
}

/// `(key, required, default)` of each arg, in schema order.
fn schema(name: &str) -> Result<Vec<(&str, bool, ArgDefault)>, String> {
    let spec = kind(name)?;
    let row = |a: &ArgSpec| (a.key, a.required, a.default);
    Ok(spec.schema.iter().map(row).collect())
}

fn row(cache: Option<Cache>) -> Row {
    Row {
        line: 4,
        id: "x".to_owned(),
        systems: Vec::new(),
        cache,
        args: Args::default(),
    }
}

#[test]
fn the_catalogue_is_the_documented_kinds_in_order() {
    let names: Vec<&str> = CATALOGUE.iter().map(|k| k.name).collect();
    assert_eq!(documented(), ["gate", "build", "update-pins", "upstream"]);
    assert_eq!(names, documented());
}

#[test]
fn lookup_finds_each_kind_and_nothing_else() {
    for spec in CATALOGUE {
        assert_eq!(lookup(spec.name), Some(spec));
    }
    assert_eq!(lookup("deploy"), None);
    assert_eq!(lookup("Gate"), None);
}

#[test]
fn every_kind_takes_timeout_last() {
    assert_eq!(TIMEOUT.default, ArgDefault::Value("30"));
    for spec in CATALOGUE {
        assert_eq!(spec.schema.last(), Some(&TIMEOUT), "{}", spec.name);
    }
}

#[test]
fn gate_and_build_take_actlith_args() -> Result<(), String> {
    let command = "nix develop --command hk check --all --check";
    let gate = vec![
        ("command", false, ArgDefault::Value(command)),
        ("timeout", false, ArgDefault::Value("30")),
    ];
    assert_eq!(schema("gate")?, gate);
    let build = vec![
        ("package", true, ArgDefault::None),
        ("timeout", false, ArgDefault::Value("30")),
    ];
    assert_eq!(schema("build")?, build);
    Ok(())
}

#[test]
fn update_pins_takes_actlith_args_and_a_forge_token() -> Result<(), String> {
    let want = vec![
        ("cron", false, ArgDefault::Value("50 6 * * *")),
        ("build", false, ArgDefault::Value(".#default")),
        ("token", false, ArgDefault::Forge),
        ("timeout", false, ArgDefault::Value("30")),
    ];
    assert_eq!(schema("update-pins")?, want);
    Ok(())
}

#[test]
fn upstream_takes_actlith_args_and_a_forge_token() -> Result<(), String> {
    let want = vec![
        ("script", true, ArgDefault::None),
        ("cron", false, ArgDefault::Value("20 6 * * *")),
        ("branch-prefix", false, ArgDefault::Value("upstream/")),
        ("token", false, ArgDefault::Forge),
        ("timeout", false, ArgDefault::Value("30")),
    ];
    assert_eq!(schema("upstream")?, want);
    Ok(())
}

#[test]
fn default_systems_are_actlith_ones() -> Result<(), String> {
    assert_eq!(kind("gate")?.default_systems, DEFAULT_SYSTEMS);
    assert_eq!(kind("build")?.default_systems, DEFAULT_SYSTEMS);
    let one = [System::X86_64Linux];
    assert_eq!(kind("update-pins")?.default_systems, one);
    assert_eq!(kind("upstream")?.default_systems, one);
    Ok(())
}

#[test]
fn each_kind_declares_its_secret_purposes() -> Result<(), String> {
    let push = [Purpose::CachePush];
    let open = [Purpose::OpenChange];
    assert_eq!(kind("gate")?.secrets, push);
    assert_eq!(kind("build")?.secrets, push);
    assert_eq!(kind("update-pins")?.secrets, open);
    assert_eq!(kind("upstream")?.secrets, open);
    Ok(())
}

#[test]
fn a_cache_push_is_needed_only_with_a_cache() -> Result<(), String> {
    let cache = Cache::parse(4, "pr0d1r2").map_err(|e| e.to_string())?;
    let cached = row(Some(cache));
    assert_eq!(kind("gate")?.purposes(&cached), [Purpose::CachePush]);
    assert_eq!(kind("gate")?.purposes(&row(None)), []);
    assert_eq!(kind("upstream")?.purposes(&cached), [Purpose::OpenChange]);
    assert_eq!(
        kind("update-pins")?.purposes(&row(None)),
        [Purpose::OpenChange]
    );
    Ok(())
}
