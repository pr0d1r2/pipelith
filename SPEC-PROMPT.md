DRAFT INPUT. This file is the prompt for the first ck:spec run. When that run
lands SPEC.md, change this line to HISTORICAL INPUT: SPEC.md and the node
specs it federates become the law, and where they differ from this file, they
win. Keep the rest of this file unchanged for context.

Paradigm: paradigm, prompt round 5 (commit c0e6c6c, no tag yet). Process,
guardrail roster and shared conventions come from there; this file states
only the product and its deviations.

pipelith

The forge-neutral core of the fleet's CI materializers: the pipe-table
registry grammar, a YAML writer that cannot emit malformed YAML, the
generated-file set with its drift check, and the forge-neutral catalogue of
CI kinds. actlith renders it for GitHub Actions, lablith for GitLab CI.

# Problem and goals

The fleet runs CI on two forges. GitHub Actions runs the open-source
repositories; a self-hosted GitLab runs the unreleased and private ones. A
project starts private on GitLab and moves to GitHub when it is released, so
its CI declaration moves with it.

actlith (GitHub) already contains, mixed with its GitHub-specific code, every
part a GitLab materializer would need again: the `.actlith` pipe-table
parser, the YAML writer, the generated header and `gen --check`, and the
kind rows with their argument schemas. Copying them into lablith would start
the drift that actlith was written to end.

Goals:
- one implementation of the registry grammar, the YAML writer and the
  generated-file drift check, used by actlith and lablith
- one forge-neutral kind catalogue: kind names, row columns, argument
  schemas and nix systems mean the same on both forges
- a row that validates against the neutral catalogue renders on both forges,
  so releasing a project changes the file name, not the rows
- extraction from actlith changes no byte of any workflow actlith renders

# Naming

- Crate and repository: `pipelith`. pipe(line) + lith, and pipe tables: the
  registry grammar it owns is pipe-separated. Fleet family `pklith`,
  `actlith`, `lablith`, `microlith`, `xenolith`.
- Library only, no binary.
- Availability checked 2026-10-16: `pipelith` free on crates.io (API returns
  404); no GitHub repository `pr0d1r2/pipelith`. Recount: `curl
  https://crates.io/api/v1/crates/pipelith`, `gh repo view
  pr0d1r2/pipelith`.
- Rejected alternatives (2026-10-16):
  - `piplith`: free, but reads as Python's `pip`.
  - `lithcore`, `corelith`: break the `<purpose>lith` pattern.

# Survey (2026-10-16)

actlith source as of commit 7ce69c6, lines of Rust per module (`cat
src/<node>/*.rs | wc -l`). Each module is marked neutral (moves here),
split, or forge-specific (stays in actlith):

- `src/registry` (686): split. `format`, `table`, `cells`, `parse`, `error`
  are the grammar (`format 1` line, `## section` headings, pipe tables, `#`
  comments, `-` empty cell, `\|` literal pipe) and move here. `rows`,
  `model` hold actlith's typed sections and stay, built on the grammar.
- `src/yaml` (1071): split. `node`, `scalar`, `text`, `render` are a
  generic ordered YAML model and writer and move here. `workflow`, `job`,
  `step` are the GitHub Actions model and stay.
- `src/kind` (878): split. `row`, `args`, `contract` and the kind names are
  neutral. `checkout`, `cachix`, `nix`, `job`, `workflow`, `step`, `run`
  build GitHub steps and stay. `secret` is split: the secret a kind needs is
  neutral, how a forge injects it is not.
- `src/gen` (164): neutral (write the file set, `--check` names stale and
  missing files) apart from target paths, which come from the forge crate.
- `src/system` (118): split. The nix system names and the refusal of
  `x86_64-darwin` are neutral; the runner labels are GitHub's.
- `src/detect` (209): neutral rules (`flake.nix` gives `gate`, a
  `nixpkgs-lock` input gives `update-pins`).
