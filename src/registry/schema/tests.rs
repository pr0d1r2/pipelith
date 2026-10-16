use super::{Schema, SectionSpec};

const BOTS: SectionSpec<'static> = SectionSpec {
    name: "bots",
    columns: &["bot", "args"],
    required: false,
    key: Some(0),
};

const PATHS: SectionSpec<'static> = SectionSpec {
    name: "hand-written",
    columns: &["path", "reason"],
    required: true,
    key: None,
};

const SCHEMA: Schema<'static> = Schema {
    sections: &[BOTS, PATHS],
};

#[test]
fn every_section_is_found_by_its_name() {
    assert_eq!(SCHEMA.spec("bots"), Some(&BOTS));
    assert_eq!(SCHEMA.spec("hand-written"), Some(&PATHS));
    assert_eq!(SCHEMA.spec("checks"), None);
}

#[test]
fn the_header_joins_the_columns() {
    assert_eq!(BOTS.header(), "bot|args");
    assert_eq!(PATHS.header(), "path|reason");
}

#[test]
fn the_names_hint_lists_every_section() {
    assert_eq!(SCHEMA.names(), "## bots, ## hand-written");
    assert_eq!(Schema { sections: &[] }.names(), "");
}
