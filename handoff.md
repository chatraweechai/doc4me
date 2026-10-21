# Handoff: Doc4Me 1.1.0 และงานภาษาไทย

อัปเดต: 2026-10-21 · สาขา `main` · รายละเอียดทุกข้อดูใน [`CHANGELOG.md`](CHANGELOG.md) หัวข้อ `[1.1.0]`

## สถานะล่าสุด

- **ออกเวอร์ชัน 1.1.0** (2026-10-21)
  - ตั้งเลขด้วย `cargo xtask version set 1.1.0` แล้วย้าย `[Unreleased]` ไปเป็น `## [1.1.0] - 2026-10-21` พร้อมลิงก์ท้ายไฟล์ (GitHub ไม่มี tag `v1.0.0` ลิงก์ `[1.1.0]` จึงชี้ไปหน้า Release)
  - push tag `v1.1.0` แล้ว workflow `windows-x64-installer.yml` จะ build MSI และ zip และสร้าง GitHub Release `Doc4Me v1.1.0` โดยใช้เนื้อหาหัวข้อ `[1.1.0]` เป็นบันทึกประจำรุ่น
  - ถ้า Release ยังไม่ขึ้น ให้ดูผลของ run ที่ tag `v1.1.0` ในแท็บ Actions
- **งานถัดไป: ภาพหน้าจอสำหรับ README** (ผู้ใช้สั่งไว้ให้ทำหลังออกเวอร์ชัน)
  - ตอนนี้ยังไม่มี `README.md` ที่ราก repo

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

## วิธีตรวจก่อน commit (ตามกฎใน CLAUDE.md)

- ใช้ `cargo xtask ci` ซึ่งรัน fmt, clippy -D warnings, test, assets, layers และ wasm
- ถ้าดิสก์น้อย ให้ใช้ `CARGO_INCREMENTAL=0` และทดสอบทีละแพ็กเกจ
  - หลังแต่ละแพ็กเกจ ให้ลบไฟล์ทดสอบใน `target/debug/deps`
  - build ทั้ง workspace ต้องใช้ดิสก์ราว 25 GB
- เทสต์ Myanmar จะข้ามไปเองเมื่อเครื่องไม่มีฟอนต์ที่มีกฎ `mym2`/`mymr` เช่นเครื่องที่มีแค่ Unifont
