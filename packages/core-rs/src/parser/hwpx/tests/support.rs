use std::collections::BTreeMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use zip::CompressionMethod;
use zip::write::SimpleFileOptions;

pub(super) fn approved_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("hwpx")
        .join("approved")
}

pub(super) fn provisional_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("hwpx")
        .join("provisional")
}

pub(super) fn approved_fixture_names() -> &'static [&'static str] {
    &[
        "blockquote-basic",
        "code-block-basic",
        "core-heading",
        "core-inline-style",
        "core-link-text",
        "core-mixed",
        "core-paragraph",
        "list-basic",
        "list-nested-depth-2",
        "ordered-list-basic",
        "ordered-list-nested-depth-2",
        "style-brand-color",
        "style-centered-layout",
        "style-typography",
        "table-alignment",
        "table-basic",
    ]
}

pub(super) fn expected_metadata_from_content_hpf(
    content_hpf: &str,
) -> super::super::RawHwpxMetadata {
    let xml = roxmltree::Document::parse(content_hpf).unwrap();
    let metadata = xml
        .descendants()
        .find(|node| node.is_element() && node.tag_name().name() == "metadata")
        .unwrap();
    super::super::RawHwpxMetadata {
        title: text_child(metadata, "title"),
        language: text_child(metadata, "language"),
        creator: named_meta(metadata, "creator"),
        lastsaveby: named_meta(metadata, "lastsaveby"),
    }
}

fn text_child(node: roxmltree::Node<'_, '_>, name: &str) -> Option<String> {
    node.children()
        .find(|child| child.is_element() && child.tag_name().name() == name)
        .map(|child| child.text().unwrap_or_default().trim().to_string())
        .filter(|value| !value.is_empty())
}

fn named_meta(node: roxmltree::Node<'_, '_>, name: &str) -> Option<String> {
    node.children()
        .find(|child| {
            child.is_element()
                && child.tag_name().name() == "meta"
                && child.attribute("name") == Some(name)
        })
        .map(|child| child.text().unwrap_or_default().trim().to_string())
        .filter(|value| !value.is_empty())
}

pub(super) fn mutate_fixture_section(
    fixture: &str,
    mutate: impl FnOnce(String) -> String,
) -> Vec<u8> {
    let root = approved_root().join(fixture).join("expected");
    let mut entries = expected_entries(&root);
    let section = String::from_utf8(entries.remove("Contents/section0.xml").unwrap()).unwrap();
    entries.insert(
        "Contents/section0.xml".to_string(),
        mutate(section).into_bytes(),
    );
    build_hwpx_archive(entries)
}

pub(super) fn add_section_entry(fixture: &str) -> Vec<u8> {
    let root = approved_root().join(fixture).join("expected");
    let mut entries = expected_entries(&root);
    entries.insert(
        "Contents/section1.xml".to_string(),
        entries.get("Contents/section0.xml").unwrap().clone(),
    );
    build_hwpx_archive(entries)
}

pub(super) fn expected_entries(root: &Path) -> BTreeMap<String, Vec<u8>> {
    let mut entries = BTreeMap::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let mut children = fs::read_dir(&dir)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .collect::<Vec<_>>();
        children.sort();
        for child in children {
            if child.is_dir() {
                stack.push(child);
                continue;
            }
            if child.extension().and_then(|ext| ext.to_str()) == Some("sha256") {
                continue;
            }
            let relative = child
                .strip_prefix(root)
                .unwrap()
                .to_string_lossy()
                .replace('\\', "/");
            entries.insert(relative, fs::read(child).unwrap());
        }
    }
    entries
}

pub(super) fn build_hwpx_archive(entries: BTreeMap<String, Vec<u8>>) -> Vec<u8> {
    let mut bytes = Vec::new();
    let mut writer = zip::ZipWriter::new(std::io::Cursor::new(&mut bytes));
    let stored = SimpleFileOptions::default().compression_method(CompressionMethod::Stored);
    let deflated = SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);

    for (path, contents) in entries {
        let options = if path == "mimetype" { stored } else { deflated };
        writer.start_file(path, options).unwrap();
        writer.write_all(&contents).unwrap();
    }
    writer.finish().unwrap();
    bytes
}
