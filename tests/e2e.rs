//! End to end over the public API, as actlith will use it (tests V39):
//! actlith's own `.actlith` @ `7ce69c6` is parsed by `registry` against a
//! schema holding `kind::WORKFLOWS`, its `ci` row is checked by `kind`,
//! a GitHub-shaped tree is built here from the row's values and written
//! by `yaml` under `files::header`; the bytes must equal the `ci.yml`
//! actlith rendered from that row (tests V38, root V3). Everything GitHub
//! (runner labels, pins, secret names, expression syntax) is test-local.

use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use pipelith::files::{self, File, Stale};
use pipelith::kind::{self, Cache, Extras, Purpose, RawRow, Row};
use pipelith::registry::{self, Document, Schema, SectionSpec};
use pipelith::system::System;
use pipelith::yaml::{Comment, Dialect, Entry, Expr, Node, Text, Value};

type Built<T> = Result<T, Box<dyn Error>>;

/// The config file the fixture stands for, as errors and the header name it.
const CONFIG: &str = ".actlith";

/// actlith's own registry @ `7ce69c6`, byte for byte.
const ACTLITH: &str = include_str!("fixtures/e2e/.actlith");

/// What actlith @ `7ce69c6` rendered from that registry's `ci` row:
/// `.github/workflows/ci.yml` (and its render fixture, the same bytes).
const GOLDEN: &str = "tests/golden/ci.yaml";

/// Where actlith writes the workflow of row `ci`.
const TARGET: &str = ".github/workflows/ci.yml";

/// actlith's forge-only sections, as its `registry::table` declares them.
const ACTIONS: SectionSpec<'static> = SectionSpec {
    name: "actions",
    columns: &["action", "version", "sha"],
    required: false,
    key: Some(0),
};

const HAND_WRITTEN: SectionSpec<'static> = SectionSpec {
    name: "hand-written",
    columns: &["path", "reason"],
    required: false,
    key: Some(0),
};

const BOTS: SectionSpec<'static> = SectionSpec {
    name: "bots",
    columns: &["bot", "args"],
    required: false,
    key: Some(0),
};

const SECTIONS: [SectionSpec<'static>; 4] =
    [kind::WORKFLOWS, ACTIONS, HAND_WRITTEN, BOTS];

/// GitHub-hosted runner labels, as actlith's `system::runner` maps them.
const RUNNERS: [(System, &str); 3] = [
    (System::X86_64Linux, "ubuntu-latest"),
    (System::Aarch64Linux, "ubuntu-24.04-arm"),
    (System::Aarch64Darwin, "macos-latest"),
];

/// actlith's pin table @ `7ce69c6`: action, commit, version.
const PINS: [(&str, &str, &str); 3] = [
    (
        "actions/checkout",
        "3d3c42e5aac5ba805825da76410c181273ba90b1",
        "v7.0.1",
    ),
    (
        "cachix/install-nix-action",
        "630ae543ea3a38a9a4166f03376c02c50f408342",
        "v31.11.0",
    ),
    (
        "cachix/cachix-action",
        "5f2d7c5294214f71b873db4b969586b980625e71",
        "v17",
    ),
];

/// The GitHub secret actlith reads for the purpose the gate needs.
const SECRETS: [(Purpose, &str); 1] =
    [(Purpose::CachePush, "secrets.CACHIX_AUTH_TOKEN")];

const DEPTH_WHY: &str = "Full history: tdd-order reads the push range, and \
    a later semver\nstep compares with the last release tag.";

const PUSH_WHY: &str = "Push what this job builds, so the next run \
    downloads it. The flake\nmakes the fleet tools follow nixpkgs-lock; \
    when that pin is newer\nthan theirs, their own caches never match and \
    every runner builds\nthem from source. Fork PRs get no secret and still \
    read the cache.";

const FILTER_WHY: &str =
    "Only what was built here; sources and tarballs stay out of the cache.";

const PUSH_FILTER: &str = "(-source$|nixpkgs\\.tar\\.gz$)";

const SKIP_FORK_PR: &str = "github.event_name == 'pull_request' \
    && github.event.pull_request.head.repo.full_name != github.repository";

/// GitHub Actions expressions, spelled as actlith's `text::Expr` did.
struct Github;

impl Dialect for Github {
    fn expr(&self, inner: &str) -> Option<String> {
        let broken = inner.contains("${{") || inner.contains("}}");
        (!broken).then(|| format!("${{{{ {inner} }}}}"))
    }

    fn escape(&self, lit: &str) -> String {
        lit.replace("${{", "${{ '${{' }}")
    }
}

fn parse() -> Built<Document> {
    let schema = Schema {
        sections: &SECTIONS,
    };
    Ok(registry::parse(CONFIG, ACTLITH, &schema)?)
}

/// The `## workflows` row `id`, as a neutral raw row.
fn raw(doc: &Document, id: &str) -> Built<RawRow> {
    for cells in doc.section(kind::SECTION) {
        let row = RawRow::from_cells(CONFIG, cells)?;
        if row.id == id {
            return Ok(row);
        }
    }
    Err(format!("no `{id}` row").into())
}

/// The row `ci`, checked against the neutral catalogue.
fn ci_row() -> Built<Row> {
    let raw = raw(&parse()?, "ci")?;
    let extras = Extras::default();
    let spec = kind::resolve(&raw, &extras)?;
    assert_eq!(spec.name, kind::GATE.name);
    assert!(kind::portable(&raw, &extras).is_empty());
    Ok(Row::check(spec, &raw, &extras)?)
}

fn expr(inner: &str) -> Built<Expr> {
    Expr::new(&Github, inner).ok_or_else(|| format!("bad `{inner}`").into())
}

fn text(s: &str) -> Node {
    Node::Value(Value::Text(Text::lit(s)))
}

fn ex(inner: &str) -> Built<Node> {
    Ok(Node::Value(Value::Text(Text::expr(expr(inner)?))))
}

fn flag(b: bool) -> Node {
    Node::Value(Value::Bool(b))
}

fn fixed(key: &'static str, node: Node) -> Entry {
    Entry::fixed(key, node)
}

/// `uses: <action>@<sha>  # <version>`, pinned from [`PINS`].
fn uses(action: &str) -> Built<Entry> {
    let pin = PINS.iter().find(|(a, _, _)| *a == action);
    let (_, sha, version) = pin.ok_or_else(|| format!("no pin `{action}`"))?;
    let entry = fixed("uses", text(&format!("{action}@{sha}")));
    Ok(entry.after(Comment::new(version)))
}

fn label(system: System) -> Built<Text> {
    let row = RUNNERS.iter().find(|(s, _)| *s == system);
    let (_, label) = row.ok_or_else(|| format!("no runner for {system}"))?;
    Ok(Text::lit(label))
}

fn secret(purpose: Purpose) -> Built<&'static str> {
    let row = SECRETS.iter().find(|(p, _)| *p == purpose);
    Ok(row.ok_or_else(|| format!("no secret for {purpose:?}"))?.1)
}

