# SPEC

## §G GOAL

`nix/` — repo gate: dev shell, guardrails, hk steps, CI, release.

## §N NAV

rel|path|lens
up|.|-
self|nix|repo gate: dev shell, guardrails, hk steps, CI, release
sib|src|the library: grammar, YAML writer, systems, files, kinds, detect
sib|tests|integration tests over public API, goldens copied from actlith
sib|dev|`pipelith-dev`, `publish = false`: commit rules, badges, notices, AGENTS step table

## §C CONSTRAINTS

- flake inputs as actlith: `nixpkgs-lock` (nixpkgs follows), `nix-hk`, `pklith`, `itok`, `microlith`, `sherd`, `characterminator`, `rekall`, `xenolith`; fleet tools pinned by tag. `pr0d1r2.cachix.org`. 4 systems. dev shell = pklith `lib.devShell`; `.envrc` `use flake`.
- gate = hk via pklith-materialized `hk.pklith.pkl` from `.pklith`; `fast` pre-commit, `all` pre-push. guard scripts `nix/guard/`.
- clippy: fleet deny list + all + pedantic, `missing_docs` deny, fleet `clippy.toml`; `rustfmt.toml` `max_width = 80`.
- commit subject `type(scope): claim` ≤72 chars, body = WHY; changelog-touched. after `no-commit-to-branch`: branch + PR, merge ⊥ squash.
- CI: GitHub Actions, hand-written in pklith `ci.yml` shape until actlith renders it (actlith ⊥ public yet); 3 runners.
- release: `cargo-release`, Keep a Changelog. publish ! explicit approval.

## §V INVARIANTS

V42: hk hooks installed IN commit that brings hk; ∀ later commit via real hooks; ⊥ `--no-verify` (`actlith/V19`)
V43: 1 check = 1 commit = config + hk step + doc row; step adopted only after rejecting a PLANTED violation, recorded in commit body (`actlith/V20`)
V44: ∀ tool in gate from nix-pinned package, ⊥ fetched at run time (`actlith/V23`)
V45: §T rows stay in id order; out-of-order run says `AFTER T<n>` (`actlith/V27`)
V46: `tdd-order` exemptions only for commits w/ ⊥ behaviour to pin; full id + reason in `nix/guard/tdd-order.exempt` & a §B row each (`actlith/V28`)
V47: child git in tests drops `GIT_*` & scratch repos run w/ `core.hooksPath=/dev/null` (`actlith/V39`); workers share ⊥ git state: `stash = "none"`, ⊥ `core.worktree` in shared config (`actlith/V40`)
V48: change to `hk.pkl` | `hk.pklith.pkl` ! show hk itself runs expected step set; `pkl eval` passing ⊥ proof (`actlith/V42`)
V49: tests run at pre-push ∴ ∀ merge ! pass `hk check --all` BEFORE commit (`actlith/V43`)
V50: `forge-neutral` step: grep `src/` for forge tokens (`${{`, `runs-on`, `uses:`, `stages:`, `image:`, runner labels) → fail (`.:V2`)
V54: once `gen-check` is laid, a new check lands as ONE commit holding its `.pklith` row + regenerated `hk.pklith.pkl` + `nix/pklith.nix`; ⊥ claim a check in `.pklith` w/o its step; after any `.pklith` commit `pklith gen --check` ! exit 0

## §T TASKS

id|status|task|cites
T19|x|dev shell FIRST: flake on `nixpkgs-lock`, 4 systems, cachix, fleet tools pinned, Rust toolchain, pklith `lib.devShell`; `.envrc`, `.gitignore` (`.claude/`, `target/`, `.direnv/`). ASK user reload|V42,V44
T20|x|hk commit: `.pklith`, `hk.pklith.pkl`, `hk.pkl`, vendored `pkl/`, `.githooks/`; hooks live from this commit|V42,V43
T21|x|`commit-msg` step; script until `pipelith-dev` (`dev:T18`)|V43
T22|x|crate-free pklith catalog checks via `pklith lay`, 1 commit each|V43,V44
T23|x|spec gates: `mth`, `itok` + `.context-limits`, `sherd`|V43
T24|x|`ctrm`, `rekall` (`AGENTS.md`), `xnl`|V43
T25|x|seed crate through live hooks: LICENSE, `Cargo.toml` workspace, `clippy.toml`, `rustfmt.toml`, `deny.toml`, CHANGELOG; taplo, rustfmt, clippy, deny, rustdoc, doctest|V43,`.:V6`
T26|x|`nix/guard/`: `rust-mirror`, `tdd-order` + exemptions, `forge-neutral`, `no-core-worktree`; `test` + `coverage` on push|V43,V46,V47,V50
T27|x|hand-written `.github/workflows/ci.yml` (pklith shape); `actionlint`, `zizmor`, yamllint|V43
T28|.|`no-commit-to-branch` main; guardrails done → ASK user reload|V42
T29|.|nix `checks.<system>`: build, test, doc; `release.toml` w/ pre-release `hk check --all`|`.:V7`
T30|.|AFTER actlith public: `.actlith` + generated `ci.yml`, `actlith gen --check` step same commit (dogfood, D10)|V43
T31|x|`README.md`, `AGENTS.md`, `CHANGELOG.md` (Keep a Changelog), `docs/LLM-DISCLAIMER.md` as actlith; AFTER T19|`dev:V40`,`.:V7`

## §B BUGS

id|date|cause|fix
B1|2026-10-17|crate seed `2ecfbc1` claimed 5 checks in `.pklith`; pre-commit ran `gen-check` in fix mode, `pklith gen` rewrote `hk.pklith.pkl` but left it unstaged ∴ seed landed w/ `gen-check` red & `pklith lay` saw nothing to lay; repaired in `a5ce7b8`|V54
B2|2026-10-17|crate seed `2ecfbc1` added `src/lib.rs` before `tdd-order` existed; doc-only stub ∴ ⊥ RED to write first|V46
B3|2026-10-17|`scripts/commit-msg.sh` (`7867333`) copied from actlith before `tdd-order` existed, ⊥ bats mirror; tested Rust port in `pipelith-dev` (`dev:T18`) replaces it|V46
