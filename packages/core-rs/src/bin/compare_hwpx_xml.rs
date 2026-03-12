use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::path::{Path, PathBuf};

#[path = "support/archive_fixture.rs"]
mod archive_fixture;

use archive_fixture::{
    FixtureResult, NormalizedEntry, comparison_excludes_for_fixture, filtered_normalized_entries,
};

#[derive(Debug)]
struct ArchiveSnapshot {
    entries: BTreeMap<String, NormalizedEntry>,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let mut args = env::args().skip(1);
    let Some(left) = args.next() else {
        return Err(usage());
    };
    let Some(right) = args.next() else {
        return Err(usage());
    };
    if args.next().is_some() {
        return Err(usage());
    }

    let left_path = PathBuf::from(left);
    let right_path = PathBuf::from(right);

    let excludes =
        comparison_excludes(&left_path, &right_path).map_err(|error| error.to_string())?;
    let left = read_snapshot(&left_path, &excludes).map_err(|error| error.to_string())?;
    let right = read_snapshot(&right_path, &excludes).map_err(|error| error.to_string())?;

    report(&left_path, &left, &right_path, &right);
    Ok(())
}

fn read_snapshot(path: &Path, excluded_paths: &[String]) -> FixtureResult<ArchiveSnapshot> {
    let bytes = std::fs::read(path)?;
    let entries = filtered_normalized_entries(&bytes, is_text_entry, excluded_paths)?;
    Ok(ArchiveSnapshot { entries })
}

fn report(left_path: &Path, left: &ArchiveSnapshot, right_path: &Path, right: &ArchiveSnapshot) {
    println!("left: {}", left_path.display());
    println!("right: {}", right_path.display());

    let left_names = left.entries.keys().cloned().collect::<BTreeSet<_>>();
    let right_names = right.entries.keys().cloned().collect::<BTreeSet<_>>();

    let missing_in_left = right_names
        .difference(&left_names)
        .cloned()
        .collect::<Vec<_>>();
    let missing_in_right = left_names
        .difference(&right_names)
        .cloned()
        .collect::<Vec<_>>();

    if !missing_in_left.is_empty() {
        println!("missing_in_left:");
        for name in &missing_in_left {
            println!("  - {name}");
        }
    }

    if !missing_in_right.is_empty() {
        println!("missing_in_right:");
        for name in &missing_in_right {
            println!("  - {name}");
        }
    }

    let common_names = left_names
        .intersection(&right_names)
        .cloned()
        .collect::<Vec<_>>();
    let differing = common_names
        .into_iter()
        .filter_map(|name| {
            let left_entry = left.entries.get(&name)?;
            let right_entry = right.entries.get(&name)?;
            if left_entry == right_entry {
                None
            } else {
                Some((name, left_entry.clone(), right_entry.clone()))
            }
        })
        .collect::<Vec<_>>();

    if !differing.is_empty() {
        println!("differing_entries:");
        for (name, left_entry, right_entry) in &differing {
            println!("  - {name}");
            match (left_entry, right_entry) {
                (NormalizedEntry::Text(left_text), NormalizedEntry::Text(right_text)) => {
                    let summary = summarize_text_difference(left_text, right_text);
                    println!("    kind: text");
                    println!("    {summary}");
                }
                _ => {
                    println!("    left:  {}", describe_entry(left_entry));
                    println!("    right: {}", describe_entry(right_entry));
                }
            }
        }
    }

    let equivalent =
        missing_in_left.is_empty() && missing_in_right.is_empty() && differing.is_empty();
    if equivalent {
        println!("status: equivalent");
    } else {
        println!("status: different");
    }
}

fn is_text_entry(name: &str) -> bool {
    name.ends_with(".xml")
        || name.ends_with(".hpf")
        || name.ends_with(".rdf")
        || name.ends_with(".txt")
        || name == "mimetype"
}

fn comparison_excludes(left: &Path, right: &Path) -> FixtureResult<Vec<String>> {
    if let Some(root) = fixture_root_from_path(left).or_else(|| fixture_root_from_path(right)) {
        return comparison_excludes_for_fixture(&root);
    }
    Ok(Vec::new())
}

fn summarize_text_difference(left: &str, right: &str) -> String {
    let left_lines = left.lines().collect::<Vec<_>>();
    let right_lines = right.lines().collect::<Vec<_>>();
    let first_diff_index = left_lines
        .iter()
        .zip(&right_lines)
        .position(|(left_line, right_line)| left_line != right_line)
        .unwrap_or_else(|| left_lines.len().min(right_lines.len()));
    let line_number = first_diff_index + 1;
    let left_preview = left_lines
        .get(first_diff_index)
        .copied()
        .unwrap_or("<missing>");
    let right_preview = right_lines
        .get(first_diff_index)
        .copied()
        .unwrap_or("<missing>");

    format!(
        "first_diff_line={line_number} left_lines={} right_lines={} left_preview={:?} right_preview={:?}",
        left_lines.len(),
        right_lines.len(),
        left_preview,
        right_preview
    )
}

fn describe_entry(entry: &NormalizedEntry) -> String {
    match entry {
        NormalizedEntry::Text(text) => format!("text(len={})", text.len()),
        NormalizedEntry::BinaryHash(hash) => format!("binary(sha256={hash})"),
    }
}

fn usage() -> String {
    "usage: cargo run -p core-rs --bin compare_hwpx_xml -- <left.hwpx> <right.hwpx>".to_string()
}

fn fixture_root_from_path(path: &Path) -> Option<PathBuf> {
    let mut current = path.parent();
    while let Some(dir) = current {
        if dir.join("fixture.toml").is_file()
            && dir.join("golden.hwpx").is_file()
            && dir.join("input.md").is_file()
        {
            return Some(dir.to_path_buf());
        }
        current = dir.parent();
    }
    None
}
