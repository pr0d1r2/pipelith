use crate::yaml::tests::Github;
use crate::yaml::{Comment, Entry, Expr, Node, Text, Value, document};

type Built<T> = Result<T, &'static str>;

/// The hand-written ci.yml pklith-shaped repositories carry, and two
/// generated shapes, from actlith @ `7ce69c6` without their header line
/// (`src/files` owns it): the bytes this writer must reproduce (`.:V3`).
const CI: &str = include_str!("../../../tests/fixtures/yaml/ci.yaml");
const BUILD: &str = include_str!("../../../tests/fixtures/yaml/build.yaml");
const BUMP: &str =
    include_str!("../../../tests/fixtures/yaml/update-pins.yaml");

fn write(comments: &[Comment], entries: &[Entry]) -> String {
    document(&Github, comments, entries)
}

fn text(s: &str) -> Node {
    Node::Value(Value::Text(Text::lit(s)))
}

fn expr(inner: &str) -> Built<Expr> {
    Expr::new(&Github, inner).ok_or("bad expression")
}

fn ex(inner: &str) -> Built<Node> {
    Ok(Node::Value(Value::Text(Text::expr(expr(inner)?))))
}

fn fixed(key: &'static str, node: Node) -> Entry {
    Entry::fixed(key, node)
}

fn user(key: &str, node: Node) -> Entry {
    Entry::user(key, node)
}

fn flag(b: bool) -> Node {
    Node::Value(Value::Bool(b))
}

fn int(n: u64) -> Node {
    Node::Value(Value::Int(n))
}

fn list(items: &[&str]) -> Node {
    Node::List(items.iter().map(|s| Text::lit(s)).collect())
}

#[test]
fn document_starts_with_marker_then_comments_and_blank_separated_keys() {
    let entries = [fixed("name", text("ci")), fixed("on", Node::Null)];
    let out = write(&[Comment::new("top")], &entries);
    assert_eq!(out, "---\n# top\nname: ci\n\non:\n");
}

#[test]
fn nested_maps_indent_two_and_empty_map_is_braces() {
    let inner = vec![
        fixed("n", int(0)),
        fixed("b", flag(false)),
        fixed("e", Node::Map(Vec::new())),
    ];
    let out = write(&[], &[fixed("a", Node::Map(inner))]);
    assert_eq!(out, "---\na:\n  n: 0\n  b: false\n  e: {}\n");
}

#[test]
fn sequence_items_carry_comments_and_trailing_comments() {
    let first = fixed("uses", text("x@1"))
        .above(Comment::new("why"))
        .after(Comment::new("v1"));
    let item = vec![first, fixed("with", Node::Null)];
    let seq = Node::Seq(vec![item, Vec::new()]);
    let out = write(&[], &[fixed("steps", seq)]);
    let want = "---\nsteps:\n  # why\n  - uses: x@1  # v1\n    with:\n  - {}\n";
    assert_eq!(out, want);
}

#[test]
fn empty_sequence_is_brackets() {
    let out = write(&[], &[fixed("s", Node::Seq(Vec::new()))]);
    assert_eq!(out, "---\ns: []\n");
}

#[test]
fn user_keys_are_quoted_when_needed() {
    let out = write(&[], &[user("on", text("x"))]);
    assert_eq!(out, "---\n'on': x\n");
}

#[test]
fn short_list_is_flow_long_list_is_block() {
    let out = write(&[], &[fixed("k", list(&["a", "b c"]))]);
    assert_eq!(out, "---\nk: [a, b c]\n");
    let word = "w".repeat(70);
    let out = write(&[], &[fixed("k", list(&[&word, &word]))]);
    assert_eq!(out, format!("---\nk:\n  - {word}\n  - {word}\n"));
}

#[test]
fn multiline_text_is_literal_block_with_blank_lines_bare() {
    let out = write(&[], &[fixed("k", text("a\n\nb\n"))]);
    assert_eq!(out, "---\nk: |\n  a\n\n  b\n");
}

#[test]
fn long_expression_folds_long_word_stays_one_line() -> Built<()> {
    let long = format!("a == '{}' && b", "x".repeat(100));
    let out = write(&[], &[fixed("k", ex(&long)?)]);
    let want = format!(
        "---\nk: >-\n  ${{{{ a == '{}' &&\n  b }}}}\n",
        "x".repeat(100)
    );
    assert_eq!(out, want);
    let word = "w".repeat(130);
    let out = write(&[], &[fixed("k", text(&word))]);
    assert_eq!(out, format!("---\nk: {word}\n"));
    Ok(())
}

#[test]
fn trailing_comment_counts_toward_width() {
    let value = "v ".repeat(55) + "v";
    let entry = fixed("k", text(&value)).after(Comment::new("note"));
    let out = write(&[], &[entry]);
    assert!(out.starts_with("---\nk: >-  # note\n"));
}

