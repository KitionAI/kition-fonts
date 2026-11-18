# Kition Fonts

Kition Fonts: the open fonts Kition's kit editors serve to their engines.

[Kition](https://kition.ai) · [Developer docs](https://developer.kition.ai)

A kit's web build copies the files it needs from a checkout of this repository next to the kit,
`../kition-fonts` (or `KITION_FONTS_DIR`), and its editor hands them to the engine's `addFont`
before any document opens (rule E9 of the kition-sdk
[engine contract](https://github.com/KitionAI/kition-sdk/blob/main/docs/platform/kits/engine.md)).
A web build has no system fonts, so without them text in these scripts shows as boxes.

## Fonts

| Family | Styles | For | License |
| --- | --- | --- | --- |
| Droid Sans Fallback | Regular | Simplified and Traditional Chinese, Japanese kana (sans) | [Apache-2.0](fonts/droid-sans-fallback/LICENSE) |
| Inter | Regular (variable: weight, optical size) | Latin, Cyrillic, Greek (sans) | [OFL-1.1](fonts/inter/OFL.txt) |
| Lora | Regular (variable: weight) | Latin, Cyrillic (serif) | [OFL-1.1](fonts/lora/OFL.txt) |
| JetBrains Mono | Regular (variable: weight) | Latin, Cyrillic (monospace) | [OFL-1.1](fonts/jetbrains-mono/OFL.txt) |
| Bricolage Grotesque | Regular (variable: weight, width, optical size) | Latin (display sans) | [OFL-1.1](fonts/bricolage-grotesque/OFL.txt) |
| Carlito | Regular, Bold, Italic, Bold Italic | Latin; metric-compatible with Calibri | [OFL-1.1](fonts/carlito/OFL.txt) |
| Caladea | Regular, Bold, Italic, Bold Italic | Latin; metric-compatible with Cambria | [OFL-1.1](fonts/caladea/OFL.txt) |
| Arimo | Regular, Italic (variable: weight) | Latin; metric-compatible with Arial | [OFL-1.1](fonts/arimo/OFL.txt) |
| Tinos | Regular, Bold, Italic, Bold Italic | Latin; metric-compatible with Times New Roman | [OFL-1.1](fonts/tinos/OFL.txt) |
| Cousine | Regular, Bold, Italic, Bold Italic | Latin; metric-compatible with Courier New | [OFL-1.1](fonts/cousine/OFL.txt) |
| Noto Color Emoji | Regular (COLRv1: color layers and gradients) | Emoji, in color | [OFL-1.1](fonts/noto-color-emoji/OFL.txt) |
| Ma Shan Zheng | Regular | Simplified Chinese, all of GB 2312 (Kai: brush regular script, heavy) | [OFL-1.1](fonts/ma-shan-zheng/OFL.txt) |
| BIZ UDPGothic | Regular | Japanese (sans, proportional kana; Gothic) | [OFL-1.1](fonts/biz-udpgothic/OFL.txt) |
| BIZ UDMincho | Regular | Japanese, with half-width katakana (serif; Mincho) | [OFL-1.1](fonts/biz-udmincho/OFL.txt) |
| Shippori Mincho | Regular | Japanese document text, without half-width katakana (old-style serif; Mincho) | [OFL-1.1](fonts/shippori-mincho/OFL.txt) |
| Noto Sans Arabic | Regular (variable: weight, width) | Arabic, Persian, Urdu (sans) | [OFL-1.1](fonts/noto-sans-arabic/OFL.txt) |
| NanumGothic | Regular | Korean Hangul, all 11,172 syllables; no Hanja (sans) | [OFL-1.1](fonts/nanum-gothic/OFL.txt) |

Each font's license file sits next to it in `fonts/<family>/`; Droid Sans Fallback also keeps its
upstream `NOTICE`. `fonts/manifest.txt` lists every file, one per line, with its family, style,
scripts, license, license file, SHA-256 and pinned source (its header comment names the fields).
[ATTRIBUTION.md](ATTRIBUTION.md) names each font's authors and the exact source it came from.

The metric-compatible families stand in for fonts documents name but the kits may not ship:
each has its original's advance widths, so text breaks into the same lines.

Ma Shan Zheng is the only Chinese Kai (楷体) face here. No Chinese Song (宋体) or FangSong (仿宋) face
meets the rules below. LXGW WenKai and Zhuque Fangsong are left out because their READMEs say they
draw on Source Han letterforms. Hanazono Mincho's OFL option reserves its name, and its file does not
state a license. Google Fonts' only Simplified Chinese serif is Noto Serif SC (Source Han Serif).

The Japanese faces are BIZ UDPGothic and BIZ UDMincho, drawn by Type Bank and Morisawa, and
Shippori Mincho, drawn by FONTDASU. Shippori Mincho is the one exception to the no-Adobe rule below
(owner decision, 2026-11-17): its kana are FONTDASU's own, but its kanji come from Genryu Mincho, which
is derived from Adobe's Source Han Serif. It has no half-width katakana; BIZ UDMincho has them.

The Korean face is NanumGothic (Nanum Gothic), drawn by Sandoll Communications for NHN (now Naver).
Its licence description names NHN instead of the licence, so its OFL comes from the `OFL.txt` beside
it; the licence test lists that exception by file. The OFL reserves the name "NanumGothic" (and
"Nanum"), so a modified or subset copy must be renamed.

## Rules

- Only open licenses whose terms are proven by the source: Apache-2.0, OFL-1.1, or MIT.
- Nothing from Adobe and nothing derived from Adobe's designs: no Source Han, Source Sans, or Source
  Serif, and no Noto CJK, which is Source Han renamed. Some kit repositories forbid them outright.
  The one exception is Shippori Mincho, by owner decision; the test that checks names lists it by
  file and name, and no other font may name Adobe.
- Every file has a manifest line, its license file next to it, and an attribution row.

## Build and test

```sh
cargo test
```

The tests check every file's checksum and license, its family name, that no name is Adobe's (but
for the named exception above), and that it has the characters of the scripts it is listed for (all 3,755 characters of GB 2312 level 1
for Simplified Chinese).

## License

The code is Apache-2.0 ([LICENSE](LICENSE)). Each font keeps its own license,
linked in the table above: Droid Sans Fallback is Apache-2.0, every other font is the SIL Open Font
License 1.1.
