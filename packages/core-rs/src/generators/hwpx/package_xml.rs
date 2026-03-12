use std::cell::Cell;

use super::profile::ResolvedHwpxCompatibilityProfile;

const SETTINGS_TEMPLATE: &str = include_str!("reference/paragraph-only/settings.xml");

#[derive(Clone, Copy)]
struct CaretPosition {
    para_id_ref: usize,
    pos: usize,
}

thread_local! {
    static SETTINGS_CARET_POS: Cell<CaretPosition> = const { Cell::new(CaretPosition {
        para_id_ref: 0,
        pos: 45,
    }) };
}

pub fn mimetype() -> &'static str {
    "application/hwp+zip"
}

pub fn version_xml(profile: ResolvedHwpxCompatibilityProfile) -> String {
    let _ = profile;
    include_str!("reference/paragraph-only/version.xml").to_string()
}

pub fn settings_xml() -> String {
    SETTINGS_CARET_POS.with(|value| {
        let caret = value.get();
        SETTINGS_TEMPLATE
            .replace(
                "paraIDRef=\"0\"",
                &format!("paraIDRef=\"{}\"", caret.para_id_ref),
            )
            .replace("pos=\"45\"", &format!("pos=\"{}\"", caret.pos))
    })
}

pub fn container_xml() -> String {
    include_str!("reference/paragraph-only/META-INF/container.xml").to_string()
}

pub fn container_rdf_xml() -> String {
    include_str!("reference/paragraph-only/META-INF/container.rdf").to_string()
}

pub fn manifest_xml() -> String {
    include_str!("reference/paragraph-only/META-INF/manifest.xml").to_string()
}

pub fn set_settings_caret_pos(pos: usize) {
    set_settings_caret(0, pos);
}

pub fn set_settings_caret(para_id_ref: usize, pos: usize) {
    SETTINGS_CARET_POS.with(|value| value.set(CaretPosition { para_id_ref, pos }));
}
