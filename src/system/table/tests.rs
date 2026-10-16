use super::{
    ALL, DEFAULT_SYSTEMS, System, SystemError, ci_system, parse_system,
};

const NAMES: [(&str, System); 4] = [
    ("x86_64-linux", System::X86_64Linux),
    ("aarch64-linux", System::Aarch64Linux),
    ("aarch64-darwin", System::Aarch64Darwin),
    ("x86_64-darwin", System::X86_64Darwin),
];

#[test]
fn parses_every_known_name() {
    for (name, system) in NAMES {
        assert_eq!(parse_system(name), Ok(system), "{name}");
    }
}

#[test]
fn name_and_display_round_trip_through_parse() {
    for (name, system) in NAMES {
        assert_eq!(system.name(), name);
        assert_eq!(system.to_string(), name);
    }
}

#[test]
fn all_lists_every_system_in_table_order() {
    assert_eq!(ALL, NAMES.map(|(_, s)| s));
}

#[test]
fn unknown_name_is_refused_by_name() {
    let err = parse_system("riscv64-linux");
    assert_eq!(err, Err(SystemError::Unknown("riscv64-linux".to_owned())));
}

#[test]
fn parse_is_exact() {
    for name in ["X86_64-linux", " x86_64-linux", "x86_64-linux ", ""] {
        assert!(parse_system(name).is_err(), "{name:?}");
    }
}

#[test]
fn unknown_message_names_it_and_the_known_ones() {
    let msg = SystemError::Unknown("riscv64-linux".to_owned()).to_string();
    assert_eq!(
        msg,
        "unknown system `riscv64-linux`; known: x86_64-linux, \
         aarch64-linux, aarch64-darwin, x86_64-darwin"
    );
}

#[test]
fn default_systems_are_ci_systems() {
    for system in DEFAULT_SYSTEMS {
        assert_eq!(ci_system(system), Ok(system), "{system}");
    }
}

#[test]
fn x86_64_darwin_is_refused_for_ci() {
    assert_eq!(
        ci_system(System::X86_64Darwin),
        Err(SystemError::NoRunner(System::X86_64Darwin))
    );
}

#[test]
fn no_runner_message_names_hand_verification() {
    let msg = SystemError::NoRunner(System::X86_64Darwin).to_string();
    assert_eq!(
        msg,
        "system `x86_64-darwin` has no hosted CI runner on any forge; \
         build and verify it by hand on an Intel Mac"
    );
}

#[test]
fn default_systems_are_the_hosted_three() {
    assert_eq!(
        DEFAULT_SYSTEMS,
        [
            System::X86_64Linux,
            System::Aarch64Linux,
            System::Aarch64Darwin
        ]
    );
}

#[test]
fn every_system_but_x86_64_darwin_is_a_default() {
    for system in ALL {
        let hosted = system != System::X86_64Darwin;
        assert_eq!(DEFAULT_SYSTEMS.contains(&system), hosted, "{system}");
    }
}

#[test]
fn error_is_a_std_error() {
    let err: &dyn std::error::Error =
        &SystemError::NoRunner(System::X86_64Darwin);
    assert!(err.source().is_none());
}
