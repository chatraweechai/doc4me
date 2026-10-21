# Handoff: Doc4Me 1.1.0 และงานภาษาไทย

อัปเดต: 2026-10-21 · สาขา `main` · รายละเอียดทุกข้อดูใน [`CHANGELOG.md`](CHANGELOG.md) หัวข้อ `[1.1.0]`

## เริ่ม session ใหม่: ทำตามลำดับนี้

1. ตรวจว่ามี tag `v1.1.0` บน GitHub แล้วหรือยัง: `git fetch origin --tags && git ls-remote --tags origin`
2. ถ้ายังไม่มี tag ให้เจ้าของ repo push จากเครื่องของตัวเอง (session บนคลาวด์ push tag ไม่ได้ git proxy ตอบ `HTTP 403`)
   - **ผู้ใช้สั่งเมื่อ 2026-10-21 08:00 UTC**: ยกเลิก 1.1.0 ที่ `7e26f97` แล้วออก 1.1.0 จากโค้ดล่าสุดแทน
     - รายการที่เคยอยู่ใต้ `[Unreleased]` (ช่องตัวเลข `DragValue`, รางตัวเลื่อนซูมในธีมมืด) ย้ายเข้า `[1.1.0]` แล้ว ข้อจำกัดเรื่อง `DragValue` ใน `[1.1.0]` ลบออกเพราะแก้แล้ว
   - คำสั่ง: tag คอมมิตล่าสุดของ `main` ที่มีข้อความ "Release 1.1.0 from the latest code"
     `git fetch origin && git tag -a v1.1.0 origin/main -m "Doc4Me 1.1.0" && git push origin v1.1.0`
   - ถ้ามีคอมมิตใหม่บน `main` หลังจากนั้นที่ไม่ใช่งานของ 1.1.0 ให้ tag คอมมิต "Release 1.1.0 from the latest code" แทน `origin/main`
   - อย่าสร้าง Release ผ่านหน้าเว็บ GitHub เอง เพราะ job `release` รัน `gh release create v1.1.0` และจะล้มถ้ามี Release นั้นอยู่แล้ว
3. ถ้ามี tag แล้ว ให้ตามดู run ของ workflow `windows-x64-installer.yml` ที่ tag จนได้ Release `Doc4Me v1.1.0` พร้อม MSI และ zip
   - ถ้า job `release` ล้ม ให้อ่าน log ด้วยเครื่องมือ GitHub (`get_job_logs`) แล้วแก้
4. หลังออก 1.1.0 แล้ว งานใหม่ทั้งหมดลง `## [Unreleased]` และห้ามแก้ `[1.1.0]`

## สถานะล่าสุด

- **ออกเวอร์ชัน 1.1.0 จากโค้ดล่าสุด: รอ push tag** (ดูข้อ 2 ข้างบน)
  - GitHub ยังไม่มี tag `v1.0.0` เลย ลิงก์ `[1.0.0]` ท้าย CHANGELOG จึงเปิดไม่ได้และคงไว้ตามกฎไม่แก้ประวัติ
- **ทำแล้วใน session 2026-10-21 เช้า (อยู่ใน `[1.1.0]`)**
  - ช่องตัวเลข `DragValue` ที่มี `range` ทั้ง 47 ช่องไม่บีบค่าเดิมของเอกสารอีก (`.clamp_existing_to_range(false)`) พร้อมเทสต์ `showing_picture_size_fields_changes_nothing` และเทสต์ตรวจซอร์ส `ranged_number_fields_keep_existing_values` ที่จะล้มถ้ามีช่องใหม่ลืมใส่
  - ธีมมืด: รางตัวเลื่อนซูมในแถบสถานะใช้สี `input_border` แล้ว (`chrome.rs`) เทสต์ `zoom_slider_rail_shows_in_the_dark_theme`
  - ผลตรวจ: `cargo test -p doc4me-ui-egui` ผ่าน 319 รายการ, clippy `-D warnings` และ fmt ผ่าน; เทสต์ใหม่ทั้ง 3 ล้มเมื่อเอาการแก้ออกชั่วคราว
  - ยังไม่ได้รัน `cargo xtask ci` ทั้ง workspace (แก้แค่ `crates/ui-egui`) และ audit ภาษา `--include-ignored` (ไม่ได้เพิ่มข้อความ UI)
  - ยังไม่ได้ถ่ายภาพ `docs/images/dark.png` และ `dark-th.png` ใหม่ ภาพเดิมยังเป็นรางที่มองไม่เห็น

