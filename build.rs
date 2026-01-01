use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Copy, Debug)]
struct IconMeta {
    offset: u32,
    len: u32,
    width: u16,
    height: u16,
}

fn main() {
    println!("cargo:rerun-if-env-changed=MATERIAL_DESIGN_ICONS_DIR");
    println!("cargo:rerun-if-changed=build.rs");

    // When the external packed icon crate is enabled, skip build-time embedding.
    // This avoids requiring a local `material-design-icons` checkout for builds.
    if std::env::var_os("CARGO_FEATURE_EXTERNAL_ICONS").is_some() {
        return;
    }

    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let blob_path = out_dir.join("material_design_icons_rgba.bin");
    let rs_path = out_dir.join("material_design_icons.rs");

    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());

    let icons_repo_dir = find_icons_repo_dir(&manifest_dir);
    let platform_root = find_platform_root(&icons_repo_dir);

    let mut blob: Vec<u8> = Vec::new();

    // category -> icon -> meta
    let mut icons_by_category: BTreeMap<String, BTreeMap<String, IconMeta>> = BTreeMap::new();
    // icon-name -> first-seen fully-qualified path "category/icon".
    let mut first_icon_path_for_name: BTreeMap<String, String> = BTreeMap::new();

    // We only embed the baseline "materialicons" set, 48px, black.
    // The pixel data is normalized to solid-white RGB + original alpha so tinting works.
    for category_entry in fs::read_dir(&platform_root)
        .unwrap_or_else(|e| panic!("Failed to read icon categories under {:?}: {e}", platform_root))
    {
        let category_entry = category_entry.expect("Failed to read directory entry");
        let category_path = category_entry.path();
        if !category_path.is_dir() {
            continue;
        }

        let Some(category_name) = category_path.file_name().and_then(|s| s.to_str()) else {
            continue;
        };

        for icon_entry in fs::read_dir(&category_path)
            .unwrap_or_else(|e| panic!("Failed to read icons under {:?}: {e}", category_path))
        {
            let icon_entry = icon_entry.expect("Failed to read directory entry");
            let icon_path = icon_entry.path();
            if !icon_path.is_dir() {
                continue;
            }

            let Some(icon_name) = icon_path.file_name().and_then(|s| s.to_str()) else {
                continue;
            };

            if let Some(png_path) = pick_baseline_png_48(&icon_path) {
                let (rgba, width, height) = load_png_as_rgba8_whitened(&png_path)
                    .unwrap_or_else(|e| panic!("Failed decoding {:?}: {e}", png_path));

                let offset = blob.len() as u32;
                blob.extend_from_slice(&rgba);
                let len = rgba.len() as u32;

                icons_by_category
                    .entry(category_name.to_string())
                    .or_default()
                    .insert(
                        icon_name.to_string(),
                        IconMeta {
                            offset,
                            len,
                            width,
                            height,
                        },
                    );

                first_icon_path_for_name
                    .entry(icon_name.to_string())
                    .or_insert_with(|| format!("{category_name}/{icon_name}"));
            }
        }
    }

    fs::write(&blob_path, &blob).unwrap_or_else(|e| panic!("Failed writing {:?}: {e}", blob_path));

    let generated_rs = generate_rust_module(&icons_by_category, &first_icon_path_for_name);
    fs::write(&rs_path, generated_rs)
        .unwrap_or_else(|e| panic!("Failed writing {:?}: {e}", rs_path));
}

fn find_icons_repo_dir(manifest_dir: &Path) -> PathBuf {
    if let Ok(env) = std::env::var("MATERIAL_DESIGN_ICONS_DIR") {
        let p = PathBuf::from(env);
        if p.is_dir() {
            return p;
        }
        panic!("MATERIAL_DESIGN_ICONS_DIR is set but not a directory: {p:?}");
    }

    for ancestor in manifest_dir.ancestors() {
        let candidate = ancestor.join("material-design-icons");
        if candidate.is_dir() {
            return candidate;
        }
    }

    panic!(
        "Could not locate a 'material-design-icons' directory. Set MATERIAL_DESIGN_ICONS_DIR to an absolute path."
    );
}

