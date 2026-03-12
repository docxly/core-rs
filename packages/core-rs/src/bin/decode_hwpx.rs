use std::env;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Component, Path, PathBuf};

use core_rs::debug_parse_hwpx;
use zip::read::ZipArchive;

const ZIP_MAGIC: &[u8; 4] = b"PK\x03\x04";
const OLE_MAGIC: &[u8; 8] = b"\xD0\xCF\x11\xE0\xA1\xB1\x1A\xE1";
const MAX_CONSOLE_TEXT_BYTES: usize = 256 * 1024;

#[derive(Debug, PartialEq, Eq)]
struct CliOptions {
    input_path: PathBuf,
    output_dir: Option<PathBuf>,
    parse_semantics: bool,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let Some(options) = parse_args(env::args().skip(1))? else {
        println!("{}", usage());
        return Ok(());
    };

    let CliOptions {
        input_path,
        output_dir,
        parse_semantics,
    } = options;
    let mut file = File::open(&input_path)
        .map_err(|error| format!("failed to open {}: {error}", input_path.display()))?;

    let mut header = [0_u8; 8];
    let read_len = file
        .read(&mut header)
        .map_err(|error| format!("failed to read {}: {error}", input_path.display()))?;
    if read_len < 4 {
        return Err(format!(
            "{} is too small to detect format",
            input_path.display()
        ));
    }

    if &header[..4] == ZIP_MAGIC {
        decode_hwpx_zip(&input_path, output_dir.as_deref(), parse_semantics)?;
        return Ok(());
    }

    if read_len >= 8 && &header == OLE_MAGIC {
        println!("format: hwp5-ole");
        println!("path: {}", input_path.display());
        println!("status: not an HWPX zip package");
        println!(
            "detail: this file uses the OLE Compound File Binary format used by Hangul Word Processor 5.x"
        );
        println!(
            "next_step: export or save the document as HWPX before attempting XML-level reverse engineering"
        );
        return Ok(());
    }

    println!("format: unknown");
    println!("path: {}", input_path.display());
    println!("magic: {}", to_hex(&header[..read_len]));
    Err("unsupported file format".to_string())
}

fn decode_hwpx_zip(
    input_path: &Path,
    output_dir: Option<&Path>,
    parse_semantics: bool,
) -> Result<(), String> {
    let file = File::open(input_path)
        .map_err(|error| format!("failed to open {}: {error}", input_path.display()))?;
    let mut archive =
        ZipArchive::new(file).map_err(|error| format!("failed to open zip archive: {error}"))?;

    println!("format: hwpx-zip");
    println!("path: {}", input_path.display());
    println!("entries: {}", archive.len());

    for index in 0..archive.len() {
        let file = archive
            .by_index(index)
            .map_err(|error| format!("failed to read zip entry {index}: {error}"))?;
        let method = format!("{:?}", file.compression());
        println!(
            "[{index:02}] {} ({} bytes, compression={})",
            file.name(),
            file.size(),
            method
        );
    }

    if let Some(dir) = output_dir {
        extract_archive(&mut archive, dir)?;
        println!("extracted_to: {}", dir.display());
    } else if parse_semantics {
        println!("content_mode: summary");
        println!("hint: use --out <directory> to inspect raw XML alongside the semantic parse");
    } else {
        print_archive_contents(&mut archive)?;
    }

    if parse_semantics {
        let bytes = fs::read(input_path)
            .map_err(|error| format!("failed to read {}: {error}", input_path.display()))?;
        let parsed = debug_parse_hwpx(&bytes)
            .map_err(|error| format!("failed to parse HWPX semantics: {error}"))?;
        println!("\n===== semantic parse =====");
        println!("{parsed}");
    }

    Ok(())
}