## งานถัดไป

1. **ถ่ายภาพ `dark.png` / `dark-th.png` ใหม่** ให้เห็นรางตัวเลื่อนซูม (วิธีดูหัวข้อ "ถ่ายภาพหน้าจอ")
2. **ภาพหน้าจออีก 5 ภาพของ 1.0.0** ใน `docs/images` (backstage, design, layout, references, review) README ไม่ได้ใช้ ถ้าจะใช้ต้องถ่ายใหม่ เพราะระยะบรรทัดเปลี่ยนตั้งแต่ 1.1.0
3. **ทวนคำแปล `README.th.md`** โดยผู้ใช้หรือ agent อิสระ (ยังไม่เคยรัน)
4. `Slider` อื่น (`read_aloud.rs`, `options.rs`, กล่อง Zoom ใน `dialogs.rs`) ใช้การบีบค่าแบบเดิมของ egui ค่าที่ใช้มาจากการตั้งค่าหรือมุมมองซึ่งอยู่ในช่วงเสมอ จึงยังไม่ได้แก้ ถ้าพบค่าที่มาจากเอกสารให้ใช้ `.clamping(egui::SliderClamping::Edits)`

## ข้อที่ผู้ใช้สั่งให้ข้าม (2026-10-21)

- ข้อ 6 เดิม: ให้นักแปลเจ้าของภาษาตรวจภาษาไทยใน UI
- ข้อ 8 เดิม: วัดค่า TH SarabunIT๙ จากไฟล์ฟอนต์จริง (ตอนนี้เป็นค่าอนุมาน)

## สิ่งที่อยู่ใน 1.1.0

- **ระยะบรรทัดแบบ Word บน Windows**
  - ไฟล์: `crates/fonts/src/word.rs` (`line_metrics`, `pick_line`, `single_line`)
  - ใช้ usWin บวก external leading ทำให้ Sarabun ได้ 1.853 em
  - ฟอนต์ที่ถูกแทนที่ใช้ระยะบรรทัดของฟอนต์ที่เอกสารขอ: Cambria 1.172 em, Courier New 1.133 em ส่วนฟอนต์อื่นใช้ค่า hhea ของตัวแทน
- **TH Sarabun New / PSK / IT๙ ขนาดเท่าต้นฉบับ**
  - ไฟล์: `crates/fonts/src/thsarabun.rs` และ `word::MetricFit`
  - ใช้กับเนื้อความ แผนภูมิ SmartArt และ m:nor ในสมการ (`display.rs` `text_glyphs`, `math.rs` `run_atoms`)
- **Kerning และ ligature แบบ Word**
  - ไฟล์: `crates/layout/src/para.rs` (`run_features`)
  - อ่านและเขียนใน .docx, RTF, ODT และอ่านจาก .doc
  - กล่อง Font แยกเป็นหน้า Font และ Advanced
- **เลขลำดับไทย**: `thaiCounting`, thaiNumbers, thaiLetters ในเมนู Numbering และกล่อง Define New Multilevel List
- **ตัวตรวจตัวสะกดไทย**: `crates/proof/src/th` กับคำที่มักเขียนผิด 105 คู่ (`assets/spelling/th-misspellings.tsv`)
- **ส่วนติดต่อผู้ใช้ภาษาไทย**
  - `crates/ui-egui/src/i18n/th.tsv` มี 2,452 รายการ ครบทุกคีย์
  - คู่มือรูปแบบและอภิธานศัพท์อยู่ที่ [`docs/localization-th.md`](docs/localization-th.md)
  - เทสต์ audit `untranslated_interface_text` ยังติด `#[ignore]` เพราะเคยล้มครั้งเดียวตอนเครื่องโหลดหนัก ให้รันด้วยมือก่อนออกเวอร์ชัน
    - `cargo test -p doc4me-ui-egui i18n -- --include-ignored` ผ่าน 60 รายการเมื่อ 2026-10-21

## ข้อควรระวังของ session บนคลาวด์ (hook ของ plugin)

