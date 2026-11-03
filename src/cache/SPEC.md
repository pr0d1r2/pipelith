# SPEC

## §G GOAL

`src/cache` — `## caches`: ANY nix binary cache a `## workflows` row's `cache` cell names, by kind; built-in cachix table as fallback. ⊥ step syntax, ⊥ secret injection (forge).

## §N NAV

rel|path|lens
up|.|-
up|src|the library: grammar, YAML writer, systems, files, kinds, detect
self|src/cache|`## caches`: any nix binary cache
sib|src/registry|pipe-table grammar: format line, sections, header rows, cells, errors
sib|src/system|nix system names, CI coverage set
sib|src/yaml|ordered YAML model & writer: quoting, layout, comments, `Dialect` trait
sib|src/files|generated header, write, drift check, path safety
sib|src/kind|neutral kind catalogue, row & arg validation, secret purposes, portability check
sib|src/runner|own runner labels per system
sib|src/detect|rows from tracked files, first config file

## §C CONSTRAINTS

- deps: std, `registry`. section shape, kinds & checks neutral; url & key values = the repo's, typed by the user.
- some caches hide their host ∴ url, key & push may each name a secret instead of a literal; secret value ⊥ in config file, ⊥ in any error.
- kinds v1 `cachix`, `https`. column set fixed ∴ a later kind (own CLI) reuses `name|kind|url|key|push` w/ its own cell meanings; `CacheKind` `#[non_exhaustive]`.
- push credential ⊥ literal: a config file is committed, often public.
- `https` push uploads paths as built, unsigned ∴ cache ! sign on serve | paths signed upstream; v1 ⊥ signing-key column.

## §I INTERFACES

- section `## caches`, header `name|kind|url|key|push`, key `name`, optional. e.g. `example-cache|https|https://cache.example.org|example-cache-1:<44 base64>|secret:EXAMPLE_NETRC`.
- cell value: `-` | `secret:NAME` (NAME = forge secret | CI variable, `[A-Za-z_][A-Za-z0-9_]*`) | literal.
- `cachix`: name = cachix cache name (`[a-z0-9]` then `[a-z0-9-]`); url `-` | `https://<name>.cachix.org`; key `-` only when built-in table knows name; push = auth token secret.
- `https`: url ! (literal `https://<host>…` | secret); key ! (literal | secret); push = secret holding a netrc file.
- key literal = `<keyname>:<base64 of 32 bytes>` (ed25519), keyname ⊥ empty, ⊥ blank, ⊥ `:`.
- push `-` → read-only.
- consts: `SECTION` = `caches`, `COLUMNS`, `CACHES` (`SectionSpec`, key col 0). forge crate puts `CACHES` in its schema.
- types: `CacheKind::{Cachix, Https}` (`#[non_exhaustive]`), `CacheValue::{Literal(String), Secret(String)}`, `Push::{ReadOnly, Forge, Secret(String)}` (`Forge` = built-in entry: forge's own cache-push secret, as before), `BinaryCache` (`name()`, `kind()`, `url()`, `key()` → `&CacheValue`, `push()`, `can_push()`, `read_secrets()`, `nix_settings()`, `nix_config()` = `Some` ⇔ url & key literal), `Caches`. `kind::Cache` stays 0.2.0's.
- fns: `Caches::from_document(file, &Document) -> Result<Caches, RegistryError>`, `Caches::{find, has_row, is_empty, names}`, `builtin(name)`, `builtin_names()`; `SECRET_PREFIX` = `secret:`.
- semver: pure addition to 0.2.0 ∴ next release `0.2.1`; `cargo semver-checks --baseline-rev v0.2.0` ! pass.

## §V INVARIANTS

V82: `cache` cell → `## caches` row of that name, else built-in cachix entry (v1 `pr0d1r2`, `Push::Forge`). `Checked.cache` = either; `Checked.row.cache` = 0.2.0 `kind::Cache` only for built-in entry ∴ secret-valued cache ⊥ reaches 0.2.0 API, `Row::check` & golden bytes unchanged. ⊥ both → `Row::check_with` error naming cache & `## caches` fix (⊥ rows → 0.2.0 message); `-` → ⊥ cache steps
V83: `## caches` checked at row-check time, located w/ line & column: unknown kind, repeated name, bad secret name, non-https literal url, malformed literal key, literal push, cachix url ≠ its name's, cachix key `-` w/o built-in key, https url | key `-` → error; ⊥ silent default; error ⊥ prints a secret's value (only its NAME)
V84: read = forge adds `extra-substituters` url & `extra-trusted-public-keys` key to job nix config; secret url | key → its name in `read_secrets()`, rendered only where forge has the secret (⊥ fork change request: job runs w/o cache)
V85: push (`Push::{Forge, Secret}`) ⇒ forge lists push secret (`Forge` → its `CachePush` secret); ⊥ on fork change request (`src/kind:V32`). `https` push: netrc from secret → file in job's temp dir (mode 0600, removed after), `netrc-file` set, `nix copy --to <url>` built outputs; ⊥ echo url. `ReadOnly` → ⊥ push step; `build` w/ read-only cache refused (`src/kind:V75`)

## §T TASKS

id|status|task|cites
T40|.|`CACHES`, `Caches`, `BinaryCache` & value types w/ tests; `Row::check_with` → `Checked`; 0.2.0 API unchanged; `lib.rs` declares node|V82,V83,V84,V85,`.:V4`,`.:V9`
T41|.|actlith & lablith render `cachix` & `https` caches per V84, V85; neutral rows render on both before release ! user approval|`.:V4`,`.:V7`

## §B BUGS

id|date|cause|fix
