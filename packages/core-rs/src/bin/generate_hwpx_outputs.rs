use std::fs;
use std::path::{Path, PathBuf};

use core_rs::{HwpxOptions, HwpxParagraphAlign, HwpxStyleOptions, generate_hwpx};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fixtures_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("hwpx")
        .join("approved");
    let out_root = Path::new("/tmp").join("hwpx-generator-outputs");

    if out_root.exists() {
        fs::remove_dir_all(&out_root)?;
    }
    fs::create_dir_all(&out_root)?;

    let mut fixtures = fixture_dirs(&fixtures_root)?;
    fixtures.sort();

    for fixture_dir in fixtures {
        let name = fixture_dir
            .file_name()
            .and_then(|value| value.to_str())
            .ok_or("invalid fixture directory name")?;
        let input = fs::read_to_string(fixture_dir.join("input.md"))?;
        let options = read_fixture_options(&fixture_dir.join("fixture.toml"))?;
        let bytes = generate_hwpx(&input, options)?;

        let fixture_out = out_root.join(name);
        fs::create_dir_all(&fixture_out)?;
        let out_file = fixture_out.join("generated.hwpx");
        fs::write(&out_file, bytes)?;

        println!("{}", out_file.display());
    }

    Ok(())
}

fn fixture_dirs(root: &Path) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut dirs = Vec::new();
    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        if path.join("input.md").is_file() && path.join("fixture.toml").is_file() {
            dirs.push(path);
        }
    }
    Ok(dirs)
}

fn read_fixture_options(path: &Path) -> Result<HwpxOptions, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(path)?;
    let mut title = None;
    let mut author = None;
    let mut strict_mode = true;
    let mut in_style = false;
    let mut style = HwpxStyleOptions::default();

    for raw_line in contents.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if line.starts_with('[') && line.ends_with(']') {
            in_style = line == "[style]";
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let key = key.trim();
        let value = value.trim();

        if in_style {
            match key {
                "body_font" => style.body_font = Some(parse_string(value)),
                "heading_font" => style.heading_font = Some(parse_string(value)),
                "body_font_size" => style.body_font_size = Some(value.parse()?),
                "heading_font_size" => style.heading_font_size = Some(value.parse()?),
                "text_color" => style.text_color = Some(parse_string(value)),
                "heading_color" => style.heading_color = Some(parse_string(value)),
                "link_color" => style.link_color = Some(parse_string(value)),
                "paragraph_align" => {
                    style.paragraph_align = Some(parse_align(&parse_string(value))?)
                }
                _ => {}
            }
            continue;
        }

        match key {
            "title" => title = Some(parse_string(value)),
            "author" => author = Some(parse_string(value)),
            "strict_mode" => strict_mode = value.parse()?,
            _ => {}
        }
    }

    Ok(HwpxOptions {
        title,
        author,
        strict_mode,
        style,
    })
}

fn parse_string(value: &str) -> String {
    let trimmed = value.trim();
    if let Some(stripped) = trimmed.strip_prefix('"').and_then(|v| v.strip_suffix('"')) {
        stripped.to_string()
    } else {
        trimmed.to_string()
    }
}

fn parse_align(value: &str) -> Result<HwpxParagraphAlign, Box<dyn std::error::Error>> {
    match value {
        "left" => Ok(HwpxParagraphAlign::Left),
        "center" => Ok(HwpxParagraphAlign::Center),
        "right" => Ok(HwpxParagraphAlign::Right),
        "justify" => Ok(HwpxParagraphAlign::Justify),
        _ => Err(format!("unsupported paragraph_align: {value}").into()),
    }
}