fn find_platform_root(repo_dir: &Path) -> PathBuf {
    // The upstream repo has a single platform root directory with category folders (action, alert, ...).
    // We intentionally don't hardcode its name into this crate.
    let mut candidates: Vec<PathBuf> = Vec::new();
    for entry in fs::read_dir(repo_dir)
        .unwrap_or_else(|e| panic!("Failed to read icons repo directory {:?}: {e}", repo_dir))
    {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();
        if path.is_dir() && path.join("action").is_dir() {
            candidates.push(path);
        }
    }

    match candidates.len() {
        1 => candidates.remove(0),
        0 => panic!(
            "Could not find a platform root under {:?} containing an 'action' category folder",
            repo_dir
        ),
        _ => panic!(
            "Multiple platform roots found under {:?} (each contains an 'action' folder): {candidates:?}",
            repo_dir
        ),
    }
}

fn pick_baseline_png_48(icon_dir: &Path) -> Option<PathBuf> {
    let base = icon_dir
        .join("materialicons")
        .join("black")
        .join("res")
        .join("drawable-xxxhdpi");

    if !base.is_dir() {
        return None;
    }

    // Prefer baseline_*_black_48.png when present.
    let mut best: Option<PathBuf> = None;
    for entry in fs::read_dir(&base).ok()? {
        let entry = entry.ok()?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let Some(name) = path.file_name().and_then(|s| s.to_str()) else {
            continue;
        };

        if name.ends_with("_black_48.png") && name.starts_with("baseline_") {
            best = Some(path);
            break;
        }
        if name.ends_with("_black_48.png") {
            best = Some(path);
        }
    }

    best
}

fn load_png_as_rgba8_whitened(path: &Path) -> Result<(Vec<u8>, u16, u16), String> {
    let bytes = fs::read(path).map_err(|e| e.to_string())?;
    let mut decoder = png::Decoder::new(std::io::Cursor::new(bytes));
    // Expand indexed (palette) images and transparency into RGB/RGBA so we always
    // get a straightforward pixel buffer.
    decoder.set_transformations(png::Transformations::EXPAND | png::Transformations::STRIP_16);
    let mut reader = decoder
        .read_info()
        .map_err(|e| format!("png read_info: {e}"))?;

    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader
        .next_frame(&mut buf)
        .map_err(|e| format!("png next_frame: {e}"))?;

    let width = info.width;
    let height = info.height;

    let data = &buf[..info.buffer_size()];

    let mut rgba = match info.color_type {
        png::ColorType::Rgba => data.to_vec(),
        png::ColorType::Rgb => {
            let mut out = Vec::with_capacity((width * height * 4) as usize);
            for rgb in data.chunks_exact(3) {
                out.extend_from_slice(&[rgb[0], rgb[1], rgb[2], 255]);
            }
            out
        }
        png::ColorType::GrayscaleAlpha => {
            let mut out = Vec::with_capacity((width * height * 4) as usize);
            for ga in data.chunks_exact(2) {
                out.extend_from_slice(&[ga[0], ga[0], ga[0], ga[1]]);
            }
            out
        }
        png::ColorType::Grayscale => {
            let mut out = Vec::with_capacity((width * height * 4) as usize);
            for g in data.iter().copied() {
                out.extend_from_slice(&[g, g, g, 255]);
            }
            out
        }
        png::ColorType::Indexed => {
            // With EXPAND enabled, we should never see Indexed here.
            return Err("indexed PNG decode did not expand as expected".to_string());
        }
    };

    // Normalize to white RGB so UI tinting works (white * tint = tint).
    for px in rgba.chunks_exact_mut(4) {
        if px[3] == 0 {
            continue;
        }
        px[0] = 255;
        px[1] = 255;
        px[2] = 255;
    }

    Ok((rgba, width as u16, height as u16))
}

fn sanitize_ident(raw: &str) -> String {
    let mut out = String::new();
    for (i, ch) in raw.chars().enumerate() {
        let valid = ch == '_' || ch.is_ascii_alphanumeric();
        if i == 0 {
            if ch.is_ascii_digit() {
                out.push('_');
            }
        }
        out.push(if valid { ch } else { '_' });
    }

    if out.is_empty() {
        out.push('_');
    }

    // Avoid keywords.
    // Rust keywords and reserved identifiers that cannot be used unescaped.
    // NOTE: We generate `r#...` raw identifiers for these.
    const KEYWORDS: &[&str] = &[
        "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false",
        "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move",
        "mut", "pub", "ref", "return", "self", "Self", "static", "struct", "super",
        "trait", "true", "type", "unsafe", "use", "where", "while", "async", "await",
        "dyn", "try", "yield",
    ];
    if KEYWORDS.contains(&out.as_str()) {
        out = format!("r#{out}");
    }

    out
}

