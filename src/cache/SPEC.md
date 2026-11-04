# SPEC

## §G GOAL

`src/cache` — `## caches` (`cachix`, `https`): caches a `## workflows` `cache` cell names; built-in cachix table fallback. ⊥ step syntax, ⊥ secret injection (forge).

## §N NAV

rel|path|lens
up|.|-
up|src|the library: grammar, YAML writer, systems, files, kinds, cache, detect
self|src/cache|`## caches`: `cachix`, `https` caches
sib|src/registry|pipe-table grammar: format line, sections, header rows, cells, errors
sib|src/system|nix system names, CI coverage set
sib|src/yaml|ordered YAML model & writer: quoting, layout, comments, `Dialect` trait
sib|src/files|generated header, write, drift check, path safety
sib|src/kind|neutral kind catalogue, row & arg validation, secret purposes, portability check
sib|src/runner|own runner labels per system
sib|src/detect|rows from tracked files, first config file

## §C CONSTRAINTS

- deps: std, `registry`. section shape, kinds & checks neutral; url & key values = the repo's, typed by the user.
- some caches hide their host | need auth to read ∴ url, key & credentials may each name a secret; secret value ⊥ in config file, ⊥ in any error.
- kinds v1 `cachix`, `https`. column set fixed ∴ a later kind (own CLI) reuses `name|kind|url|key|push` w/ its own cell meanings; `CacheKind` `#[non_exhaustive]`.
- credential (push | read) ⊥ literal: config file committed, often public. column set fixed ∴ read credential = arg in `push` cell, ⊥ 6th column (old files stay valid).
- `https` push uploads paths as built, unsigned ∴ cache ! sign on serve | paths signed upstream; v1 ⊥ signing-key column.

## §I INTERFACES

- section `## caches`, header `name|kind|url|key|push`, key `name`, optional. e.g. `example-cache|https|https://cache.example.org|example-cache-1:<44 base64>|secret:EXAMPLE_NETRC`.
- cell value: `-` | `secret:NAME` (NAME = forge secret | CI variable, `[A-Za-z_][A-Za-z0-9_]*`) | literal.
- `cachix`: name = cachix cache name (`[a-z0-9]` then `[a-z0-9-]`); url `-` | `https://<name>.cachix.org`; key `-` only when built-in table knows name; push = auth token secret.
- `https`: name = `[A-Za-z0-9._-]+`, ⊥ leading `-`; url ! (literal `https://<host>…`, ⊥ `user@`, | secret); key ! (literal | secret); push = secret holding a netrc file, endpoint ! accept authenticated `PUT`.
- key literal = `<keyname>:<base64 of 32 bytes>` (ed25519), keyname ⊥ empty, ⊥ blank, ⊥ `:`.
- `push` cell: `-` → read-only · `secret:NAME` → push · args `push=secret:P, read=secret:R` (either | both; `read` https only) → read netrc for substituter auth.
- consts: `SECTION` = `caches`, `COLUMNS`, `CACHES` (`SectionSpec`, key col 0). forge crate puts `CACHES` in its schema.
- types (enums `#[non_exhaustive]`): `CacheKind::{Cachix, Https}`, `CacheValue::{Literal(String), Secret(String)}`, `Push::{ReadOnly, Forge, Secret(String)}` (`Forge` = built-in entry: forge's own push secret, as before), `BinaryCache` (`name()`, `kind()`, `url()`, `key()` → `&CacheValue`, `push()`, `can_push()`, `read_netrc()`, `read_secrets()` (url, key, read netrc names), `nix_settings()`, `nix_config()` = `Some` ⇔ url & key literal ∧ ⊥ read netrc), `Caches`. `kind::Cache` stays 0.2.0's.
- fns: `Caches::from_document(file, &Document) -> Result<Caches, RegistryError>`, `Caches::{find, has_row, is_empty, names}`, `builtin(name)`, `builtin_names()`; `SECRET_PREFIX` = `secret:`.
- semver: additions only; `cargo semver-checks --baseline-rev <last tag>` ! pass.

## §V INVARIANTS

V82: `cache` cell → `## caches` row of that name, else built-in cachix entry (v1 `pr0d1r2`, `Push::Forge`). `Checked.cache` = either; `Checked.row.cache` = 0.2.0 `kind::Cache` only for built-in entry ∴ secret-valued cache ⊥ reaches 0.2.0 API, `Row::check` & golden bytes unchanged. ⊥ both → `Row::check_with` error naming cache & `## caches` fix (⊥ rows → 0.2.0 message); `-` → ⊥ cache steps
V83: `## caches` checked at row-check time, located w/ line & column: unknown kind, repeated name, name ∉ its kind's charset, bad secret name, non-https | `user@` | host-less literal url (url ⊥ echoed, any scheme), malformed literal key, literal push, cachix url ≠ its name's, cachix key `-` w/o built-in key, https url | key `-` → error; ⊥ silent default; pipelith error ⊥ prints a secret's value (only its NAME)
V84: read = forge adds `extra-substituters` url & `extra-trusted-public-keys` key to job nix config; secret url | key → its name in `read_secrets()`, rendered only where forge has the secret (⊥ fork change request: job runs w/o cache)
V85: push (`Push::{Forge, Secret}`) ⇒ forge lists push secret (`Forge` → its `CachePush` secret); ⊥ on fork change request (`src/kind:V32`). `https` push: netrc from secret → file in job's temp dir (mode 0600, removed after), `netrc-file` set, `nix copy --to <url>` built outputs; ⊥ echo url. `ReadOnly` → ⊥ push step; `build` w/ read-only cache refused (`src/kind:V75`)
V86: read netrc (`https`, `read=secret:R`) → forge writes it to 0600 file in job temp dir before nix runs, sets `netrc-file` in job nix config (nix reads it for substituters); run ⊥ secret (fork change request) → ⊥ cache. literal | unknown arg | `read` on cachix → error

## §T TASKS

id|status|task|cites
T40|x|`CACHES`, `Caches`, `BinaryCache` & value types w/ tests; `Row::check_with` → `Checked`; 0.2.0 API unchanged; `lib.rs` declares node|V82,V83,V84,V85,`.:V4`,`.:V9`
T41|.|actlith & lablith render `cachix` & `https` caches per V84, V85, V86 (`read_netrc()`); forge ! mask secret url in job logs (`nix copy` output ⊥ pipelith's); neutral rows render on both before release ! user approval|`.:V4`,`.:V7`
T42|.|`read=` netrc arg in `push` cell; `BinaryCache::read_netrc` w/ tests; `nix_config` `None` w/ read netrc|V83,V86,`.:V9`

## §B BUGS

id|date|cause|fix
