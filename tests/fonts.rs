//! Assertions on every font file: checksum, licence, names, and the characters it is here for.

use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read(file: &str) -> Vec<u8> {
    std::fs::read(root().join(file)).unwrap_or_else(|error| panic!("{file}: {error}"))
}

/// Licences the fonts may have: open, and none of them Adobe's (see README.md).
const LICENCES: &[&str] = &["Apache-2.0", "OFL-1.1", "MIT"];

#[test]
fn every_file_matches_its_checksum_and_has_its_licence() {
    for font in kition_fonts::fonts().unwrap() {
        let digest = Sha256::digest(read(font.file));
        let hex: String = digest.iter().map(|byte| format!("{byte:02x}")).collect();
        assert_eq!(hex, font.sha256, "{}", font.file);
        assert!(LICENCES.contains(&font.licence), "{}: {}", font.file, font.licence);
        assert!(root().join(font.licence_file).is_file(), "{}", font.licence_file);
        assert!(Path::new(font.file).starts_with("fonts"), "{}", font.file);
    }
}

/// Fonts whose licence description (name 13) states no licence, each with exactly what it states
/// instead; their licence is the licence file beside them, from the same source.
///
/// NanumGothic's reads "NHN Corporation", its licensor; NHN's OFL.txt from google/fonts and Naver's
/// download page (https://hangeul.naver.com/font) give the licence. The owner approved this
/// exception on 2026-11-17.
const UNSTATED_LICENCE_EXCEPTIONS: &[(&str, &str)] = &[("fonts/nanum-gothic/NanumGothic-Regular.ttf", "NHN Corporation")];

/// The font's own licence description (name 13) agrees with the manifest.
#[test]
fn every_font_states_the_licence_it_is_listed_with() {
    for font in kition_fonts::fonts().unwrap() {
        let bytes = read(font.file);
        let face = ttf_parser::Face::parse(&bytes, 0).unwrap();
        let stated: Vec<String> = face.names().into_iter().filter(|name| name.name_id == 13).filter_map(|name| name.to_string()).collect();
        if let Some((_, instead)) = UNSTATED_LICENCE_EXCEPTIONS.iter().find(|(file, _)| *file == font.file) {
            // Exactly what the exception says, so it never hides another statement.
            assert!(!stated.is_empty() && stated.iter().all(|text| text == instead), "{}: {stated:?}", font.file);
            continue;
        }
        let expected = match font.licence {
            "Apache-2.0" => "Apache License",
            "OFL-1.1" => "SIL Open Font License",
            other => other,
        };
        assert!(stated.iter().any(|text| text.contains(expected)), "{}: {stated:?}", font.file);
    }
}

/// Names that may credit Adobe, each for one file only, by owner decision. Every other name of
/// that file, and every name of every other file, is still checked.
///
/// Shippori Mincho (owner decision 2026-11-17, an explicit exception to the README's no-Adobe rule):
/// its kanji come from Genryu Mincho, a derivative of Adobe's Source Han Serif, so its manufacturer
/// name credits Adobe. ATTRIBUTION.md states that lineage.
const ADOBE_NAME_EXCEPTIONS: &[(&str, &str)] = &[("fonts/shippori-mincho/ShipporiMincho-Regular.ttf", "FONTDASU / Google Inc. / but / Adobe")];

#[test]
fn every_font_has_the_family_it_is_listed_as_and_no_adobe_name() {
    for font in kition_fonts::fonts().unwrap() {
        let bytes = read(font.file);
        let face = ttf_parser::Face::parse(&bytes, 0).unwrap();
        let names: Vec<String> = face.names().into_iter().filter_map(|name| name.to_string()).collect();
        assert!(names.iter().any(|name| name == font.family), "{}: {names:?}", font.file);
        let excepted = |name: &String| ADOBE_NAME_EXCEPTIONS.contains(&(font.file, name.as_str()));
        let joined = names.iter().filter(|name| !excepted(name)).cloned().collect::<Vec<_>>().join(" ").to_lowercase();
        for banned in ["adobe", "source han", "noto sans cjk", "noto serif cjk"] {
            assert!(!joined.contains(banned), "{} names {banned}", font.file);
        }
    }
}

/// An exception names a file of the manifest and a name that file really has, so none outlives
/// its font.
#[test]
fn every_adobe_name_exception_is_a_name_of_a_listed_font() {
    let fonts = kition_fonts::fonts().unwrap();
    for (file, name) in ADOBE_NAME_EXCEPTIONS {
        assert!(fonts.iter().any(|font| font.file == *file), "{file} is not in the manifest");
        let bytes = read(file);
        let face = ttf_parser::Face::parse(&bytes, 0).unwrap();
        assert!(face.names().into_iter().filter_map(|n| n.to_string()).any(|n| n == *name), "{file} has no name {name:?}");
    }
}