- `src/script` (38): the seeded-script mechanism is neutral; the script
  bodies differ per forge (opening a PR versus a merge request).
- Forge-specific, stay: `src/pin`, `src/fetch`, `src/bots`, `src/import`,
  `src/check` (totality depends on the forge's file layout), `src/report`
  (GitHub actions and secrets), `src/cli`, the kind nodes `gate`, `build`,
  `bump`.

pklith uses the same pipe-table grammar for `.pklith`, with its own parser.

# Interface sketch

Library modules, each a sherd node:

- `registry`: parse a pipe-table file into sections of rows of cells, with
  line numbers; the caller declares which sections and columns exist. Every
  error names file, line and column. Refuses a newer `format`.
- `yaml`: ordered maps, sequences, scalars with one quoting rule, comments
  attached to nodes; render to bytes with a fixed layout. A value that would
  need YAML's ambiguous forms is quoted, never emitted bare.
- `kind`: the neutral catalogue. Kind names (`gate`, `build`, `update-pins`,
  `upstream`), the row columns `id|kind|systems|cache|args`, each kind's
  argument schema, the secrets each kind needs by purpose (cache push, open
  a change request). Validation of a row against it.
- `system`: nix system names; the set every forge must cover; refusal of a
  system no forge can run.
- `files`: a rendered file set (path, bytes), the generated header taking
  the tool name and config file name, `write` and `check`.
- `detect`: proposed rows from tracked files.

Each forge crate supplies: its config file name, its forge-only sections and
columns, the YAML model for its pipeline format, its runner table, its pin
table and its file layout.

# Decisions (2026-10-16)

1. Three crates, three repositories: `pipelith` (this, library),
   `actlith` (GitHub Actions), `lablith` (GitLab CI). Each forge crate
   depends on pipelith; pipelith depends on no forge. Local.
2. Extraction source is actlith at commit 7ce69c6. Code moves with its unit
   tests and its §V rows, so a moved rule keeps its proof. Local.
3. Byte-identical gate: actlith's golden workflows do not change when it
   switches to pipelith. A golden diff in the switch commit is a bug in the
   extraction, not a template change. Local.
4. Neutral rows: the row grammar and the kind catalogue are the contract
   between the two forges. A row that passes `kind` validation must render
   on both. A forge may add forge-only sections (actlith `## bots`,
   `## actions`) and forge-only argument keys, but each forge-only key is
   marked as such in the forge crate and reported by a portability check.
   Local.
5. Kind catalogue document: the human catalogue of what each kind does
   moves here as `docs/KINDS.md` in forge-neutral terms (intent, triggers
   as events, secrets as purposes, row values). Each forge crate keeps a
   rendering page (`docs/KINDS-GITHUB.md`, `docs/KINDS-GITLAB.md`) that maps
   intent to its syntax. Local.
6. New kinds: a kind enters the neutral catalogue only when it renders on
   both forges. A forge-only kind lives in that forge crate and is refused
   by the other forge's portability check. Refines actlith V24. Local.
7. Deterministic and offline: no network, no clock, no host, no env, as in
   actlith V1. Pin resolution (networked) stays in each forge crate.
   Candidate for the paradigm (already a fleet habit).
8. Dependencies: none at runtime beyond std. A dependency needs a §R row,
   as in actlith. Local.
9. Versioning: semver; any change to the rendered bytes of the YAML writer
   or the generated header is at least a minor version, with a changelog
   entry, since it changes every consumer's generated files (actlith V18).
   Local.
10. Visibility: public on GitHub from the first push, MIT, like the other
    fleet tools. CI is GitHub Actions rendered by actlith, so pipelith
    dogfoods through its first consumer. Local.
11. Consumption before release: actlith and lablith take pipelith as a git
    dependency pinned to a full commit, allowed for that one URL in their
    `deny.toml` `[sources]`, with the nix build's `outputHashes` entry.
    Before actlith `0.1.0-rc.1` is published, pipelith is published first
    and the git dependency becomes a crates.io version. Publishing needs
    explicit approval. Local.
12. pklith is a later candidate consumer of `registry`. Proposing that to
    pklith is an issue on pklith after pipelith ships, without naming
    private repositories. Not part of this build. Local.

# Decisions, round 2 (2026-10-16, gap answers)

Answered by the agent running the spec, from the survey and actlith's
code, while the owner was away. Each is a default for the owner to
review; a reversal is a later round, not an edit here.

13. Open question 1 does not block the `files` node: the generated
    header and `check` take the tool name and the config file name as
    arguments, so both answers fit. Default until the owner decides:
    each tool keeps its own file (`.actlith`, `.lablith`). Local.
14. The seeded-script mechanism has no node of its own. A seeded
    script is a generated file whose comment character is `#`, so
    `files` owns its header and drift check; the script bodies stay in
    the forge crates. Local.
15. Public surface: pipelith is a library, so its node modules are
    `pub` and documented, unlike actlith where every node is private.
    `lib.rs` declares the nodes and owns no logic. The public items are
    the semver surface. Local.
16. Registry schema: the caller passes a schema (section names, column
    headers, which sections are required), and `registry` returns
    sections of rows of trimmed cells with line numbers. The typed cell
    readers that actlith's `cells` holds (required, optional `-`,
    comma list, `key=value` args) are neutral and move with it. Local.
