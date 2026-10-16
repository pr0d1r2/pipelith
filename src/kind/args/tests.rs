//! Arg schema check: unknown, repeated, missing, defaults, forge keys
//! (T14, V31, V34).

use crate::kind::{
    ArgDefault, ArgSpec, Args, Extras, KindError, KindSpec, RawRow, TIMEOUT,
    parse_args,
};
use crate::registry::Arg;

const SPEC: KindSpec = KindSpec {
    name: "build",
    schema: &[
        ArgSpec {
            key: "package",
            required: true,
            default: ArgDefault::None,
        },
        ArgSpec {
            key: "note",
            required: false,
            default: ArgDefault::None,
        },
        ArgSpec {
            key: "token",
            required: false,
            default: ArgDefault::Forge,
        },
        TIMEOUT,
    ],
    default_systems: &[],
    secrets: &[],
};

const BARE: KindSpec = KindSpec {
    schema: &[],
    ..SPEC
};

/// A forge-only key, with a default.
const RUNNER: [ArgSpec; 1] = [ArgSpec {
    key: "runner",
    required: false,
    default: ArgDefault::Value("small"),
}];

const NONE: Extras<'static> = Extras {
    kinds: &[],
    keys: &[],
};

fn row(args: &[(&str, &str)]) -> RawRow {
    RawRow {
        line: 12,
        id: "build".to_owned(),
        kind: "build".to_owned(),
        args: args
            .iter()
            .map(|&(k, v)| Arg {
                key: k.to_owned(),
                value: v.to_owned(),
            })
            .collect(),
        ..RawRow::default()
    }
}

fn refused(spec: &KindSpec, args: &[(&str, &str)]) -> String {
    parse_args(spec, &row(args), &NONE)
        .err()
        .map(|e| e.to_string())
        .unwrap_or_default()
}

fn parsed(args: &[(&str, &str)]) -> Result<Args, KindError> {
    parse_args(&SPEC, &row(args), &NONE)
}

#[test]
fn given_values_are_kept_and_defaults_filled() -> Result<(), KindError> {
    let args = parsed(&[("package", "hk")])?;
    assert_eq!(args.line(), 12);
    assert_eq!(args.get("package"), Some("hk"));
    assert_eq!(args.get("timeout"), Some("30"));
    assert_eq!(args.get("note"), None);
    Ok(())
}

#[test]
fn a_forge_default_is_left_for_the_forge() -> Result<(), KindError> {
    assert_eq!(parsed(&[("package", "hk")])?.get("token"), None);
    let given = parsed(&[("package", "hk"), ("token", "BOT")])?;
    assert_eq!(given.get("token"), Some("BOT"));
    Ok(())
}

#[test]
fn a_given_value_beats_the_default() -> Result<(), KindError> {
    let args = parsed(&[("timeout", "45"), ("package", "hk")])?;
    assert_eq!(args.get("timeout"), Some("45"));
    assert_eq!(args.number("timeout")?, Some(45));
    assert_eq!(args.number("note")?, None);
    Ok(())
}

#[test]
fn an_unknown_key_is_refused_naming_line_and_keys() {
    assert_eq!(
        refused(&SPEC, &[("package", "hk"), ("flavor", "x")]),
        "line 12: kind `build` has no arg `flavor` (remove `flavor` or use \
         one of: package, note, token, timeout)"
    );
}

#[test]
fn a_kind_without_args_says_so() {
    assert_eq!(
        refused(&BARE, &[("x", "1")]),
        "line 12: kind `build` has no arg `x` (write `-` in args)"
    );
}

#[test]
fn a_missing_required_key_is_refused() {
    assert_eq!(
        refused(&SPEC, &[]),
        "line 12: kind `build` needs arg `package` (add `package=<value>` \
         to args)"
    );
}

#[test]
fn a_repeated_key_is_refused() {
    assert_eq!(
        refused(&SPEC, &[("package", "a"), ("package", "b")]),
        "line 12: arg `package` is given twice (keep one `package=`)"
    );
}

#[test]
fn a_forge_key_is_valid_only_where_declared() -> Result<(), KindError> {
    let given = [("package", "hk"), ("runner", "big")];
    let refusal = refused(&SPEC, &given);
    assert!(refusal.contains("has no arg `runner`"), "{refusal}");
    let extras = Extras {
        kinds: &[],
        keys: &RUNNER,
    };
    let args = parse_args(&SPEC, &row(&given), &extras)?;
    assert_eq!(args.get("runner"), Some("big"));
    let args = parse_args(&SPEC, &row(&[("package", "hk")]), &extras)?;
    assert_eq!(args.get("runner"), Some("small"));
    Ok(())
}

#[test]
fn a_non_number_is_refused() -> Result<(), KindError> {
    let args = parsed(&[("package", "hk"), ("timeout", "soon")])?;
    let error = args.number("timeout").err().map(|e| e.to_string());
    assert_eq!(
        error.as_deref(),
        Some(
            "line 12: arg `timeout` is `soon`, not a whole number (write \
             `timeout=<number>`)"
        )
    );
    Ok(())
}

#[test]
fn the_timeout_is_one_to_360_minutes() -> Result<(), KindError> {
    assert_eq!(parsed(&[("package", "hk")])?.timeout()?, 30);
    assert_eq!(Args::default().timeout()?, 30);
    for ok in ["1", "360"] {
        let args = parsed(&[("package", "hk"), ("timeout", ok)])?;
        assert_eq!(args.timeout()?.to_string(), ok);
    }
    Ok(())
}

#[test]
fn a_timeout_out_of_range_is_refused() -> Result<(), KindError> {
    for bad in ["0", "361"] {
        let args = parsed(&[("package", "hk"), ("timeout", bad)])?;
        let error = args.timeout().err().map(|e| e.to_string());
        let want = format!(
            "line 12: timeout `{bad}` is not 1 to 360 minutes (write \
             `timeout=<1..360>`)"
        );
        assert_eq!(error, Some(want));
    }
    Ok(())
}
