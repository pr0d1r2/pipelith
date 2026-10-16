//! The `pipelith-dev` binary: `main` hands argv to the library and exits
//! with its code. The rules themselves are unit-tested beside each module.

use std::process::Command;

/// No verb is a usage error: exit 2, with the usage line on stderr.
#[test]
fn the_binary_exits_with_the_library_code() {
    let out = Command::new(env!("CARGO_BIN_EXE_pipelith-dev")).output();
    let out = out.ok();
    let code = out.as_ref().and_then(|o| o.status.code());
    let err = out.map(|o| String::from_utf8_lossy(&o.stderr).into_owned());
    let err = err.unwrap_or_default();
    assert_eq!(code, Some(2), "{err}");
    assert!(err.starts_with("pipelith-dev: usage: "), "{err}");
}