#[test]
fn literal_text_passes_the_dialect_escape() {
    let out = write(&[], &[fixed("run", text("echo ${{ x }}"))]);
    assert_eq!(out, "---\nrun: 'echo ${{ ''${{'' }} x }}'\n");
}

const SKIP_FORK: &str = "github.event_name == 'pull_request' && \
    github.event.pull_request.head.repo.full_name != github.repository";

const PUSH_WHY: &str = "Push what this job builds, so the next run \
    downloads it. The flake\nmakes the fleet tools follow nixpkgs-lock; \
    when that pin is newer\nthan theirs, their own caches never match and \
    every runner builds\nthem from source. Fork PRs get no secret and still \
    read the cache.";

const CI_WHY: &str = "Hand-written until `actlith gen` can render the gate \
    kind, then generated\n(T10, V21). Orchestration only: the gate is \
    hk.pkl, run through the same\ndev shell a laptop uses, so CI and a local \
    `hk check` cannot disagree\nabout a tool version. One job per CI system \
    (SPEC.md C).";

const NIX_CONFIG: &str = "extra-substituters = https://pr0d1r2.cachix.org\n\
    extra-trusted-public-keys = pr0d1r2.cachix.org-1:\
    NfWjbhgAj41byXhCKiaE+av3Vnphm1fTezHXEGsiQIM=\n";

const FETCH_WHY: &str = "Full history: tdd-order reads the push range, \
    and a later semver\nstep compares with the last release tag.";

const FILTER_WHY: &str = "Only what was built here; sources and \
    tarballs stay out of the cache.";

/// A `uses:` step: the pinned reference with its version after it.
fn uses(reference: &str, version: &str, with: Vec<Entry>) -> Vec<Entry> {
    let head = fixed("uses", text(reference)).after(Comment::new(version));
    vec![head, fixed("with", Node::Map(with))]
}

fn run(command: &str) -> Vec<Entry> {
    vec![fixed("run", text(command))]
}

fn checkout() -> Vec<Entry> {
    let with = vec![
        user("persist-credentials", flag(false)),
        user("fetch-depth", int(0)).above(Comment::new(FETCH_WHY)),
    ];
    let sha = "actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1";
    uses(sha, "v7.0.1", with)
}

fn install_nix() -> Vec<Entry> {
    let with = vec![user("extra_nix_config", text(NIX_CONFIG))];
    let sha =
        "cachix/install-nix-action@630ae543ea3a38a9a4166f03376c02c50f408342";
    uses(sha, "v31.11.0", with)
}

fn cachix() -> Built<Vec<Entry>> {
    let filter = text("(-source$|nixpkgs\\.tar\\.gz$)");
    let with = vec![
        user("name", text("pr0d1r2")),
        user("authToken", ex("secrets.CACHIX_AUTH_TOKEN")?),
        user("skipPush", ex(SKIP_FORK)?),
        user("pushFilter", filter).above(Comment::new(FILTER_WHY)),
    ];
    let sha = "cachix/cachix-action@5f2d7c5294214f71b873db4b969586b980625e71";
    let mut step = uses(sha, "v17", with);
    if let Some(first) = step.first_mut() {
        first.comments.push(Comment::new(PUSH_WHY));
    }
    Ok(step)
}

fn strategy() -> Node {
    let systems = "x86_64-linux, aarch64-linux, aarch64-darwin";
    let os = list(&["ubuntu-latest", "ubuntu-24.04-arm", "macos-latest"]);
    let axis = user("os", os).above(Comment::new(systems));
    Node::Map(vec![
        fixed("fail-fast", flag(false)),
        fixed("matrix", Node::Map(vec![axis])),
    ])
}

fn gate_job() -> Built<Node> {
    let gate = run("nix develop --command hk check --all --check");
    let steps = vec![checkout(), install_nix(), cachix()?, gate];
    Ok(Node::Map(vec![
        fixed("name", text("gate")),
        fixed("runs-on", ex("matrix.os")?),
        fixed("strategy", strategy()),
        fixed("timeout-minutes", int(30)),
        fixed("steps", Node::Seq(steps)),
    ]))
}

fn gate_on() -> Node {
    let push = Node::Map(vec![fixed("branches", list(&["main"]))]);
    Node::Map(vec![
        fixed("push", push),
        fixed("pull_request", Node::Null),
        fixed("workflow_dispatch", Node::Null),
    ])
}

fn concurrency(id: &str, cancel: bool) -> Built<Node> {
    let group = Text::lit(&format!("{id}-")).then_expr(expr("github.ref")?);
    Ok(Node::Map(vec![
        fixed("group", Node::Value(Value::Text(group))),
        fixed("cancel-in-progress", flag(cancel)),
    ]))
}

