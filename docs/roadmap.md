# Doc4Me roadmap: milestones and what's next

> **Last reviewed:** 2026-11-02 · **Last updated:** 2026-11-02 · **Change:** minor (M14: Doc4Me v1.0.0, v1.1.0, v1.1.1 and v1.1.2 published on the fork `veevie55/doc4me`, v1.1.2 the latest; readiness is still 86.0% as measured on Windows on 2026-11-01 (page-count passes 162/222, page-boundary passes 87/222); a page break before an empty paragraph ending a section no longer leaves a blank page, not yet remeasured on Windows) · **Target:** Microsoft Word (Microsoft 365) for Windows (readiness measured on Windows with Word-made PDF references; direct UI parity has not yet been remeasured in Word for Windows)

Forward-looking plan. The summary is [`ROADMAP.md`](../ROADMAP.md); the ranked work list is
[`gaps.md`](gaps.md); the numbers are in [`target-app-parity.md`](target-app-parity.md). Hours are
Opus 5.5 agent wall-clock.

## Current focus (toward beta, ~150–230 h)

1. **Merge or close the ~45 open PRs** before starting overlapping work: many close catalog gaps
   (Clipboard pane #264, Manage Styles #270, Group #267), fix field bugs (Hebrew RTL #20, IME
   fixes #155–#164) or add languages (nb, nn, he). Owner review needed.
2. **DOCX real-world fidelity** (gap #1, 30–50 h): the 222-document corpus is measured for open,
   round-trip, stability and pagination; all documents preserve measured structure after save and
   reopen, but formatting still needs direct comparison with Word. Omitted/empty paragraph defaults
   also have synthetic save/reopen coverage.
3. **Preserve what we can't render** (gap #2, 10–15 h): charts, SmartArt, OLE, ink, group shapes
   survive a round trip and show their fallback pictures.
4. **Stability sweep** (gap #3, 20–30 h): the open crash, hang and write-failure issues, plus an
   autosave/recovery soak test.
5. **Pagination fidelity** (gap #4, 30–50 h): compat options (column balancing and footnote
   continuation landed), a local page-by-page comparison harness (Aptos is drawn with the bundled
   metric-compatible Intos since 2026-10-25).
6. **Formatting revisions and content controls** (gap #5, 12–18 h), **native printing** (gap #6,
   6–10 h), **dialog depth** (gap #7, 20–30 h).

## Alpha gate (core workflows)

The core-workflow gate from the craftrules progress-docs standard: what a typical Word user does
every day, checked end to end on the main platform, including save and reopen. The main platform is
now Windows desktop (Word for Windows is the target); the results below were checked on macOS
desktop in WordCraft and have not been re-run on Windows yet.
"Partial" here means rough but not blocking: the workflow completes.

| Core workflow | Works end to end? | Evidence | Hours to pass |
|---|---|---|---|
| Write and format a document: type, fonts, paragraph formatting, styles, lists, undo, find/replace; save as .docx and reopen | yes | 625 commands (2026-10-25), typing parity pinned by `tests_typing.rs`, DOCX round-trip tests (`crates/docx/tests/roundtrip.rs`) | 0 |
| Open a .docx someone sent, edit it, send back a .docx Word opens | partial (not blocking) | Word opens our files; typical text/table/picture documents work; on 222 real-world documents all open and preserve measured structure through save and reopen (formatting not compared yet); charts, SmartArt and OLE dropped silently, move tracking lost (gaps #1, #2, #5) | 0 (beta work: 50–80) |
| Build a structured document: tables, pictures, headers/footers, page numbers, sections, columns | yes | Tables with row splitting and floating tables (#137), floating pictures (#136), first/odd/even headers; columns don't balance (gap #4) | 0 |
| Review with others: comments, track changes, accept/reject, compare | yes | Margin balloons, tracked insert/delete incl. paragraph marks (#125, #244), formatting revisions (#41), compare | 0 |
| References for reports and papers: TOC, footnotes, citations and bibliography, captions | yes | Updatable TOC field (#52), notes, APA/MLA/Chicago/IEEE, Zotero (#189); long footnotes don't continue (gap #4) | 0 |
| Share or print: export PDF, print | partial (not blocking) | PDF export solid (krilla); desktop printing goes through a PDF, no native print dialog (gap #6) | 0 (beta work: 6–10) |

**Result: passes.** Every core workflow completes on macOS and the work saves and reopens; the
partial rows are fidelity gaps (beta work), not broken workflows. Doc4Me stays **alpha**.

## Milestones

| # | Milestone | State | Remaining |
|---|---|---|---|
| M0 | Skeleton and vertical slice (model, layout, render, engine, Word-style UI, CLI, MCP, web) | done | — |
| M1 | DOCX I/O | corpus measured: 222/222 open and preserve measured structure; direct formatting comparison remains | 30–50 h |
| M2 | Home tab | done (Paste Special, Clipboard pane, Style Inspector, Manage Styles and Asian Typography landed; Dictate with whisper.cpp on the computer, 2026-10-25) | — |
| M3 | Insert tab | done: equations (#191), icons (#309), charts, SmartArt, Screenshot › Screen Clipping, Online Video, 3D Models (as a picture), Online Pictures (Openverse), Get Add-ins (Doc4Me's own catalog) and Ink Equation landed | — |
| M4 | Layout and Design tabs | done except column balancing, group | 6–10 h |
| M5 | Tables | done (row splitting, floating tables, style formatting); custom table styles (#256); Draw Table and Eraser (#303) | 6–10 h |
| M6 | References | first version done (TOC fields, cross-references, Zotero); footnote continuation missing; all 12 bibliography styles (#383) | 15–25 h |
| M7 | Review | done (balloons, paragraph-mark revisions, formatting revisions #41); move tracking, multilingual proofing missing | 35–60 h |
| M8 | View | done; window commands (New Window, Arrange All, Split, View Side by Side, Synchronous Scrolling, Switch Windows) landed 2026-10-25 | — |
| M9 | Mailings | done (first version); Excel/Outlook data sources and email merge missing | 6–10 h |
| M10 | File/Backstage | mostly done; native printing, Options depth missing | 12–20 h |
| M11 | Draw and objects | text wrap, text boxes, canvas handles, rotation (#332), ink pens and eraser (#307), contour wrap (#354), lasso, Add Pen, Ink to Shape and Ink Replay (#395) done; grouping (`arrange.group`) and Ink to Math (own recogniser, 2026-10-25) done | 30–50 h |
| M12 | Formats breadth (PDF, ODT, RTF, HTML, MD, TXT, LaTeX, `.doc` import) | done (first versions); encrypted `.docx` opens and saves (agile encryption); `.doc` write, encrypted `.doc`, minor formats missing | 60–100 h |
| M13 | Performance budgets | on track (1.4 ms relayout); large real-world documents unmeasured | 10–20 h |
| M14 | 1.0 polish, packaging, signing | Doc4Me v1.0.0, v1.1.0, v1.1.1 and v1.1.2 (the latest) published on the fork [`veevie55/doc4me`](https://github.com/veevie55/doc4me/releases) on 2026-11-02, the earlier three rebuilt from their own commits (Windows x64 MSI and portable zip, unsigned; CI signs once the signing secrets are set); WordCraft, upstream, shipped v0.1.0–v0.4.0 on every platform | — |
| M15 | Localization (12 key languages, RTL interface, proofing languages) | 30/95 catalogs meet the 95% string threshold; English, Polish and Thai proofing, a proofing language per run | 75–120 h |
| M16 | Right-to-left and East Asian typography | RTL paragraphs done (#207) | 30–45 h |

## Path to 100% measured readiness

The readiness number in [`ROADMAP.md`](../ROADMAP.md) was an estimate (~60%): a weighted sum of
estimated dimension parities. Since stage B it is measured: **66.1%** at first, **73.4%** in the
2026-10-25 report, and **86.0%** on 2026-11-01 (`readiness.md`, all 222 corpus documents; DOCX open and round-trip 222/222), 14.0
points short of the target. The current score excluding 26 deferred owner-bound checks is 88.5%.
The target is **100% ready for real work, measured**
(raised from 80% to 95% by the owner on 2026-10-25, and to 100% on 2026-10-27): every dimension
comes from a fixed pass/fail check set, and one command computes the number. Checks may be added,
never removed, so the number can't drift by editing the list.

**A. Build the measurement first** (done 2026-10-25: `cargo xtask readiness`, check sets in
[`readiness/`](readiness/README.md), 222 corpus documents; result in
[`readiness.md`](readiness.md)):

1. `cargo xtask readiness`: reads every check set's results and computes the weighted pass rate
   with the existing weights (features 0.30, file formats 0.20, layout 0.15, UI/UX 0.10,
   stability 0.10, performance 0.05, localization 0.04, platforms 0.03, ecosystem 0.02, hardware
   0.01); writes `docs/readiness.md`, whose number replaces the estimate in `ROADMAP.md`.
2. Real-world DOCX corpus: at least 200 files that may be reused (public-domain government
   documents and openly licensed ones), each with a reference PDF that Microsoft Word produced
   (PDF Producer "Microsoft Word"), so no Word installation is needed. Files stay out of git
   (`/corpus`); a manifest in the repo lists URL, licence and SHA-256.
3. File-format checks per corpus file: opens without error; save and reopen keep text, styles,
   tables, pictures and unknown parts.
4. Pagination checks against the reference PDFs: page count, and the first and last words of
   every page.
5. Stability checks: open, lay out and save every corpus file under a timeout with no panic or
   hang; the autosave soak test; the existing hostile-parameter fuzzing.
6. Feature and UI checklist: a fixed table of Word for Windows features, dialogs and options and
   Microsoft's published default shortcut list, each row tied to a test or a manual check on
   Windows.
7. Performance thresholds (open a 500-page document, typing latency, scrolling), localization
   (catalog completeness per language against Word's 30 interface languages), platforms (build
   and install per platform), hardware and ecosystem rows as pass/fail.

**B. Measure once** and record the first measured number in `ROADMAP.md` (done 2026-10-25: 66.1%
replaces ~60% in `ROADMAP.md`, `target-app-parity.md` and `gaps.md`).

**C. Fix what the checks fail**, highest weight per hour first, until `cargo xtask readiness`
reports 100.0% on the latest commit: every row of every check set passing. The 2026-11-01
measurement passes stability (222/222 corpus; 5/5 hand checks) and performance (6/6); the largest
remaining areas are pagination (56.1%), dialog checks (187/228), and localization (30/95). The
corpus now matches Word's page count for 162/222 documents and page boundaries for 87/222; the
leading table-row page-break fix removes a blank page in govuk-003, but its page-boundary check
still differs. A zero-size Word frame-marker fix restores page flow in zenodo-030 (34 pages versus
Word's 32).
Some rows can only pass
on the owner's hardware: the Windows field reports of stability (startup crash, freezes, saving
without administrator rights), typing and scrolling on a Windows PC, and the printer, pen,
microphone, scanner, touch and multi-monitor rows; the platform rows also need the Windows arm64,
Linux and macOS builds, which wait on the owner's go-ahead (2026-10-26). 100% further needs every
corpus document to paginate exactly as Word does, an interface catalog for each of the 95 languages
Microsoft 365 offers, and every Word dialog and shortcut in the lists, including the ones tied to
Microsoft services (owner rule: out of scope), so those rows decide how close the number can get.
Those rows wait as `deferred` until the owner asks for them (owner, 2026-10-27), and
[`readiness.md`](readiness.md) reports a second number beside the measured one: readiness without
the deferred rows, also aimed at 100% ([rules](readiness/README.md)). The work: real-world DOCX failures (gap #1), pagination
(gap #4), crashes and hangs on Windows (gap #3), Word ink round trip (gap #2), native printing
(gap #6), missing dialogs and options (gap #7) and shortcuts (gap #24), editing Word's charts and
SmartArt (gap #13), and the alpha gate re-run on Windows.

## Next after beta

Charts and SmartArt editing, the Draw tab, `.doc` writing, PDF Reflow, the remaining interface
languages, screen-reader access.

## Revision history

| Date | Change | Summary |
|---|---|---|
| 2026-11-02 | minor | M14: v1.0.0–v1.1.2 published on the fork `veevie55/doc4me` (v1.1.2 the latest), after Actions of `chatraweechai/doc4me` stopped over a billing lock; a page break before an empty paragraph ending a section leaves no blank page (Linux corpus: page counts 166 → 168, page breaks 89 → 90) |
| 2026-11-01 | minor | Map classic New Toolbar and menu customization dialogs to the tested Customize Ribbon replacement; dialogs 187/228, readiness 86.0% (88.5% without deferred) |
| 2026-11-01 | minor | Map the legacy Advanced Settings dialog to the tested File › Options › Advanced pane; dialogs 184/228, readiness 85.8% (88.3% without deferred) |
| 2026-11-01 | minor | Verify eco-003 with Syncthing 2.1.7: save-in-place on peer B synced to peer A and both edits reopened; ecosystem 5/6, readiness 85.7% (88.2% without deferred) |
| 2026-11-01 | minor | Skip only leading table-row split pieces containing page-break clusters; govuk-003 page count now matches (5) without changing its still-failing page-boundary check; corpus page counts 162/222, page breaks 87/222; readiness 85.4% (87.8% without deferred) |
| 2026-11-01 | minor | `w:gridBefore`/`w:gridAfter` support fixes govuk-009 pagination; corpus passes rise to 161/222 page counts and 87/222 page breaks; readiness 85.4% (87.8% without 26 deferred) |
| 2026-11-01 | minor | Readiness 85.3% (87.7% without 26 deferred); exclude Macintosh-only dlg-040; zero-size frame markers stay in page flow; Syncthing ecosystem check remains untested |
| 2026-11-01 | minor | Full readiness 85.2% on 222 documents (87.6% without 26 deferred checks); DOCX round-trip 222/222; shortcuts 268/280 after Alt+Shift+F10 available-actions menu; remaining score is blocked by pagination, dialogs and localization |
| 2026-10-27 | trivial | Target raised from 95% to 100% measured (owner); what 100% needs |
| 2026-10-25 | trivial | Target raised from 80% to 95% measured (owner); the rows only the owner can check |
| 2026-10-25 | trivial | Stage C so far: measured 73.4% |
| 2026-10-25 | trivial | Path to 80%: stage B done, 66.1% measured replaces the ~60% estimate; alpha-gate row for received .docx from the corpus |
| 2026-10-25 | trivial | Path to 80%: stage A done (measurement and a 222-document corpus); the number goes into `ROADMAP.md` in stage B |
| 2026-10-25 | trivial | Path to 80% measured readiness: build the measurement (xtask readiness, DOCX corpus with Word-made PDFs, pagination, stability, feature checklist), measure, fix to 80% |
| 2026-10-25 | trivial | Beta list item 5 no longer waits on an Aptos-metric font (Intos bundled) |
| 2026-10-25 | trivial | Next after beta no longer lists ecosystem decisions or an in-app assistant (out of scope, owner decision) |
| 2026-10-25 | trivial | M2, M3 and M11 done: Dictate, Online Pictures, Get Add-ins, Ink Equation and Ink to Math landed with free and open-source parts; catalog parity 499/499 |
| 2026-10-25 | trivial | M1, M2, M3, M8, M11 and M12 rows checked against the code; M3 and M8 done apart from service-bound items; alpha gate's command count 618 |
| 2026-10-11 | trivial | M15: Polish interface and Polish proofing landed |
| 2026-10-11 | trivial | M3: icons landed (#309) |
| 2026-10-11 | trivial | M11: contour (Tight/Through) wrap done (#354) |
| 2026-10-11 | trivial | M6: the eight missing bibliography styles landed (#383) |
| 2026-10-11 | trivial | M7: formatting revisions landed (#41) |
| 2026-10-11 | trivial | Omitted `w:pPrDefault` stays omitted across saves; explicit empty/default properties and later spacing edits remain writable. Synthetic preservation checks only; readiness estimates unchanged |
| 2026-10-10 | trivial | M5: Draw Table and Eraser landed (#303) |
| 2026-10-10 | trivial | Merged main: PRs that landed removed from the Current focus list |
| 2026-10-10 | minor | Current focus heading back to ~150–230 h to beta: full number restored to the additive weighted sum (~60%), method aligned with the standard, no new evidence |
| 2026-10-10 | minor | Alpha gate table added (core-workflow gate from the progress-docs standard): all six workflows pass, stage stays alpha |
| 2026-10-10 | major | Created: milestones moved from ROADMAP.md with remaining hours; Current focus from gaps.md; M15–M16 added |
| 2026-10-10 | trivial | M11: ink pens and eraser landed (#307) |
| 2026-10-11 | trivial | M11: lasso, Add Pen, Ink to Shape and Ink Replay landed (#395) |