- **commit อัตโนมัติ**: ทุกครั้งที่ใช้ Edit/Write (รวมถึง agent ย่อย และไฟล์นอก repo) hook จะรัน `git add <ไฟล์>; git commit -m "Claude checkpoint: …"`
  - ถ้ามีไฟล์ที่ stage ค้างไว้ จะถูก commit ไปด้วย ให้ stage แล้ว commit ในคำสั่ง Bash เดียวกัน
  - ก่อน push ให้รวม commit เหล่านี้: `git reset --soft origin/main` แล้ว commit ใหม่ครั้งเดียว
- **formatter**: จัดตารางในไฟล์ `.md` ใหม่ทั้งไฟล์หลัง Edit/Write (เช่น `ROADMAP.md` เคยเปลี่ยน 439 บรรทัด)
  - แก้ไฟล์ที่มีตารางผ่านสคริปต์หรือ `sed` แล้วตรวจ `git diff --stat` ทุกครั้ง
- **ข้อจำกัดของ Bash**
  - ห้ามรัน Python แบบ inline ให้เขียนสคริปต์เป็นไฟล์ใน scratchpad แล้วรันด้วย `python3 -I`
  - คำสั่งยาวได้ไม่เกิน 500 ตัวอักษร และมีตัวคั่นคำสั่งได้ไม่เกิน 10 ตัว (`|` ในข้อความ Markdown ก็นับด้วย)
  - เขียนไฟล์ใน `.git` ไม่ได้ (เช่น `.git/info/exclude`)
- **`CLAUDE.md` ถูกเติมเอง**: hook ชื่อ context-recovery ต่อบล็อก "Session Recovery" ท้าย `CLAUDE.md`
  - อย่า commit บล็อกนี้ ให้คืนไฟล์ด้วย `git checkout -- CLAUDE.md`
- **stop hook** เตือนเมื่อ working tree ไม่สะอาด
  - ไดเรกทอรี log ของ plugin (`.anchor/`, `.harnesskit/`, `.viberevert/`) อยู่ใน `.gitignore` แล้ว
- **รันเทสต์ผ่าน pipe**: ใช้ `set -o pipefail` เพื่อให้ exit code เป็นของ cargo

## ถ่ายภาพหน้าจอ

- `cargo build -p doc4me-ui-egui --example ui_shot` แล้วรัน `target/debug/examples/ui_shot script.jsonl` (รูปแบบสคริปต์ดูหัวไฟล์ `crates/ui-egui/examples/ui_shot.rs`)
- container ไม่มี GPU ต้องติดตั้ง Vulkan แบบซอฟต์แวร์ก่อน: `apt-get update && apt-get install -y mesa-vulkan-drivers`
  - ถ้าไม่ติดตั้งจะล้มด้วย `No adapter found`
- กล่องฟอนต์: `ui.dialog` (`name: font`) แล้ว `ui.click` ที่แท็บขั้นสูง (อังกฤษ 630,257 pt ไทย 625,257 pt) และ `ui.move` ไปนอกหน้าต่างเพื่อไม่ให้มีตัวชี้เมาส์ในภาพ; ตั้งค่าตัวอักษรด้วย `format.set` (`{"props": {...}}`)
- คำสั่งที่ใช้สร้างภาพ `thai.png`: `ui.language` (`th`), `review.language` (`th-TH`, `default: true`), `styles.apply`, `text.insert` (`raw: true`), `text.newParagraph`, `view.zoomOut` และ `ui.key` Home พร้อม `cmd: true` เพื่อกลับไปต้นเอกสาร
- ธีมมืด: `ui.theme` (`value: dark`)

## วิธีตรวจก่อน commit (ตามกฎใน CLAUDE.md)

- ใช้ `cargo xtask ci` ซึ่งรัน fmt, clippy -D warnings, test, assets, layers และ wasm
- ถ้าดิสก์น้อย ให้ใช้ `CARGO_INCREMENTAL=0` และทดสอบทีละแพ็กเกจ
  - หลังแต่ละแพ็กเกจ ให้ลบไฟล์ทดสอบใน `target/debug/deps`
  - build ทั้ง workspace ต้องใช้ดิสก์ราว 25 GB
- เทสต์ Myanmar จะข้ามไปเองเมื่อเครื่องไม่มีฟอนต์ที่มีกฎ `mym2`/`mymr` เช่นเครื่องที่มีแค่ Unifont