/// `fail-fast: false` over an `os` axis of the row's runner labels, the
/// systems named in a comment above it.
fn strategy(row: &Row) -> Built<Node> {
    let names: Vec<&str> = row.systems.iter().map(|s| s.name()).collect();
    let labels = row.systems.iter().map(|&s| label(s));
    let os = fixed("os", Node::List(labels.collect::<Built<_>>()?))
        .above(Comment::new(&names.join(", ")));
    Ok(Node::Map(vec![
        fixed("fail-fast", flag(false)),
        fixed("matrix", Node::Map(vec![os])),
    ]))
}

fn checkout() -> Built<Vec<Entry>> {
    let depth = fixed("fetch-depth", Node::Value(Value::Int(0)))
        .above(Comment::new(DEPTH_WHY));
    let with = vec![fixed("persist-credentials", flag(false)), depth];
    Ok(vec![
        uses("actions/checkout")?,
        fixed("with", Node::Map(with)),
    ])
}

fn install_nix(cache: &Cache) -> Built<Vec<Entry>> {
    let config = fixed("extra_nix_config", text(&cache.nix_config()));
    let with = fixed("with", Node::Map(vec![config]));
    Ok(vec![uses("cachix/install-nix-action")?, with])
}

fn cachix(cache: &Cache, token: &str) -> Built<Vec<Entry>> {
    let filter =
        fixed("pushFilter", text(PUSH_FILTER)).above(Comment::new(FILTER_WHY));
    let with = vec![
        fixed("name", text(cache.name())),
        fixed("authToken", ex(token)?),
        fixed("skipPush", ex(SKIP_FORK_PR)?),
        filter,
    ];
    let step = uses("cachix/cachix-action")?.above(Comment::new(PUSH_WHY));
    Ok(vec![step, fixed("with", Node::Map(with))])
}

/// Checkout, install-nix, cachix when the row has a cache, the gate.
fn steps(row: &Row) -> Built<Node> {
    let mut steps = vec![checkout()?];
    if let Some(cache) = &row.cache {
        steps.push(install_nix(cache)?);
        let purposes = kind::GATE.purposes(row);
        assert_eq!(purposes, [Purpose::CachePush]);
        steps.push(cachix(cache, secret(Purpose::CachePush)?)?);
    }
    let command = row.args.get("command").ok_or("no command")?;
    steps.push(vec![fixed("run", text(command))]);
    Ok(Node::Seq(steps))
}