fn read_only() -> Node {
    Node::Map(vec![fixed("contents", text("read"))])
}

fn ci() -> Built<Vec<Entry>> {
    Ok(vec![
        fixed("name", text("ci")),
        fixed("on", gate_on()),
        fixed("concurrency", concurrency("ci", true)?),
        fixed("permissions", read_only()),
        fixed("jobs", Node::Map(vec![user("gate", gate_job()?)])),
    ])
}

#[test]
fn ci_yml_is_reproduced_byte_for_byte() -> Built<()> {
    assert_eq!(write(&[Comment::new(CI_WHY)], &ci()?), CI);
    Ok(())
}

#[test]
fn writing_is_deterministic() -> Built<()> {
    let entries = ci()?;
    assert_eq!(write(&[], &entries), write(&[], &entries.clone()));
    Ok(())
}

fn bump_job() -> Node {
    let with = vec![user("persist-credentials", flag(true))];
    let steps = vec![
        uses("actions/checkout@abc", "v7", with),
        run("scripts/actlith/update-pins.sh"),
    ];
    let permissions = Node::Map(vec![
        fixed("contents", text("write")),
        fixed("pull-requests", text("write")),
    ]);
    Node::Map(vec![
        fixed("runs-on", text("ubuntu-latest")),
        fixed("permissions", permissions),
        fixed("timeout-minutes", int(30)),
        fixed("steps", Node::Seq(steps)),
    ])
}

fn bump() -> Built<Vec<Entry>> {
    let cron = vec![fixed("cron", text("50 6 * * *"))];
    let on = Node::Map(vec![
        fixed("schedule", Node::Seq(vec![cron])),
        fixed("workflow_dispatch", Node::Null),
    ]);
    let jobs = Node::Map(vec![user("update-pins", bump_job())]);
    Ok(vec![
        fixed("name", text("update-pins")),
        fixed("on", on),
        fixed("concurrency", concurrency("update-pins", false)?),
        fixed("permissions", read_only()),
        fixed("jobs", jobs),
    ])
}

#[test]
fn scheduled_shape_with_job_permissions() -> Built<()> {
    assert_eq!(write(&[], &bump()?), BUMP);
    Ok(())
}

fn pair(os: &str, system: &str) -> Vec<Entry> {
    vec![user("os", text(os)), user("system", text(system))]
}

fn include_strategy() -> Node {
    let include = Node::Seq(vec![
        pair("ubuntu-latest", "x86_64-linux"),
        pair("macos-latest", "aarch64-darwin"),
    ]);
    let matrix = Node::Map(vec![fixed("include", include)]);
    Node::Map(vec![
        fixed("fail-fast", flag(false)),
        fixed("matrix", matrix),
    ])
}

fn build_job() -> Built<Node> {
    let strategy = include_strategy();
    let env = Node::Map(vec![user("SYSTEM", ex("matrix.system")?)]);
    let steps = vec![run("nix build --print-build-logs .#default")];
    Ok(Node::Map(vec![
        fixed("runs-on", ex("matrix.os")?),
        fixed("strategy", strategy),
        fixed("timeout-minutes", int(10)),
        fixed("env", env),
        fixed("steps", Node::Seq(steps)),
    ]))
}

fn verify_step() -> Built<Vec<Entry>> {
    Ok(vec![
        fixed("name", text("verify cache")),
        fixed("id", text("verify")),
        fixed("if", ex("success()")?),
        fixed("run", text("scripts/actlith/verify-cache.sh")),
        fixed("env", Node::Map(vec![user("CACHE", text("pr0d1r2"))])),
    ])
}

fn verify_job() -> Built<Entry> {
    let main = "github.event_name == 'push' && github.ref == 'refs/heads/main'";
    let job = Node::Map(vec![
        fixed("name", text("verify-cache")),
        fixed("needs", list(&["build"])),
        fixed("if", ex(main)?),
        fixed("runs-on", text("ubuntu-latest")),
        fixed("timeout-minutes", int(10)),
        fixed("steps", Node::Seq(vec![verify_step()?])),
    ]);
    let why = "a cachix push failure does not fail its job";
    Ok(user("verify-cache", job).above(Comment::new(why)))
}

fn build() -> Built<Vec<Entry>> {
    let jobs = vec![user("build", build_job()?), verify_job()?];
    let env = Node::Map(vec![user("NIX_CONFIG", text("a = b"))]);
    Ok(vec![
        fixed("name", text("build")),
        fixed("on", gate_on()),
        fixed("permissions", Node::Map(Vec::new())),
        fixed("env", env),
        fixed("jobs", Node::Map(jobs)),
    ])
}

#[test]
fn needs_if_include_env_and_empty_map_shape() -> Built<()> {
    assert_eq!(write(&[], &build()?), BUILD);
    Ok(())
}
