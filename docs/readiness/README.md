# Readiness check sets

The measured readiness number comes from these files, not from estimates. `cargo xtask readiness`
reads them, runs what can be run, and writes [`../readiness.md`](../readiness.md). The plan that
uses the number is in [`../roadmap.md`](../roadmap.md#path-to-80-measured-readiness).

## Rules

- Every check passes or doesn't. A check that wasn't run, or a hand check marked `untested`,
  doesn't pass.
- A dimension's score is the mean of its check sets' pass rates; the number is the weighted sum
  of the dimensions, with the weights of [`../target-app-parity.md`](../target-app-parity.md):

  | Dimension | Weight | Check sets |
  |---|---|---|
  | Features | 0.30 | `ribbon` (the Word ribbon catalog, `cargo xtask parity`), `dialogs.tsv` |
  | File formats | 0.20 | corpus: opens; text and structure survive save and reopen |
  | Layout and pagination | 0.15 | corpus: page count; first and last word of every page |
  | UI/UX | 0.10 | `shortcuts.tsv` |
  | Stability | 0.10 | corpus: no crash or hang in any step; `manual.tsv` (`stability`) |
  | Performance | 0.05 | `performance.tsv` |
  | Localization | 0.04 | `languages.tsv` |
  | Platforms | 0.03 | `manual.tsv` (`platforms`) |
  | Ecosystem | 0.02 | `manual.tsv` (`ecosystem`) |
  | Hardware | 0.01 | `manual.tsv` (`hardware`) |

- Lists are fixed: rows may be added, and a status may change with evidence, but rows are never
  removed to move the number. A row out of scope stays with `check` = `excluded` and the reason.
- The number counts as final once the corpus checks have run on at least 200 documents.

## Files

| File | What | Source | How a row passes |
|---|---|---|---|
| `shortcuts.tsv` | Word for Windows' default keyboard shortcuts, 329 rows | Microsoft's [Keyboard shortcuts in Word](https://support.microsoft.com/en-us/office/keyboard-shortcuts-in-word-95ef89dd-7142-4b50-afb2-f762f663ceb2), Windows tab, as published on 2026-10-25; rows listing several keys (the function-key table) split into one row per key | `registry`: one of the row's `chord`s (separated by `\|`, written as Doc4Me writes shortcuts, `Mod` for Ctrl) is bound to `command`, directly or, with Shift, as a caret move Shift extends; `manual`: `status` is `pass` |
| `dialogs.tsv` | Word's dialog boxes, 235 rows | Microsoft's [WdWordDialog enumeration](https://learn.microsoft.com/en-us/office/vba/api/word.wdworddialog) | `registry`: `command` (Doc4Me's equivalent dialog) exists; `manual`: `status` is `pass` |
| `languages.tsv` | Microsoft 365 Apps' interface languages, 95 rows | Microsoft's [languages, culture codes and companion proofing languages](https://learn.microsoft.com/en-us/microsoft-365-apps/deploy/overview-deploying-languages-microsoft-365-apps) | `catalog` names a file in `crates/ui-egui/src/i18n` holding at least 95% of the interface strings any catalog holds (English always passes) |
| `performance.tsv` | Timing checks on generated documents (about 20, 100 and 500 pages of English text), and typing and scrolling in the window | Fixed limits | The median of three runs is within `threshold_ms`; rows measured `manual` are checked by hand in the app (`status`) |
| `manual.tsv` | Stability, platform, ecosystem and hardware checks done by hand | Gaps, hardware parity | `status` is `pass`, with `date` and `evidence` |
| `corpus.tsv` | Real-world documents with a reference PDF made by Microsoft Word | GOV.UK publications (Open Government Licence v3.0) and Zenodo records under a Creative Commons licence that publish the same document as `.docx` and as a PDF (on Zenodo, files of the same name) whose Producer or Creator is Microsoft Word (or Acrobat PDFMaker for Word, which also prints through Word's layout), with at least 90% of their words in common, 1–80 pages; the licence is in each row | See the corpus checks below |

How a mapping was decided is in each row's `note`. Hand checks name their evidence; when the
published shortcut list contradicts Word's long-standing key (subscript, the hyphens, Collapse),
the row follows the published list and says so in its note, to be confirmed on Word for Windows.
Excluded: Word for Mac's page-setup dialogs (the target is Word for Windows), Information Rights
Management and the Outlook address book (Microsoft services) and the VBA editor (out of scope).

## The corpus

`cargo xtask readiness --download` fetches every listed file into `corpus/readiness/` (never
committed) and keeps only files whose SHA-256 matches `corpus.tsv`. `cargo xtask readiness
--corpus` then runs `doc4me-cli` on each document, one process per step with a three-minute
timeout:

| Set | Passes when |
|---|---|
| `corpus-open` | `doc4me-cli info` opens and lays it out |
| `corpus-roundtrip` | after `doc4me-cli convert` to `.docx`, the words of `doc4me-cli text` are unchanged, and so is the document's make-up from `doc4me-cli inspect`: the number of blocks, objects and runs of each type and style (paragraphs, tables, pictures, fields, bookmarks, shapes, paragraph and character styles), of parts of each kind (headers, footers, text boxes, notes, comments) and of sections |
| `corpus-pages` | the PDF `doc4me-cli convert` makes has as many pages as Word's (`pdfinfo`) |
| `corpus-breaks` | every page starts and ends with the same word as Word's page (`pdftotext`, letters and digits only) |
| `corpus-stability` | no step panicked, was killed or ran out of time |

Without `--corpus` the corpus sets count as not run, so the number stays low until they run.