fn job(row: &Row) -> Built<Node> {
    let timeout = u64::from(row.args.timeout()?);
    Ok(Node::Map(vec![
        fixed("name", text(kind::GATE.name)),
        fixed("runs-on", ex("matrix.os")?),
        fixed("strategy", strategy(row)?),
        fixed("timeout-minutes", Node::Value(Value::Int(timeout))),
        fixed("steps", steps(row)?),
    ]))
}

fn on() -> Node {
    let push = vec![fixed("branches", Node::List(vec![Text::lit("main")]))];
    Node::Map(vec![
        fixed("push", Node::Map(push)),
        fixed("pull_request", Node::Null),
        fixed("workflow_dispatch", Node::Null),
    ])
}

fn concurrency(id: &str) -> Built<Node> {
    let group = Text::lit(&format!("{id}-")).then_expr(expr("github.ref")?);
    Ok(Node::Map(vec![
        fixed("group", Node::Value(Value::Text(group))),
        fixed("cancel-in-progress", flag(true)),
    ]))
}

/// The gate workflow of `row` as the YAML tree actlith lowers it to.
fn workflow(row: &Row) -> Built<Vec<Entry>> {
    let read = vec![fixed("contents", text("read"))];
    let jobs = vec![Entry::user(kind::GATE.name, job(row)?)];
    Ok(vec![
        fixed("name", text(&row.id)),
        fixed("on", on()),
        fixed("concurrency", concurrency(&row.id)?),
        fixed("permissions", Node::Map(read)),
        fixed("jobs", Node::Map(jobs)),
    ])
}

/// The generated file: actlith's header, then the document.
fn render() -> Built<String> {
    let body = pipelith::yaml::document(&Github, &[], &workflow(&ci_row()?)?);
    Ok(format!("{}{body}", files::header("actlith", CONFIG)))
}

/// The golden's bytes; `UPDATE_GOLDEN=1` first rewrites it with `bytes`
/// (never by default: a golden changes only with a CHANGELOG entry, V38).
fn golden(bytes: &str) -> Built<String> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(GOLDEN);
    let update = std::env::var("UPDATE_GOLDEN").is_ok_and(|v| v == "1");
    update.then(|| fs::write(&path, bytes)).transpose()?;
    Ok(fs::read_to_string(path)?)
}

#[test]
fn actlith_ci_row_renders_actlith_golden_byte_for_byte() -> Built<()> {
    let bytes = render()?;
    assert_eq!(bytes, golden(&bytes)?);
    Ok(())
}

#[test]
fn ci_row_takes_the_gate_defaults_and_the_named_cache() -> Built<()> {
    let row = ci_row()?;
    let names: Vec<&str> = row.systems.iter().map(|s| s.name()).collect();
    assert_eq!(names, ["x86_64-linux", "aarch64-linux", "aarch64-darwin"]);
    assert_eq!(row.cache.as_ref().map(Cache::name), Some("pr0d1r2"));
    assert_eq!(row.args.timeout()?, kind::DEFAULT_TIMEOUT);
    Ok(())
}

#[test]
fn rendering_twice_gives_the_same_bytes() -> Built<()> {
    assert_eq!(render()?, render()?);
    Ok(())
}

/// A fresh, empty directory under the system temp dir for `name`.
fn scratch(name: &str) -> Built<PathBuf> {
    let dir = std::env::temp_dir()
        .join(format!("pipelith-e2e-{}-{name}", std::process::id()));
    if dir.exists() {
        fs::remove_dir_all(&dir)?;
    }
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

fn drift(root: &Path, file: &File) -> Built<Vec<Stale>> {
    let stale = files::check(root, std::slice::from_ref(file))?;
    Ok(stale)
}

#[test]
fn check_finds_the_written_file_fresh_and_the_edited_one_stale() -> Built<()> {
    let root = scratch("check")?;
    let file = File {
        path: TARGET.to_owned(),
        bytes: render()?,
    };
    assert_eq!(drift(&root, &file)?, [Stale::Missing(TARGET.to_owned())]);
    assert_eq!(files::write(&root, std::slice::from_ref(&file))?, [TARGET]);
    assert!(drift(&root, &file)?.is_empty());
    fs::write(root.join(TARGET), format!("{}# edited\n", file.bytes))?;
    assert_eq!(drift(&root, &file)?, [Stale::Changed(TARGET.to_owned())]);
    fs::remove_dir_all(root)?;
    Ok(())
}
