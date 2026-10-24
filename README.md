# Doc4Me

**English** · [ภาษาไทย](README.th.md)

Doc4Me is a clean-room, open-source word processor written in Rust. It aims for complete parity
with Microsoft Word, then goes further on speed, openness and agent control: every feature is a
command that scripts and AI agents can run, over a local control channel or MCP.

It is in **alpha**: see [ROADMAP.md](ROADMAP.md) for what works today and what is left.

![Doc4Me with the built-in sample document](docs/images/hero.png)

## Download

Windows x64 builds (an MSI installer and a portable zip) are on the
[Releases](https://github.com/chatraweechai/doc4me/releases) page. They are not code-signed yet, so
Windows SmartScreen may ask before it runs them. The MSI installs over earlier Doc4Me versions.

On macOS, Linux and BSD, build it from source (below). Packaging scripts for macOS, Linux and the
web build are in [`packaging/`](packaging/).

## Highlights

- Opens and saves `.docx`, RTF, ODT, HTML, Markdown, LaTeX and plain text; reads `.doc`; exports
  PDF and page images (PNG, JPG/JPEG).
- A Word-style ribbon, styles, lists, tables, equations, SmartArt, charts, comments and tracked
  changes, references and citations (with Zotero), mail merge and Reading Mode.
- Interface in English, Thai, Russian, Hungarian, Turkish, Italian and other languages. Thai comes
  with dictionary word breaks, a Thai spelling checker, Thai list numbers and line spacing measured
  as Word for Windows measures it; see the [Thai page](README.th.md).
- Light and dark themes; in the dark theme the page stays white.
- `doc4me-cli` converts and edits documents without a window, and `doc4me-cli mcp` serves every
  command to AI agents ([docs/mcp.md](docs/mcp.md)).

### Kerning and ligatures

The Font dialog's Advanced page sets "Kerning for fonts" and ligatures, as Word stores them in
`.docx`. Without them, text gets Word's defaults: no kerning, and no ligatures in Latin text
(complex scripts such as Arabic and Thai always use their font's ligatures).

![The Advanced page of the Font dialog, with Kerning for fonts and Ligatures](docs/images/font.png)

### Dark theme

![Doc4Me in the dark theme](docs/images/dark.png)

## Build from source

Install Rust 1.90 or newer, then:

```sh
cargo run --release -p doc4me                 # the app
cargo run --release -p doc4me -- --sample     # the app with the sample document
cargo run --release -p doc4me-cli -- convert in.docx out.pdf
```

Before sending a change, run `cargo xtask ci` (format, clippy, tests, assets, layering and the
WebAssembly build). How the code is organised: [docs/architecture.md](docs/architecture.md);
rules for contributors and agents: [AGENTS.md](AGENTS.md); releases:
[docs/releasing.md](docs/releasing.md); what changed: [CHANGELOG.md](CHANGELOG.md).

## Licence

Doc4Me is licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your
option. The bundled fonts are under the SIL Open Font License and the Thai word-break data under
the [Unicode licence](LICENSE-UNICODE); see [NOTICE](NOTICE) and [ATTRIBUTION.md](ATTRIBUTION.md)
for every third-party component.

Doc4Me is an independent project, not affiliated with or endorsed by Microsoft. It contains no
Microsoft icons, images, templates or other assets. The screenshots above are of Doc4Me itself.
