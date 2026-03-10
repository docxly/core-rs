use std::env;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use zip::read::ZipArchive;

const ZIP_MAGIC: &[u8; 4] = b"PK\x03\x04";
const OLE_MAGIC: &[u8; 8] = b"\xD0\xCF\x11\xE0\xA1\xB1\x1A\xE1";

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let mut args = env::args().skip(1);
    let Some(input_arg) = args.next() else {
        return Err(usage());
    };

    let mut output_dir = None;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--out" => {
                let Some(dir) = args.next() else {
                    return Err("--out requires a directory path".to_string());
                };
                output_dir = Some(PathBuf::from(dir));
            }
            _ => return Err(format!("unknown argument: {arg}\n\n{}", usage())),
        }
    }

    let input_path = PathBuf::from(input_arg);
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
        decode_hwpx_zip(&input_path, output_dir.as_deref())?;
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

fn decode_hwpx_zip(input_path: &Path, output_dir: Option<&Path>) -> Result<(), String> {
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
    } else {
        print_archive_contents(&mut archive)?;
    }

    Ok(())
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

        let mut bytes = Vec::new();
        entry.read_to_end(&mut bytes).map_err(|error| {
            format!(
                "failed to read console contents for {}: {error}",
                entry.name()
            )
        })?;

        let text = String::from_utf8(bytes)
            .map_err(|error| format!("{} is not valid UTF-8 text: {error}", entry.name()))?;
        print!("{}", text.replace("\r\n", "\n"));

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
        let out_path = output_dir.join(entry.name());

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

fn usage() -> String {
    "usage: cargo run -p core-rs --bin decode_hwpx -- <path-to-hwpx> [--out <directory>]"
        .to_string()
}