fn parse_args<I>(args: I) -> Result<Option<CliOptions>, String>
where
    I: IntoIterator<Item = String>,
{
    let mut input_path: Option<PathBuf> = None;
    let mut output_dir: Option<PathBuf> = None;
    let mut parse_semantics = false;

    let mut args = args.into_iter();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--help" | "-h" => return Ok(None),
            "--out" => {
                let Some(dir) = args.next() else {
                    return Err("--out requires a directory path".to_string());
                };
                output_dir = Some(PathBuf::from(dir));
            }
            "--parse" => parse_semantics = true,
            _ if arg.starts_with("--") => {
                return Err(format!("unknown argument: {arg}\n\n{}", usage()));
            }
            _ => {
                if input_path.is_some() {
                    return Err(format!(
                        "multiple input paths provided: {} and {arg}\n\n{}",
                        input_path.as_ref().expect("checked above").display(),
                        usage()
                    ));
                }
                input_path = Some(PathBuf::from(arg));
            }
        }
    }

    let Some(input_path) = input_path else {
        return Err(usage());
    };

    Ok(Some(CliOptions {
        input_path,
        output_dir,
        parse_semantics,
    }))
}

fn print_archive_contents(archive: &mut ZipArchive<File>) -> Result<(), String> {
    println!("content_mode: console");

    for index in 0..archive.len() {
        let mut entry = archive
            .by_index(index)
            .map_err(|error| format!("failed to read zip entry {index}: {error}"))?;

        println!("\n===== {} =====", entry.name());

        if !is_console_text(entry.name()) {
            println!(
                "[binary entry omitted: {} bytes, compression={:?}]",
                entry.size(),
                entry.compression()
            );
            continue;
        }

        let entry_name = entry.name().to_string();
        let mut bytes = Vec::new();
        (&mut entry)
            .take(MAX_CONSOLE_TEXT_BYTES as u64 + 1)
            .read_to_end(&mut bytes)
            .map_err(|error| {
                format!(
                    "failed to read console contents for {}: {error}",
                    entry_name
                )
            })?;
        let truncated = bytes.len() > MAX_CONSOLE_TEXT_BYTES;
        if truncated {
            bytes.truncate(MAX_CONSOLE_TEXT_BYTES);
            trim_to_utf8_boundary(&mut bytes);
        }

        let text = String::from_utf8(bytes)
            .map_err(|error| format!("{} is not valid UTF-8 text: {error}", entry_name))?;
        print!("{}", text.replace("\r\n", "\n"));
        if truncated {
            println!(
                "\n[truncated console output for {} at {} bytes]",
                entry_name, MAX_CONSOLE_TEXT_BYTES
            );
        }

        if !text.ends_with('\n') {
            println!();
        }
    }

    Ok(())
}

fn extract_archive(archive: &mut ZipArchive<File>, output_dir: &Path) -> Result<(), String> {
    fs::create_dir_all(output_dir)
        .map_err(|error| format!("failed to create {}: {error}", output_dir.display()))?;

    for index in 0..archive.len() {
        let mut entry = archive
            .by_index(index)
            .map_err(|error| format!("failed to read zip entry {index}: {error}"))?;
        let out_path = resolve_output_path(output_dir, entry.name())?;

        if entry.name().ends_with('/') {
            fs::create_dir_all(&out_path)
                .map_err(|error| format!("failed to create {}: {error}", out_path.display()))?;
            continue;
        }

        if let Some(parent) = out_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|error| format!("failed to create {}: {error}", parent.display()))?;
        }

        let mut output = File::create(&out_path)
            .map_err(|error| format!("failed to create {}: {error}", out_path.display()))?;
        std::io::copy(&mut entry, &mut output)
            .map_err(|error| format!("failed to write {}: {error}", out_path.display()))?;

        if is_xml_like(entry.name()) {
            normalize_xml_file(&out_path)?;
        }
    }

    Ok(())
}