/// Characters each script must have, from everyday text.
fn sample(script: &str) -> &'static str {
    match script {
        "Hans" => "北极光的成因说明这里简体中文汉字测试们这样更好宣传东区西区南区",
        "Hant" => "繁體中文測試這裡說明們極光東區",
        "Jpan" => "あいうえおアイウエオ日本語ｱｲｳ",
        "Latn" => "The quick brown fox jumps over the lazy dog 0123456789 ÀÉÎÕÜßçñøå œŁłŞş “”‘’–—…€",
        "Cyrl" => "Съешь же ещё этих мягких французских булок, да выпей чаю ЁЇЄ",
        "Grek" => "Ξεσκεπάζω την ψυχοφθόρα βδελυγμία ΑΒΓΔΩ",
        // Arabic: every letter of the alphabet, Arabic-Indic digits, comma and question mark, and
        // Persian and Urdu letters.
        "Arab" => "ابتثجحخدذرزسشصضطظعغفقكلمنهوية ءآأؤإئ ٠١٢٣٤٥٦٧٨٩ ،؟ پچژگکی ٹڈڑںے",
        // Hangul: a pangram, and compatibility jamo (every syllable is checked below).
        "Hang" => "다람쥐 헌 쳇바퀴에 타고파 한국어 ㄱㄴㄷㄹㅁㅂㅅㅇㅈㅊㅋㅌㅍㅎ ㅏㅑㅓㅕㅗㅛㅜㅠㅡㅣ",
        // Emoji: faces, hands, objects, symbols, and the regional indicators flags are made of (no
        // variation selectors: they select a glyph, they have none).
        "Zsye" => "😀😂🥰👍👋🎉🚀✅❌⭐☕🔥💡📌🗂🇨🇳🇺🇸",
        other => panic!("no sample for script {other}"),
    }
}

/// Sample characters a font may lack, each for one file and script only, by owner decision; another
/// face listed for the script must have them.
///
/// Shippori Mincho has no half-width katakana (U+FF61 to U+FF9F); BIZ UDMincho has them, and
/// Kition PDF draws them with it. The owner approved this exception on 2026-11-17.
const MISSING_SAMPLE_EXCEPTIONS: &[(&str, &str, &str)] = &[("fonts/shippori-mincho/ShipporiMincho-Regular.ttf", "Jpan", "ｱｲｳ")];

#[test]
fn every_font_covers_the_scripts_it_is_listed_for() {
    let fonts = kition_fonts::fonts().unwrap();
    for font in &fonts {
        let bytes = read(font.file);
        let face = ttf_parser::Face::parse(&bytes, 0).unwrap();
        for script in &font.scripts {
            let missing: String = sample(script).chars().filter(|&c| face.glyph_index(c).is_none()).collect();
            let allowed = MISSING_SAMPLE_EXCEPTIONS.iter().find(|(file, s, _)| *file == font.file && s == script).map_or("", |(_, _, chars)| *chars);
            // Exactly the excepted characters: an exception never hides another gap, nor outlives one.
            assert_eq!(missing, allowed, "{} lacks {script} characters {missing}", font.file);
        }
    }
    for (file, script, chars) in MISSING_SAMPLE_EXCEPTIONS {
        let others = fonts.iter().filter(|font| font.file != *file && font.scripts.contains(script));
        let covered = others.into_iter().any(|font| {
            let bytes = read(font.file);
            let face = ttf_parser::Face::parse(&bytes, 0).unwrap();
            chars.chars().all(|c| face.glyph_index(c).is_some())
        });
        assert!(covered, "no other {script} face has {chars}, which {file} lacks");
    }
}

/// Simplified Chinese: every character of GB 2312's first level (the 3,755 most common).
#[test]
fn the_simplified_chinese_faces_cover_gb2312_level_one() {
    // gb2312-level1.txt lists them, rows 16 to 55 of GB 2312 decoded to Unicode.
    let level_one = include_str!("gb2312-level1.txt");
    let chars: Vec<char> = level_one.chars().filter(|c| !c.is_whitespace()).collect();
    assert_eq!(chars.len(), 3755);
    for font in kition_fonts::fonts().unwrap().into_iter().filter(|font| font.scripts.contains(&"Hans")) {
        let bytes = read(font.file);
        let face = ttf_parser::Face::parse(&bytes, 0).unwrap();
        let missing: String = chars.iter().filter(|&&c| face.glyph_index(c).is_none()).collect();
        assert!(missing.is_empty(), "{} lacks {missing}", font.file);
    }
}

/// Hangul: every one of the 11,172 precomposed syllables (U+AC00 to U+D7A3).
#[test]
fn the_hangul_faces_cover_every_syllable() {
    for font in kition_fonts::fonts().unwrap().into_iter().filter(|font| font.scripts.contains(&"Hang")) {
        let bytes = read(font.file);
        let face = ttf_parser::Face::parse(&bytes, 0).unwrap();
        let missing: String = ('\u{AC00}'..='\u{D7A3}').filter(|&c| face.glyph_index(c).is_none()).collect();
        assert!(missing.is_empty(), "{} lacks {missing}", font.file);
    }
}
