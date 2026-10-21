# Handoff: Doc4Me 1.1.0 และงานภาษาไทย

อัปเดต: 2026-10-21 · สาขา `main` · รายละเอียดทุกข้อดูใน [`CHANGELOG.md`](CHANGELOG.md) หัวข้อ `[1.1.0]`

## สถานะล่าสุด

- **ออกเวอร์ชัน 1.1.0: รอ push tag** (2026-10-21)
  - คอมมิต `Release 1.1.0` (260459d) และคอมมิต README อยู่บน `main` แล้ว
  - push tag จาก session นี้ไม่ได้ เพราะ git proxy ตอบ `HTTP 403` เฉพาะการ push tag (push สาขา `main` ได้ปกติ) และเครื่องมือ GitHub ที่มีสร้าง tag หรือ Release ไม่ได้
  - ให้เจ้าของ repo tag คอมมิตล่าสุดบน `main` ที่มี README:
    `git fetch origin && git tag -a v1.1.0 origin/main -m "Doc4Me 1.1.0" && git push origin v1.1.0`
  - อย่าสร้าง Release ผ่านหน้าเว็บ GitHub เอง เพราะ job `release` ของ workflow จะรัน `gh release create v1.1.0` แล้วล้มถ้ามี Release นั้นอยู่แล้ว
  - หลัง push tag workflow `windows-x64-installer.yml` จะ build MSI และ zip และสร้าง Release `Doc4Me v1.1.0` โดยใช้หัวข้อ `[1.1.0]` ใน CHANGELOG เป็นบันทึกประจำรุ่น
  - GitHub ยังไม่มี tag `v1.0.0` เลย (1.0.0 build ผ่าน Actions บน `main` เท่านั้น) ลิงก์ `[1.1.0]` ท้าย CHANGELOG จึงชี้ไปหน้า Release
- **README และภาพหน้าจอ**: ทำแล้ว อยู่ในหัวข้อ `[1.1.0]`
  - `README.md` ที่ราก repo กับภาพ `docs/images/hero.png`, `thai.png`, `dark.png` ที่ถ่ายจาก 1.1.0
  - ภาพอื่นใน `docs/images` (backstage, design, layout, references, review) ยังเป็นของ 1.0.0
  - สคริปต์ถ่ายภาพ (`ui_shot`) ไม่ได้เก็บใน repo วิธีถ่ายดูหัวข้อ "ถ่ายภาพหน้าจอ" ด้านล่าง
- **สิ่งที่สังเกตเห็นแต่ยังไม่แก้**: ในธีมมืด แถบของตัวเลื่อนซูมที่มุมขวาล่างมองไม่เห็น เห็นแค่ปุ่มเลื่อน

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

## ข้อที่ผู้ใช้สั่งให้ข้าม (2026-10-21)

- ข้อ 6 เดิม: ให้นักแปลเจ้าของภาษาตรวจภาษาไทยใน UI
- ข้อ 8 เดิม: วัดค่า TH SarabunIT๙ จากไฟล์ฟอนต์จริง (ตอนนี้เป็นค่าอนุมาน)

## ข้อควรระวังของ session นี้

- hook จะ commit อัตโนมัติเป็น "Claude checkpoint: <ไฟล์>" ทุกครั้งที่ใช้ Edit/Write
  - commit เหล่านี้ยังไม่ถูก push ให้ `git reset --soft origin/main` แล้ว commit ใหม่ครั้งเดียวก่อน push
- hook formatter จะจัดตารางใน `.md` ใหม่ทั้งไฟล์หลัง Edit/Write
  - เช่น `ROADMAP.md` เคยเปลี่ยนไป 439 บรรทัด
  - ถ้าจะแก้ไฟล์ที่มีตาราง ให้แก้ผ่านสคริปต์หรือ `sed` แล้วตรวจ `git diff --stat` ทุกครั้ง
- hook ห้ามรัน Python แบบ inline ใน Bash ให้เขียนสคริปต์เป็นไฟล์ก่อนแล้วค่อยรัน

## ถ่ายภาพหน้าจอ

- `cargo build -p doc4me-ui-egui --example ui_shot` แล้วรัน `target/debug/examples/ui_shot script.jsonl` (รูปแบบสคริปต์ดูหัวไฟล์ `crates/ui-egui/examples/ui_shot.rs`)
- container ไม่มี GPU ต้องติดตั้ง Vulkan แบบซอฟต์แวร์ก่อน: `apt-get update && apt-get install -y mesa-vulkan-drivers`
  - ถ้าไม่ติดตั้งจะล้มด้วย `No adapter found`
- คำสั่งที่ใช้สร้างภาพ `thai.png`: `ui.language` (`th`), `review.language` (`th-TH`, `default: true`), `styles.apply`, `text.insert` (`raw: true`), `text.newParagraph`, `view.zoomOut` และ `ui.key` Home พร้อม `cmd: true` เพื่อกลับไปต้นเอกสาร

## วิธีตรวจก่อน commit (ตามกฎใน CLAUDE.md)

- ใช้ `cargo xtask ci` ซึ่งรัน fmt, clippy -D warnings, test, assets, layers และ wasm
- ถ้าดิสก์น้อย ให้ใช้ `CARGO_INCREMENTAL=0` และทดสอบทีละแพ็กเกจ
  - หลังแต่ละแพ็กเกจ ให้ลบไฟล์ทดสอบใน `target/debug/deps`
  - build ทั้ง workspace ต้องใช้ดิสก์ราว 25 GB
- เทสต์ Myanmar จะข้ามไปเองเมื่อเครื่องไม่มีฟอนต์ที่มีกฎ `mym2`/`mymr` เช่นเครื่องที่มีแค่ Unifont
