# Built by an LLM, deliberately and in the open

<!-- hallucinogen:tending-disclaimer start -->
**Tended by an autonomous loop running Codex with GPT-5.6 luna at low
reasoning effort.** When Codex is unavailable the loop falls back to Claude,
then to a local model. The loop opens pull requests, reviews them itself
and merges them once they are green, without a human reading the diff: the
merge gate is this repository's own checks plus that automated review, not
human approval. On top of that, the maintainer runs periodic meta-reviews
with agents and corrects drift or bugs they find.

What keeps that checkable is mechanical rather than a matter of trust:
the loop generates tests and linter configurations, so its gates give
the same answer every time, and it builds command-line tools, with clear
documentation, that a person can use to inspect the same state the loop sees.

Some classes of change are held for a human by design: releases, anything
touching the loop's own safety rails, and anything that could publish to
a package registry. Everything else is not.

**Origin.** This project was built in spec-driven development sessions with
Claude Opus 5.5, with a human reviewing every change. The text below records
that period: what it says about human review holds for the code written then,
not for the merges the loop makes now.

---
<!-- hallucinogen:tending-disclaimer end -->

This repository (code, spec, tests and prose) was written by
[Claude Code](https://claude.com/claude-code) running Anthropic's **Claude
Opus 5.5**. Every commit carries a `Co-Authored-By: Claude Opus 5.5`
trailer, which `git log --format=%B | grep -c Co-Authored-By` will
confirm against `git rev-list --count HEAD`. A human owns every decision,
reviews every diff, and is accountable for what ships.

That is the disclaimer. The rest of this file is why it is a design note
rather than an apology, and what you can check for yourself.

## Why say it at all

An LLM writes plausible code, and plausible is not correct. A reader who
does not know how a repository was produced cannot calibrate how hard to
look at it.

pipelith makes the point sharper. It exists so that two CI
materializers share one grammar, one YAML writer and one drift check
instead of two copies that drift apart. A repository that did that for
others while its own rules lived in prose would be arguing against
itself. So the standard here is: **a claim in prose is a defect until
something executes it.** The numbers in this file are such claims: the
`readme-badges` step fails when they no longer match their sources.

## The method is spec-driven development

[`SPEC.md`](../SPEC.md) and one `SPEC.md` per node below it are the law,
not a description written afterwards. Together they carry
**61 invariants** (the `V` rows: what must stay true), `T` tasks (what is
decided and what is not), **6 bugs** in their `B` tables (each paired
with the rule that now catches it), and `R` research (the measurements
the constraints rest on). A rule and its checker land in the same
commit, because a rule with no runner gates nothing.

The spec's format is checked mechanically by
[`microlith`](https://github.com/pr0d1r2/microlith) and its federation by
[`sherd`](https://github.com/pr0d1r2/sherd), so "the spec says so" is a
statement about a file that was parsed, not one that was skimmed.

## The guardrails are git hooks that also run on CI

Entering the dev shell (`nix develop`, or `direnv allow`) points git at
the tracked hooks, which run [hk](https://github.com/jdx/hk) against one
definition of the gate in [`hk.pkl`](../hk.pkl); the README badge counts
its steps. Each check was laid in its own commit, and the commit that
brings a rule records the planted violation it refused.
[`ci.yml`](../.github/workflows/ci.yml) runs that same definition on
three platforms, so a laptop and a runner cannot disagree. Line coverage
has a floor of 100% that may only rise, `unsafe` is forbidden, and the
library touches no network at all.

## The record is deliberately unflattering

Every entry in the bug table was green before it was found.

**B1: checks were claimed before their steps existed.** The commit that
seeded the crate named five checks in `.pklith`; the pre-commit hook
regenerated the step file but left it unstaged, so the commit landed
with its own `gen-check` red and `pklith lay` finding nothing left to
lay. A later commit laid the steps by hand, and the rule now is one
commit per check holding its row and its regenerated steps.

**B2: the crate's first source file came before the rule that orders
tests first.** It was a doc-only stub, so there was no failing test to
write first; it is listed by full commit id in
`nix/guard/tdd-order.exempt` with the reason.

**B3: the first commit-message hook was a shell script copied without
tests.** It predated the rule that every script has a test; the tested
Rust port in `pipelith-dev` replaced it.

**B4: the pre-commit hook fixed a file and then left the fix out.** A
README regenerated during the commit stayed unstaged, so the commit
landed with stale badges, the same way B1 happened. The pre-commit hook
now only checks; a fixable finding refuses the commit and names the fix.

**B5: an expression could stay unquoted when its YAML could not.** The
writer, ported from actlith, judged an expression by its inner text, so
`#a` came out reading as a comment and `a:` as a mapping. It now judges
the rendered scalar. No golden byte changed.

**B6: two literal parts could join into a live expression.** Each was
escaped on its own, so `$` followed by `{{ x }}` opened one. Adjacent
literals are now joined before they are escaped.

## What a reader should check

1. **Does the gate run for you?** `nix develop`, then `hk check --all`. If
   a claim here is false, that is where it shows.
2. **Does the bug table look like a real log or a curated one?**
3. **Do the invariants have runners?** An invariant that nothing executes
   is exactly the defect this project exists to remove.

## Accountability

The human named in [`LICENSE`](../LICENSE) is responsible for this code,
including the parts a model wrote and the parts nobody caught. "The LLM
wrote it" explains provenance; it never transfers responsibility.
