//! The fonts Kition's kit editors serve to their engines at run time (kition-sdk engine contract E9, `addFont`),
//! as listed in `fonts/manifest.txt`. The files themselves are what a kit's web build copies; this
//! crate only reads the manifest, so the checks in `tests/` can assert on every entry.

/// One font file of the manifest.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Font {
    pub family: &'static str,
    pub style: &'static str,
    /// Relative to the repository root.
    pub file: &'static str,
    /// ISO 15924 codes of the scripts it is here for.
    pub scripts: Vec<&'static str>,
    /// SPDX identifier.
    pub licence: &'static str,
    pub licence_file: &'static str,
    pub sha256: &'static str,
    /// Where the file comes from, pinned to a commit.
    pub source: &'static str,
}

/// The manifest as text.
pub const MANIFEST: &str = include_str!("../fonts/manifest.txt");

/// Every font of the manifest, in its order; an error names the first line that does not parse.
pub fn fonts() -> Result<Vec<Font>, String> {
    MANIFEST
        .lines()
        .enumerate()
        .filter(|(_, line)| !line.trim().is_empty() && !line.starts_with('#'))
        .map(|(index, line)| {
            let fields: Vec<&str> = line.split(" | ").map(str::trim).collect();
            let [family, style, file, scripts, licence, licence_file, sha256, source] = fields[..] else {
                return Err(format!("manifest line {}: expected 8 fields, found {}", index + 1, fields.len()));
            };
            Ok(Font { family, style, file, scripts: scripts.split(',').map(str::trim).collect(), licence, licence_file, sha256, source })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn the_manifest_parses() {
        let fonts = super::fonts().unwrap();
        assert!(!fonts.is_empty());
        assert!(fonts.iter().all(|font| font.sha256.len() == 64 && font.source.starts_with("https://")));
    }
}
