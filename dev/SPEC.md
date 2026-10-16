# SPEC

## §G GOAL

`dev/` — `pipelith-dev`, `publish = false`: repo tooling & its drift checks.

## §N NAV

rel|path|lens
up|.|-
self|dev|`pipelith-dev`, `publish = false`: commit rules, badges, notices, AGENTS step table
sib|src|the library: grammar, YAML writer, systems, files, kinds, detect
sib|tests|integration tests over public API, goldens copied from actlith
sib|nix|repo gate: dev shell, guardrails, hk steps, CI, release

## §C CONSTRAINTS

- workspace member, `publish = false`, `release = false`. ported from actlith-dev.

## §V INVARIANTS

V40: README badges block, `docs/THIRD-PARTY-NOTICES.md` & AGENTS.md step table generated; drift → gate fails (`actlith/V36`)
V41: commit-msg & changelog rules = tested Rust here; hk `commit-msg` runs them via `cargo run -p pipelith-dev` (`actlith/V37`)

## §T TASKS

id|status|task|cites
T18|.|port actlith-dev: commit-msg & changelog rules (RED, GREEN), AGENTS step table, README badges, notices + drift steps|V40,V41

## §B BUGS

id|date|cause|fix
