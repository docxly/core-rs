use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};
use zip::read::ZipArchive;

#[derive(Clone, Debug, PartialEq, Eq)]
enum NormalizedEntry {
    Text(String),
    BinaryHash(String),
}

impl fmt::Display for NormalizedEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Text(text) => write!(f, "text(len={})", text.len()),
            Self::BinaryHash(hash) => write!(f, "binary(sha256={hash})"),
        }
    }
}

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

    let left = read_snapshot(&left_path)?;
    let right = read_snapshot(&right_path)?;

    report(&left_path, &left, &right_path, &right);
    Ok(())
}

fn read_snapshot(path: &Path) -> Result<ArchiveSnapshot, String> {
    let file =
        File::open(path).map_err(|error| format!("failed to open {}: {error}", path.display()))?;
    let mut archive = ZipArchive::new(file)
        .map_err(|error| format!("failed to open {} as zip: {error}", path.display()))?;

    let mut entries = BTreeMap::new();

    for index in 0..archive.len() {
        let mut entry = archive.by_index(index).map_err(|error| {
            format!(
                "failed to read zip entry {index} from {}: {error}",
                path.display()
            )
        })?;
        let name = entry.name().to_string();

        let normalized = if is_text_entry(&name) {
            let mut text = String::new();
            entry.read_to_string(&mut text).map_err(|error| {
                format!(
                    "failed to read text entry {} from {}: {error}",
                    name,
                    path.display()
                )
            })?;
            NormalizedEntry::Text(normalize_text(&text))
        } else {
            let mut bytes = Vec::new();
            entry.read_to_end(&mut bytes).map_err(|error| {
                format!(
                    "failed to read binary entry {} from {}: {error}",
                    name,
                    path.display()
                )
            })?;
            NormalizedEntry::BinaryHash(sha256_hex(&bytes))
        };

        entries.insert(name, normalized);
    }

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
                    println!("    left:  {left_entry}");
                    println!("    right: {right_entry}");
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

fn normalize_text(text: &str) -> String {
    text.replace("\r\n", "\n")
        .lines()
        .map(str::trim_end)
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string()
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
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

fn usage() -> String {
    "usage: cargo run -p core-rs --bin compare_hwpx_xml -- <left.hwpx> <right.hwpx>".to_string()
}
