# Handoff: งานภาษาไทยและความเที่ยงตรงแบบ Word (Doc4Me 1.0.x)

อัปเดต: 2026-10-19 · สาขา `main` · รายละเอียดทุกข้อดูใน [`CHANGELOG.md`](CHANGELOG.md) หัวข้อ `[Unreleased]`

## สิ่งที่เสร็จแล้วในรอบนี้

- **ระยะบรรทัดแบบ Word บน Windows**
  - ไฟล์: `crates/fonts/src/word.rs` (`line_metrics`, `pick_line`, `single_line`)
  - ใช้ usWin บวก external leading ทำให้ Sarabun ได้ 1.853 em
  - ฟอนต์ที่ถูกแทนที่ใช้ระยะบรรทัดของฟอนต์ที่เอกสารขอ: Cambria 1.172 em, Courier New 1.133 em ส่วนฟอนต์อื่นใช้ค่า hhea ของตัวแทน
- **TH Sarabun New / PSK / IT๙ ขนาดเท่าต้นฉบับ**
  - ไฟล์: `crates/fonts/src/thsarabun.rs` และ `word::MetricFit`
  - ย่อ Sarabun ให้สูงเท่าต้นฉบับ ใช้ความกว้าง kerning และระยะบรรทัด 1.331 em ของต้นฉบับ
- **Kerning และ ligature แบบ Word**
  - ไฟล์: `crates/layout/src/para.rs` (`run_features`)
  - kerning เปิดเฉพาะเมื่อมี `w:kern` ligature เปิดเฉพาะเมื่อมี `w14:ligatures`
  - อักษรซับซ้อน (อาหรับ ไทย ฯลฯ) ใช้ ligature ของฟอนต์เสมอ
- **เลขลำดับไทย**
  - `thaiCounting` (หนึ่ง สอง … สิบเอ็ด) และชนิดรายการ thaiNumbers, thaiLetters, thaiCounting
  - อยู่ในเมนู Numbering และกล่อง Define New Multilevel List
- **ตัวตรวจตัวสะกดไทย**
  - ไฟล์: `crates/proof/src/th`
  - ตรวจลำดับการพิมพ์ผิด และคำที่มักเขียนผิด 105 คู่ (`assets/spelling/th-misspellings.tsv`)
  - เน้นความแม่นยำ ไม่ทำเครื่องหมายคำที่ไม่รู้จัก
- **ส่วนติดต่อผู้ใช้ภาษาไทย**
  - ไฟล์: `crates/ui-egui/src/i18n/th.tsv` มี 2,433 รายการ ครบทุกคีย์
  - คู่มือรูปแบบและอภิธานศัพท์อยู่ที่ [`docs/localization-th.md`](docs/localization-th.md)
- **แก้ข้อบกพร่องจากการตรวจแบบ adversarial**
  - ลิเกเจอร์อาหรับ, ค่า ligatures ที่ไม่ถูกต้อง, ฟอนต์ที่ค่า usWin เป็นศูนย์
  - เลขไทยบนรายการเลขอารบิก
  - ตัวตรวจไทยกับคำยาว, พจนานุกรมผู้ใช้ และคำไทยติดอังกฤษ

## งานค้างและข้อจำกัดที่ทราบ

1. **ยังไม่ออกเวอร์ชัน**
   - เวอร์ชันใน `Cargo.toml` ยังเป็น 1.0.0 และรายการทั้งหมดอยู่ใต้ `[Unreleased]`
   - ขั้นตอนออกเวอร์ชัน:
     1. ตั้งเลขเวอร์ชันใหม่ (เช่น 1.1.0) ด้วย `cargo xtask version` หรือแก้ `[workspace.package] version`
     2. ย้าย `[Unreleased]` ไปเป็น `## [1.1.0] - วันที่`
     3. push tag `v1.1.0` workflow `windows-x64-installer.yml` จะสร้าง installer และ GitHub Release ให้
2. **การอ่าน kerning จาก ODT, RTF และ .doc ยังไม่ได้ทำ**
   - ค่าที่ต้อง map: `style:letter-kerning`, `\kerningN`, sprmCHpsKern
   - ไฟล์ที่ต้องแก้: model `Fmt` ใน `crates/formats/src/model.rs`, `rtf.rs`, `odt.rs` และ `crates/docbin`
3. **กล่อง Font (แท็บ Advanced) ยังไม่มีตัวเลือก Kerning for fonts และ Ligatures** ตอนนี้ตั้งได้ผ่าน `format.set` เท่านั้น
4. **ข้อความในแผนภูมิ, SmartArt และ m:nor ที่ใช้ TH Sarabun ยังวาดด้วย Sarabun ขนาดเต็ม**
   - ตำแหน่งโค้ด: `display.rs` `text_glyphs` และ `math.rs`
5. **คีย์ที่ใช้ร่วมกันใน UI ต้องแยกในโค้ด**
   - คีย์: Editor, Letter, Display, Open, Arabic
   - ตอนนี้แต่ละคีย์แปลได้แบบเดียว ดูรายละเอียดใน `docs/localization-th.md`
6. **ภาษาไทยใน UI ยังไม่ได้ให้นักแปลมืออาชีพที่เป็นเจ้าของภาษาตรวจทาน**
   - รอบนี้ตรวจด้วยสคริปต์และเทสต์ `thai_catalog_is_typed_cleanly` เท่านั้น
7. **เทสต์ audit ที่ ignore ไว้ (`untranslated_interface_text`) ยังล้มอยู่ ซึ่งเป็นปัญหาเดิม**
   - สาเหตุคือป้ายขนาดซอง "#10   4.13 × 9.5 in" ใน `dialogs_layout.rs` ไม่ได้ผ่าน `tl!`
8. **ค่าของ TH SarabunIT๙ อนุมาน ยังไม่ได้วัดจากไฟล์จริง**
   - ค่าเลขไทยอนุมานจากการออกแบบของฟอนต์
   - ถ้าได้ไฟล์ IT๙ มา ให้วัดและแก้ `thsarabun.rs`

## วิธีตรวจก่อน commit (ตามกฎใน CLAUDE.md)

- ใช้ `cargo xtask ci` ซึ่งรัน fmt, clippy -D warnings, test, assets, layers และ wasm
- ถ้าดิสก์น้อย ให้ใช้ `CARGO_INCREMENTAL=0` และทดสอบทีละแพ็กเกจ
  - หลังแต่ละแพ็กเกจ ให้ลบไฟล์ทดสอบใน `target/debug/deps`
  - build ทั้ง workspace ต้องใช้ดิสก์ราว 25 GB
- เทสต์ Myanmar จะข้ามไปเองเมื่อเครื่องไม่มีฟอนต์ที่มีกฎ `mym2`/`mymr` เช่นเครื่องที่มีแค่ Unifont
