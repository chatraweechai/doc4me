# Handoff: Doc4Me 1.1.0 และงานภาษาไทย

อัปเดต: 2026-10-21 04:30 UTC · สาขา `main` ที่คอมมิต `a4de4dc` · รายละเอียดทุกข้อดูใน [`CHANGELOG.md`](CHANGELOG.md) หัวข้อ `[1.1.0]`

## เริ่ม session ใหม่: ทำตามลำดับนี้

1. ตรวจว่ามี tag `v1.1.0` บน GitHub แล้วหรือยัง: `git fetch origin --tags && git ls-remote --tags origin`
2. ถ้ายังไม่มี tag
   - ให้เจ้าของ repo push tag จากเครื่องของตัวเอง เพราะ session บนคลาวด์ push tag ไม่ได้ (git proxy ตอบ `HTTP 403` เฉพาะ tag ส่วน push สาขา `main` ได้ปกติ)
   - คำสั่ง: `git fetch origin && git tag -a v1.1.0 origin/main -m "Doc4Me 1.1.0" && git push origin v1.1.0`
   - ถ้ามีคอมมิตใหม่บน `main` หลัง `a4de4dc` ให้ย้ายรายการใต้ `[Unreleased]` เข้า `[1.1.0]` ก่อน tag หรือ tag `a4de4dc` แทน `origin/main`
   - อย่าสร้าง Release ผ่านหน้าเว็บ GitHub เอง เพราะ job `release` รัน `gh release create v1.1.0` และจะล้มถ้ามี Release นั้นอยู่แล้ว
3. ถ้ามี tag แล้ว ให้ตามดู run ของ workflow `windows-x64-installer.yml` ที่ tag จนได้ Release `Doc4Me v1.1.0` พร้อม MSI และ zip
   - ถ้า job `release` ล้ม ให้อ่าน log ด้วยเครื่องมือ GitHub (`get_job_logs`) แล้วแก้
4. หลังออก 1.1.0 แล้ว งานที่เหลือดูหัวข้อ "งานถัดไป"
   - รายการใหม่ทั้งหมดลง `## [Unreleased]` เพราะ `[1.1.0]` ถือว่าออกแล้ว ห้ามแก้

## สถานะล่าสุด

- **ออกเวอร์ชัน 1.1.0: รอเจ้าของ repo push tag**
  - เลขเวอร์ชันใน `Cargo.toml` เป็น 1.1.0 และ CHANGELOG มีหัวข้อ `## [1.1.0] - 2026-10-21` (บันทึกประจำรุ่นยาว 221 บรรทัด)
  - `a4de4dc` คือคอมมิตที่ควร tag: CI `windows-x64-installer.yml` run #21 build MSI และ zip ผ่าน (2026-10-21 04:23 UTC)
  - GitHub ยังไม่มี tag `v1.0.0` เลย (1.0.0 build ผ่าน Actions บน `main` เท่านั้น) ลิงก์ `[1.1.0]` ท้าย CHANGELOG จึงชี้ไปหน้า Release ส่วนลิงก์ `[1.0.0]` เปิดไม่ได้และคงไว้ตามกฎไม่แก้ประวัติ
- **README และภาพหน้าจอ**: ทำแล้ว อยู่ในหัวข้อ `[1.1.0]`
  - `README.md` (อังกฤษ) และ `README.th.md` (ไทย) มีลิงก์สลับภาษากันที่หัวหน้า
  - ภาพเป็นคู่ฉากเดียวกัน หน้าจออังกฤษ/ไทย: `hero.png`/`thai.png`, `font.png`/`font-th.png` (หน้าขั้นสูงของกล่องฟอนต์), `dark.png`/`dark-th.png`
  - ภาพอื่นใน `docs/images` (backstage, design, layout, references, review) ยังเป็นของ 1.0.0 และ README ไม่ได้ใช้
  - การตรวจคำแปลหน้าไทยโดย agent อิสระไม่ได้รัน (ติด session limit) ควรให้ผู้ใช้หรือ agent อ่านทวนอีกรอบ
- **แก้แล้วก่อนออก 1.1.0**: หน้าขั้นสูงของกล่องฟอนต์เปิดเคิร์นนิงเองเมื่อแค่เปิดหน้า เพราะ `DragValue` บีบค่า 0 เป็น 0.5 (`crates/ui-egui/src/dialogs.rs`, เทสต์ `showing_the_font_advanced_page_changes_nothing`)
  - ตรวจแล้ว: `cargo test -p doc4me-ui-egui` ผ่าน (316 รายการ exit 0), clippy `-D warnings` และ fmt ผ่าน

## งานถัดไป

1. **ช่องตัวเลขที่เปลี่ยนเอกสารเองเมื่อแค่แสดง** (ข้อจำกัดที่ทราบใน `[1.1.0]`)
   - `egui::DragValue` ที่มี `range` อีกราว 46 ช่องบีบค่าที่มีอยู่ให้เข้าช่วงทุกครั้งที่วาด และ egui นับเป็นการเปลี่ยนค่า (`response.changed()` เป็นจริง) ถ้าค่าในเอกสารอยู่นอกช่วง แค่แสดงช่องก็เปลี่ยนเอกสาร
   - ตัวอย่างที่ยืนยันแล้ว: ช่องตัดภาพบนแถบคำสั่ง (`ribbon.rs` ช่วง 0–45% ผูกกับ `r.changed()`) แค่เลือกภาพที่ถูกตัดเกิน 45% ค่าก็ถูกเปลี่ยนเป็น 45%
   - ช่องอื่นต้องไล่ดูทีละช่องว่าค่าในเอกสารออกนอกช่วงได้จริงไหม (ช่อง "Start at" ของเลขบรรทัดและเชิงอรรถเริ่มที่ 1 เหมือน Word จึงไม่ใช่ปัญหา)
   - รายชื่อทั้งหมด: `grep -rn "DragValue::new" crates/ui-egui/src | grep "\.range("`
   - วิธีแก้ที่น่าจะใช้: `.clamp_existing_to_range(false)` ทุกช่อง พร้อมเทสต์ที่วาดช่องกับค่านอกช่วงแล้วตรวจว่าค่าไม่เปลี่ยน (แบบเดียวกับเทสต์ของกล่องฟอนต์)
2. **ธีมมืด**: แถบของตัวเลื่อนซูมที่มุมขวาล่างมองไม่เห็น เห็นแค่ปุ่มเลื่อน (ดูได้ใน `docs/images/dark.png`)
3. **ภาพหน้าจออีก 5 ภาพของ 1.0.0** ใน `docs/images` ถ้าจะใช้ต้องถ่ายใหม่ เพราะระยะบรรทัดเปลี่ยนตั้งแต่ 1.1.0

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