fn generate_rust_module(
    icons_by_category: &BTreeMap<String, BTreeMap<String, IconMeta>>,
    first_icon_path_for_name: &BTreeMap<String, String>,
) -> String {
    let mut out = String::new();

    out.push_str("// Generated icon table.\n");
    out.push_str("//\n");
    out.push_str("// This file is generated at compile time by build.rs.\n");
    out.push_str("// It embeds baseline 48px icons as RGBA8 (white + alpha) for tintable UI icons.\n\n");

    out.push_str("#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]\n");
    out.push_str("pub struct IconId {\n");
    out.push_str("    pub offset: u32,\n");
    out.push_str("    pub len: u32,\n");
    out.push_str("    pub width: u16,\n");
    out.push_str("    pub height: u16,\n");
    out.push_str("}\n\n");

    out.push_str("static ICON_RGBA: &[u8] = include_bytes!(concat!(env!(\"OUT_DIR\"), \"/material_design_icons_rgba.bin\"));\n\n");

    out.push_str("impl IconId {\n");
    out.push_str("    pub fn rgba(self) -> &'static [u8] {\n");
    out.push_str("        let start = self.offset as usize;\n");
    out.push_str("        let end = start + self.len as usize;\n");
    out.push_str("        &ICON_RGBA[start..end]\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");

    // Modules
    for (category, icons) in icons_by_category.iter() {
        let cat_ident = sanitize_ident(category);
        out.push_str(&format!("pub mod {cat_ident} {{\n"));
        for (icon, meta) in icons.iter() {
            let icon_ident = sanitize_ident(icon);
            out.push_str(&format!(
                "    pub const {icon_ident}: super::IconId = super::IconId {{ offset: {offset}u32, len: {len}u32, width: {w}u16, height: {h}u16 }};\n",
                offset = meta.offset,
                len = meta.len,
                w = meta.width,
                h = meta.height
            ));
        }
        out.push_str("}\n\n");
    }

    // ALL list (stable)
    out.push_str("/// All embedded icons as (\"category/icon\", IconId).\n");
    out.push_str("pub const ALL: &[(&str, IconId)] = &[\n");
    for (category, icons) in icons_by_category.iter() {
        let cat_ident = sanitize_ident(category);
        for (icon, _) in icons.iter() {
            let icon_ident = sanitize_ident(icon);
            out.push_str(&format!(
                "    (\"{category}/{icon}\", {cat_ident}::{icon_ident}),\n",
                category = category,
                icon = icon
            ));
        }
    }
    out.push_str("];\n\n");

    // Name-only lookup (first match)
    out.push_str("/// Map an icon folder name (case-insensitive) to a default IconId.\n");
    out.push_str("///\n");
    out.push_str("/// If multiple categories contain the same icon name, the first one (stable) wins.\n");
    out.push_str("pub fn by_name(name: &str) -> Option<IconId> {\n");
    out.push_str("    let key = name.trim().to_ascii_lowercase();\n");
    out.push_str("    match key.as_str() {\n");

    // Ensure stable output + no duplicates.
    let mut seen: BTreeSet<String> = BTreeSet::new();
    for (icon_name, path) in first_icon_path_for_name.iter() {
        if !seen.insert(icon_name.to_ascii_lowercase()) {
            continue;
        }
        let (category, icon) = path
            .split_once('/')
            .unwrap_or_else(|| panic!("bad icon path: {path}"));
        let cat_ident = sanitize_ident(category);
        let icon_ident = sanitize_ident(icon);
        out.push_str(&format!(
            "        \"{name}\" => Some({cat_ident}::{icon_ident}),\n",
            name = icon_name.to_ascii_lowercase(),
            cat_ident = cat_ident,
            icon_ident = icon_ident
        ));
    }
    out.push_str("        _ => None,\n");
    out.push_str("    }\n");
    out.push_str("}\n");

    out
}