fn resolve_output_path(output_dir: &Path, entry_name: &str) -> Result<PathBuf, String> {
    let mut relative = PathBuf::new();
    for component in Path::new(entry_name).components() {
        match component {
            Component::Normal(part) => relative.push(part),
            Component::CurDir => {}
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => {
                return Err(format!(
                    "refusing to extract unsafe archive entry path: {entry_name}"
                ));
            }
        }
    }

    if relative.as_os_str().is_empty() {
        return Err(format!(
            "refusing to extract archive entry with empty resolved path: {entry_name}"
        ));
    }

    Ok(output_dir.join(relative))
}

fn normalize_xml_file(path: &Path) -> Result<(), String> {
    let content = fs::read_to_string(path)
        .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
    let normalized = content.replace("\r\n", "\n");
    let mut output = File::create(path)
        .map_err(|error| format!("failed to rewrite {}: {error}", path.display()))?;
    output
        .write_all(normalized.as_bytes())
        .map_err(|error| format!("failed to rewrite {}: {error}", path.display()))?;
    Ok(())
}

fn is_xml_like(name: &str) -> bool {
    name.ends_with(".xml") || name.ends_with(".hpf") || name.ends_with(".rdf")
}

fn is_console_text(name: &str) -> bool {
    is_xml_like(name) || name.ends_with(".txt") || name == "mimetype"
}

fn to_hex(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|byte| format!("{byte:02X}"))
        .collect::<Vec<_>>()
        .join(" ")
}

fn trim_to_utf8_boundary(bytes: &mut Vec<u8>) {
    while !bytes.is_empty() && std::str::from_utf8(bytes).is_err() {
        bytes.pop();
    }
}

fn usage() -> String {
    [
        "usage: cargo run -p core-rs --bin decode_hwpx -- [options] <path-to-hwpx>",
        "",
        "options:",
        "  --out <directory>  extract the archive to a directory",
        "  --parse            print the semantic HWPX parse instead of dumping raw XML to stdout",
        "  --help             show this help text",
        "",
        "examples:",
        "  cargo run -p core-rs --bin decode_hwpx -- sample.hwpx",
        "  cargo run -p core-rs --bin decode_hwpx -- --parse sample.hwpx",
        "  cargo run -p core-rs --bin decode_hwpx -- sample.hwpx --out /tmp/sample",
    ]
    .join("\n")
}

#[cfg(test)]
mod tests {
    use super::{CliOptions, parse_args, resolve_output_path, trim_to_utf8_boundary};
    use std::path::{Path, PathBuf};

    #[test]
    fn parses_help_flag_without_error() {
        assert_eq!(parse_args(["--help".to_string()]).unwrap(), None);
        assert_eq!(parse_args(["-h".to_string()]).unwrap(), None);
    }

    #[test]
    fn parses_options_before_and_after_input_path() {
        let options = parse_args([
            "--parse".to_string(),
            "sample.hwpx".to_string(),
            "--out".to_string(),
            "/tmp/out".to_string(),
        ])
        .unwrap()
        .unwrap();

        assert_eq!(
            options,
            CliOptions {
                input_path: PathBuf::from("sample.hwpx"),
                output_dir: Some(PathBuf::from("/tmp/out")),
                parse_semantics: true,
            }
        );
    }

    #[test]
    fn rejects_multiple_input_paths() {
        let error = parse_args(["a.hwpx".to_string(), "b.hwpx".to_string()]).unwrap_err();
        assert!(error.contains("multiple input paths"));
    }

    #[test]
    fn rejects_unsafe_output_paths() {
        let error = resolve_output_path(Path::new("/tmp/out"), "../evil.xml").unwrap_err();
        assert!(error.contains("unsafe archive entry path"));

        let error = resolve_output_path(Path::new("/tmp/out"), "/etc/passwd").unwrap_err();
        assert!(error.contains("unsafe archive entry path"));
    }

    #[test]
    fn trims_truncated_bytes_to_a_utf8_boundary() {
        let mut bytes = "한글".as_bytes()[..5].to_vec();
        trim_to_utf8_boundary(&mut bytes);
        assert_eq!(std::str::from_utf8(&bytes).unwrap(), "한");
    }
}