17. Portability check: `kind` exposes a function that takes a row and
    the forge's declared extra argument keys and returns one finding per
    forge-only key or forge-only kind, so each forge crate can report
    which rows would not survive a move. Local.
18. Repository layout follows actlith: one published crate `pipelith`,
    a `dev/` crate `pipelith-dev` with `publish = false`, a `nix/` gate
    node and a `tests/` node holding the golden YAML bytes copied from
    actlith, so the byte-identical gate (decision 3) is proven here
    before actlith switches. Local.

# Non-goals

- No binary and no CLI. Users run actlith or lablith.
- No forge-specific code: no GitHub Actions or GitLab CI keys, runner
  labels, action pins, image digests or API calls.
- No YAML reader. Forge crates that import existing pipelines decide on
  their own (actlith decision 23).
- No generic templating. Kinds stay typed Rust data in each forge crate.

# Prior art and rejected designs

- One tool with a `--forge` flag: rejected. Pinning, hardening checks,
  file layout and linters differ per forge, so most code would be
  per-forge anyway, and actlith's spec is GitHub-specific from its goal on.
- Copy the shared modules into lablith: rejected. Two copies of a parser
  and a writer drift, which is the problem actlith exists to solve.
- A cargo workspace holding all three crates: rejected for now. The fleet
  is one repository per tool, actlith is already built, and its own gate
  and release flow assume one published crate.
- Extract later, after lablith copies the code (rule of two): rejected.
  The second consumer is certain and starts tomorrow, so waiting only
  creates the copy that then has to be removed.

# Open questions

1. Config file name on both forges. Proposed: each tool keeps its own
   (`.actlith`, `.lablith`), same grammar, and a release is `git mv
   .lablith .actlith` plus `actlith gen`. Alternative: one neutral
   `.pipelith` read by both, which removes the rename but changes actlith's
   interface. Blocks the `files` node and the release-migration task.
2. Where the release migration lives: a `lablith release-to-github` verb,
   an `actlith import --from .lablith`, or only a documented procedure.
   Blocks nothing in this crate.

# Process

The paradigm lifecycle: dev shell, then ask me to reload; guardrails first,
one check per commit, then ask me to reload; TDD; 1-to-1 unit coverage with
floor and ceiling, aiming at 100%; federated SPEC-DAG built in sherd waves.
Build order with the sibling crates: pipelith nodes first, then actlith
switches to pipelith (byte-identical goldens), then lablith.
