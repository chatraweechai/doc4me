# Localization parity with Microsoft Word

> **Last reviewed:** 2026-10-10 · **Last updated:** 2026-10-24 · **Change:** trivial (language and entry counts corrected, Thai proofing and digit conversion; previously minor: Thai interface catalog, complete; previously minor: Thai proofing, Thai counting list numbers, TH Sarabun New/PSK drawn at their own size, Word for Windows line spacing; previously minor: Thai documents: Sarabun default font, dictionary word breaks, Thai list numbers; previously minor: Turkish interface catalog completed, 2,325 entries, #774; previously minor: font list shows fonts by their names in the interface language, #505; previously minor: Turkish interface added; previously major: first version; catalogs measured from `crates/ui-egui/src/i18n`, Word's languages from its bundle listing) · **Target:** Microsoft Word (Microsoft 365) for Windows (measured until 2026-10-21 against Word for Mac 16.113.4; not re-measured on Windows yet)

Interface languages, script support and proofing per language. How translation works (catalog
format, adding a language, clean-room rule): `crates/ui-egui/src/i18n/mod.rs`.

**Dimension: ~27% (estimated; 25% before Polish and per-run proofing languages), 75–120 h to full
for the twelve key languages** plus ~45 h for the other ~17 languages Word ships, plus
native-speaker review (human).

## What Word ships (measured by listing the bundle)

- **30 interface languages** (31 `.lproj` folders incl. Base): ar, cs, da, de, el, en, en-GB, es,
  es-MX, fi, fr, fr-CA, he, hu, id, it, ja, ko, nl, no, pl, pt (BR), pt-PT, ru, sk, sv, th, tr,
  zh-CN, zh-TW. Arabic and Hebrew get a mirrored interface. No Hindi or Vietnamese interface on Mac.
- **120 proofing tools** (`SharedSupport/Proofing Tools`): spellers for ~50 languages, grammar
  checkers for ~20, hyphenators for ~16, thesauri for ~20.

## What Doc4Me ships (measured)

- **19 interface languages**: English plus `zh-hans`, `zh-hant`, `ja`, `uk`, `es`, `pt-br`,
  Serbian `sr`/`sr-latn` (#250), Estonian `et`, Turkish `tr`, Russian `ru`, Italian `it` (#594), German `de` (#574), Polish `pl`, Czech `cs`, Hungarian `hu`, French `fr` and Thai `th`. Catalogs have 2,040–2,458 entries
  each; Polish has every key of every other catalog except the style names and equation gallery labels only Russian, Italian, Hungarian and Turkish translate so far (a test checks it). A test (`every_tab_and_command_is_translated`) enforces that every ribbon tab, command
  label and ribbon location is translated in every language.
  `pl`, `pl-PL` and `pl_PL.UTF-8` system locales pick Polish automatically.
- Of the 675 English strings a scan extracts from `tl!(…)` calls and command labels/locations, each
  catalog covers **661 (98%)**. Some newer dialog and pane strings (Equation tab tooltips, Zotero,
  Read Aloud, Backstage greetings) aren't wrapped in `tl!` yet, so they show in English in every
  language: the true share of visible strings is **~93–95%** (estimated).
- Pending PRs: Norwegian Bokmål (#208) and Nynorsk (#210), Hebrew with RTL (#20).
- **Proofing: English, Polish and Thai** (Thai: spelling only), chosen per run from its language (`w:lang`, resolved through
  the styles; Review › Language and the status bar set it). Languages without proofing tools get
  no spelling or grammar marks (as in Word without the language's tools) instead of being flagged
  as English. Detail in [Proofing per language](#proofing-per-language).
- Document scripts: shaping via HarfRust (HarfBuzz port) for every script; bidi (UAX #9) and Arabic
  shaping tested (#207); IME on all desktop platforms (winit fixes pending, #155–#164).

## The twelve key languages

Status: `full` = every user-visible string translated and the script renders and edits correctly;
`partial`; `menus only`; `none`. Hours are to `full`, Opus 5.5 agent wall-clock, excluding the
shared items below.

| Language | Code | UI strings translated | Dialogs / tooltips / help | Script support | Native review | Status | Hours to full |
|---|---|---|---|---|---|---|---|
| English | en | source (100%) | ✅ | Latin ✅ | yes | **full** | 0 |
| Simplified Chinese (Mandarin) | zh-hans | 929 entries; 98% of extracted, ~95% of visible | most; newer panes English | CJK ✅, IME ✅; Windows UI font fixed (#248); interface font chain covers every catalog character, installed CJK font after the embedded ones when it doesn't, PingFang found on macOS 12 (#486); font list by Chinese names (宋体, 微软雅黑, #505); no vertical text | no | partial | 3–5 |
| Spanish | es | ~930; 98% / ~95% | most | Latin ✅ | no | partial | 1–2 |
| Hindi | hi | 0 | ❌ | Devanagari shaping via HarfRust, untested; no hyphenation | no | none | 4–6 |
| Arabic | ar | 0 | ❌ | shaping, joining, lam-alef, bidi ✅ (#207); no kashida; **no mirrored UI** | no | none | 6–8 |
| French | fr | 2,033 entries | most; newer dialogs partly English | Latin ✅ | no | partial | 1–2 |
| Portuguese | pt-br (pt-PT none) | ~930; 98% / ~95% (pt-BR) | most | Latin ✅ | no | partial | 2–4 (pt-BR finish + pt-PT) |
| Indonesian | id | 0 | ❌ | Latin ✅ | no | none | 3–4 |
| Japanese | ja | ~930; 98% / ~95% | most | CJK ✅, IME ✅; no vertical text, ruby or document grid | no | partial | 3–5 |
| German | de | 2,021 entries; same key set as zh-hans (#574); 98% of extracted, ~95% of visible | most | Latin ✅ | no | partial | 1–2 |
| Korean | ko | 0 | ❌ | Hangul ✅; macOS Korean IME fixes pending (#163, #164) | no | none | 3–5 |
| Vietnamese | vi | 0 | ❌ | Latin with stacked diacritics, untested | no | none | 3–4 |

Other shipped languages: **13** — Traditional Chinese (`zh-hant`), Ukrainian (`uk`), Serbian
Cyrillic and Latin (`sr`, `sr-latn`, #250), Estonian (`et`), Turkish (`tr`), Russian (`ru`), Italian (`it`, #594), Polish (`pl`), Czech (`cs`), Hungarian (`hu`) and French (`fr`), all partial, and Thai (`th`, complete; below).
Estonian uses the existing fonts, follows `et-EE` system locales, and is available in Options or
through `ui.language` with `{"value":"et"}`. Its 1,307 entries cover the existing interface catalog.
Estonian spelling, grammar, dates and document templates are not added by the interface catalog.
Turkish works the same way (`tr-TR` and `tr-CY` system locales, `{"value":"tr"}`) with all 2,325
entries, the same keys as the Russian catalog. Command search folds the Turkish dotted and dotless I (`italik` finds İtalik, `YAZI TİPİ`
finds Yazı Tipi) while English labels and ASCII command ids keep their usual matching. Turkish
proofing is not included; terminology and checks: [`localization-tr.md`](localization-tr.md).
Hungarian follows `hu-HU` system locales and `{"value":"hu"}`, with a complete catalog of 2,419 entries: every key of
the Russian catalog, built-in style names shown translated, plus the citation style names. Hungarian proofing is not included; terminology and conventions:
[`localization-hu.md`](localization-hu.md).

Polish and Thai are the only ones besides English with proofing tools (Thai: spelling only), and a
Polish interface writes new documents in Polish (`pl-PL`).

**Thai (`th`)**: a complete interface catalog (`th.tsv`, 2,458 entries, every key of the other catalogs: ribbon, dialogs, messages, built-in style names, equation gallery), chosen by `th`, `th-TH` and `th_TH.UTF-8` system locales or in File › Options; written from a 903-term glossary and a style guide (commands are verbs, labels nouns, Royal Society loanword spelling, ฟอนต์ for Font, one space between Thai and Latin or digits, Arabic digits in fields). New documents keep US English as their editing language, since Thai text is proofed whatever its language. Not yet reviewed by a native-speaking professional translator. Home › Font has Convert to Thai Digits and Convert to Arabic Digits (0–9 ↔ ๐–๙ in the selection; with Track Changes on, a revision). Thai documents: Sarabun (OFL, bundled) is the default
document and interface font, so Thai draws the same everywhere, shaped through HarfRust with its
mark stacking (`layout::tests_complex::thai_is_drawn_with_the_bundled_sarabun_by_default`). Thai,
Lao and Khmer lines wrap between dictionary words (ICU4X, `doc::cluster::dictionary_breaks`), and
double-click, Ctrl+arrows and the word count use the same word boundaries; tone marks and ์ stay
inside their word. TH Sarabun New, TH SarabunPSK and TH SarabunIT๙ are drawn with Sarabun when not
installed, scaled to their letter height and set on their own advance widths, kerning and 1.331 em
line spacing, so lines and pages break as with the real font (`fonts::thsarabun`; measured for TH Sarabun New and
PSK, inferred for IT๙, which reuses TH Sarabun New's metrics); the other Thai
document fonts are drawn with Sarabun at its own size. Single line spacing follows Word for Windows
(OS/2 win metrics plus external leading: Sarabun 1.853 em). Thai digits, ฿ and
Thai/Khmer punctuation stay with the word before them (no line starts with ๒ or ។). A document made
here is saved to .docx with its complex-script font, size and weight spelled out (`w:cs`, `w:szCs`,
`w:bCs`, `w:iCs`), so Word shows its Thai text the same; a .docx from Word keeps its own. List
labels draw Thai digits (๑ ๒ ๓), letters (ก ข ค ง, without ฃ ฅ ฆ) and counting words (หนึ่ง สอง …
สิบเอ็ด ยี่สิบเอ็ด … หนึ่งร้อยเอ็ด), all three offered in the Numbering menu and Define New
Multilevel List.
Thai Distributed alignment spreads full lines between characters. Text boxes in the interface
leave room for tone marks raised over upper vowels (ที่, น้ำ). Thai proofing marks only what is
certainly wrong (see the table below).

## Proofing per language

| Language | Spelling | Suggestions | Grammar | Hyphenation |
|---|---|---|---|---|
| English (`en`, any region; text with no language) | Moby word list (public domain, ~160k words) with inflection rules | edit distance | repeated word, `a`/`an`, sentence capital, space before punctuation, double space | Moby dictionary, our Liang patterns, heuristic |
| Polish (`pl`) | The Hunspell `pl_PL` dictionary installed on the system (OS package, Homebrew, LibreOffice, or the `DOC4ME_DICTIONARIES` folder; not bundled, so no Polish spelling marks without it), read at runtime by a Hunspell-style affix engine (`crates/proof/src/affix.rs`); names capitalized, compounds part by part, `Kennedy'ego` | missing diacritics first (`zolw` → `żółw`), the dictionary's confusion table (`żeka` → `rzeka`, `chuśtawka` → `huśtawka`), missing space (`napewno` → `na pewno`), one typo, then a stem scan within two typos | Polish messages: repeated word, space before punctuation, double space, sentence capital (knows `np.`, `r.`, initials, ordinals, dialogue dashes), missing comma before `że`/`który`/`ponieważ`/`gdyż`/`aby`/`żeby`/`jeśli`/`jeżeli`/`gdyby`/`lecz` (not after `mimo`, `chyba`, `tak`, prepositions…), `we`/`ze` (`we wtorek`, `ze szkoły`), doubled punctuation | TeX Polish patterns (Kołodziejska, Jackowski, Ryćko; MIT option) with their 20 exception words; a Polish syllable heuristic for letters outside the alphabet |
| Thai (any Thai text, whatever the run's language) | Precision first: characters typed in an order no Thai word has (เเ for แ, ํา for ำ, a tone mark or ์ before the vowel, a tone mark after ำ, a doubled mark, two tone marks) and common misspellings marked only when ICU4X's dictionary sees them as a whole word (`assets/spelling/th-misspellings.tsv`: อนุญาติ, กระเพรา, อีเมล์…); unknown words are never marked | the word typed in order; the Royal Society spelling | — | — (Thai isn't hyphenated; lines wrap between words) |
| Any other | — (no marks) | — | — | the English patterns, as before |

Measured with the SJP.PL dictionary: every one of the ~4.5 million forms an independent expansion of its affix file
produces is accepted, and 200,000 one-edit mutations that aren't words are all rejected; a check
takes ~1 µs, a suggestion list 1–35 ms (release build). Not yet: other languages' dictionaries,
a thesaurus for Polish, Word's style and clarity refinements, Polish-specific AutoCorrect.

Italian (`it`, #594) follows `it`, `it-IT` and `it-CH` system locales. Its catalog has every key of
the others plus 51 strings that are wrapped in `tl!` but were in no catalog yet (File › Info
properties, the File › Home greeting, list levels, Equation tab gallery and tooltips). Ribbon tabs,
groups and command names use the names Italian Word users know (feature names, as `AGENTS.md`
allows); all other text is written from the meaning of the English. No Italian proofing yet.
## Shared work (not in the per-language hours)

| Item | Hours | Notes |
|---|---|---|
| Wrap the remaining English literals in `tl!` and translate them in every catalog | 4–6 | Makes the six partial languages full except script items |
| Mirrored (right-to-left) interface for Arabic and Hebrew | 15–25 | Ribbon, panes, dialogs, rulers |
| Vertical text, ruby, document grid (Chinese, Japanese, Korean) | counted in [layout-parity.md](layout-parity.md) | |
| Proofing dictionaries, grammar and hyphenation for the 11 non-English key languages | 20–40 | Open dictionaries only (licence check per dictionary); issues #25, #40, #100. Per-run languages, the Hunspell-style affix engine and the TeX pattern reader landed with Polish, so a language with an open Hunspell dictionary and hyphenation patterns is mostly data plus grammar rules |
| Native-speaker review of every catalog | human | None reviewed yet |

## Revision history

| Date | Change | Summary |
|---|---|---|
| 2026-10-19 | minor | Thai interface catalog (2,433 entries, complete); Thai misspelling list grown to 105 pairs; review fixes (Thai/Latin words split for spelling, user dictionary clears typing-order marks) |
| 2026-10-16 | minor | Thai proofing (typing-order mistakes and common misspellings, Royal Society spellings suggested), Thai counting list numbers and Thai list kinds in the Numbering menu, TH Sarabun New/PSK/IT๙ drawn with Sarabun at their own size, widths and line spacing, Word for Windows single line spacing |
| 2026-10-15 | minor | Thai: digits and punctuation stay with their word, Thai letters skip ฆ (ก ข ค ง), complex-script values spelled out only when saving a document made here, interface text boxes keep raised tone marks |
| 2026-10-14 | minor | Thai documents: bundled Sarabun as the default font, dictionary word breaks for Thai, Lao and Khmer (lines, selection, word count), Thai list numbers, complex-script defaults for Word |
| 2026-10-11 | minor | Turkish interface catalog completed (2,325 entries, the same keys as Russian): equation gallery, built-in style names, watermarks, paper sizes and preview tiles; the interface audit checks Turkish too (#774) |
| 2026-10-11 | minor | Czech (`cs`) interface catalog added |
| 2026-10-11 | minor | Polish interface (13 languages) and Polish proofing: SJP.PL spelling with suggestions, Polish grammar rules, TeX Polish hyphenation; proofing language per run; dimension ~25% → ~27% |
| 2026-10-11 | minor | German interface catalog (#574): 2,021 entries covering the full zh-hans key set; locale, saved-preference and bundled-font tests; no German proofing |
| 2026-10-11 | minor | Italian interface catalog (#594), locale selection, saved preference and font coverage checks; count labels put the noun first so they read right at 1 |
| 2026-10-11 | minor | Font dialog sets the Asian text font (`w:eastAsia`) and the Latin text font separately; the Asian list holds the installed Chinese, Japanese and Korean fonts by their interface-language names (#505) |
| 2026-10-11 | minor | French interface catalog (2,033 entries, @pechnikov2026-star), `fr-*` system locales, saved preference and font coverage checks; no French proofing |
| 2026-10-11 | minor | Font list shows installed fonts by their names in the interface language (宋体 for SimSun in Chinese, Japanese and Korean names likewise) and finds them by either name; a Chinese, Japanese or Korean font is stored as `w:eastAsia` too (#505) |
| 2026-10-11 | minor | Chinese/Japanese interface fonts: a test checks every catalog character has a glyph; an installed CJK font follows the embedded faces whenever they miss one; macOS font asset folders searched for PingFang (#486) |
| 2026-10-10 | minor | Russian interface (#300; 1,524 entries, built-in style names shown translated; covers everything the interface audit finds) |
| 2026-10-11 | minor | Hungarian interface catalog (2,419 entries, built-in style names shown translated; covers everything the interface audit finds), locale selection, saved preference and font coverage checks |
| 2026-10-11 | minor | Turkish interface catalog (1,307 entries), locale selection, saved preference, Turkish casing in command search and font coverage checks; catalog sizes re-counted |
| 2026-10-11 | trivial | English spelling (#520): our supplement `assets/spelling/en-extra.txt` adds ~220 common modern words the Moby list lacks ("desktop", "download", "email"…) |
| 2026-10-10 | minor | Estonian interface catalog, locale selection, saved preference and font coverage checks; count labels put the noun first so they read right at 1 |
| 2026-10-10 | minor | Serbian (Cyrillic and Latin) interface merged (#250) |
| 2026-10-10 | major | First version: twelve-language table, catalog coverage measured, Word's 30 UI languages and 120 proofing tools listed |
