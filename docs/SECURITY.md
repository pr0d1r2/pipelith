# Security policy

## Reporting a vulnerability

Please report vulnerabilities privately, through GitHub's
[private vulnerability reporting](https://github.com/pr0d1r2/pipelith/security/advisories/new),
not in a public issue. Include what you ran, what happened, and what you
expected. A registry text or a row that reproduces it is worth more than
a description of one.

You will get an acknowledgement within a week. A fix, or a reason there
will not be one, follows as soon as the issue is understood.

## Supported versions

pipelith has no release yet. Fixes land on `main`; once releases exist,
the latest one is supported.

## What pipelith does and does not do

pipelith is a library. It has no binary, runs no commands, and never
touches the network, the clock or the environment (SPEC.md V1). The
whole of its contact with the outside is:

- `files::write` and `files::check` read and write the files their
  caller lists, and only under the root the caller passes. A path that
  is empty, absolute, or has a `.`, `..` or `\` segment is refused
  before anything is opened (`src/files` V29).
- `detect::seed_init` creates the first config file under the root and
  refuses to overwrite one that exists.

What it guarantees about its output, since a forge crate commits that
output and a CI runner then executes it:

- The YAML writer cannot emit malformed YAML: a value that would need
  one of YAML's ambiguous forms is quoted, never written bare
  (`src/yaml` V24).
- Literal text never reads back as a forge expression: the forge's
  `yaml::Dialect` escapes every literal part, and an expression that
  would end or nest early is refused when it is built (`src/yaml` V25).
- Row values that end up in a command (`build`, `script`,
  `branch-prefix`, `token`, `package`, `cron`) are checked against a
  plain-word or field grammar when the row is validated, not when it is
  rendered (`docs/KINDS.md`).

In scope: input a forge crate reads from a repository's config file
that makes pipelith write outside the root, emit YAML that parses
differently from what the tree said, or let a row value become an
expression or a second command. Out of scope: what a forge crate does
with pipelith's output, which is that crate's own policy.

There is no `unsafe` code (`unsafe_code = "forbid"`), and the lints
deny `unwrap`, `expect`, `panic`, indexing and unchecked arithmetic, so
an abort on hostile input is a defect, not a design. pipelith has no
dependencies beyond std ([`THIRD-PARTY-NOTICES.md`](THIRD-PARTY-NOTICES.md)).
