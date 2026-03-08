pub fn mimetype() -> &'static str {
    "application/hwp+zip"
}

pub fn version_xml() -> String {
    include_str!("reference/paragraph-only/version.xml").to_string()
}

pub fn settings_xml() -> String {
    include_str!("reference/paragraph-only/settings.xml").to_string()
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
