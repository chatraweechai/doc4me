# Handoff: งานภาษาไทยและความเที่ยงตรงแบบ Word (Doc4Me 1.0.x)

อัปเดต: 2026-10-21 · สาขา `main` · รายละเอียดทุกข้อดูใน [`CHANGELOG.md`](CHANGELOG.md) หัวข้อ `[Unreleased]`

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
  - ไฟล์: `crates/ui-egui/src/i18n/th.tsv` มี 2,452 รายการ ครบทุกคีย์
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
2. ~~การอ่าน kerning จาก ODT, RTF และ .doc~~ ทำแล้ว (2026-10-20)
   - `Fmt::kern` ใน `crates/formats/src/model.rs`, `\kerningN` ใน `rtf.rs`, `style:letter-kerning` ใน `odt.rs` (ODF มีแค่เปิด/ปิด), sprmCHpsKern 0x484B ใน `crates/docbin/src/fmt.rs` (อ่านอย่างเดียว)
   - เทสต์: `kerning_round_trips_through_rtf_and_odt`, `kerning_for_fonts_is_read`
3. ~~กล่อง Font ไม่มี Kerning/Ligatures~~ ทำแล้ว (2026-10-20) แยกเป็นหน้า Font และ Advanced
4. ~~ข้อความในแผนภูมิ, SmartArt และ m:nor ที่ใช้ TH Sarabun~~ ทำแล้ว (2026-10-20) `display.rs` `text_glyphs` และ `math.rs` `run_atoms` ใช้ `metric_fit` เทสต์ `drawing_text_in_th_sarabun_is_sized_like_the_original` และ m:nor มีเทสต์ `normal_text_in_th_sarabun_is_sized_like_the_original` แล้ว (2026-10-21)
5. ~~คีย์ที่ใช้ร่วมกันใน UI~~ แยกแล้ว (2026-10-20) ด้วย context ใน `th.tsv`: bibliography/equation/spacing/numeral ส่วน Letter ไม่ต้องแยก ดู `docs/localization-th.md`
6. **ภาษาไทยใน UI ยังไม่ได้ให้นักแปลมืออาชีพที่เป็นเจ้าของภาษาตรวจทาน**
   - รอบนี้ตรวจด้วยสคริปต์และเทสต์ `thai_catalog_is_typed_cleanly` เท่านั้น
7. ~~เทสต์ audit `untranslated_interface_text`~~ รันด้วย `--ignored` แล้ว (2026-10-21)
   - พบคาตาล็อก ru/hu/tr/th ขาดข้อความโหมดอ่านและข้อความสถานะ เติมแล้ว (รวม it)
   - ข้อความ error ของคำสั่ง (ตัวพิมพ์เล็ก) audit ยกเว้นผ่าน `pseudo::statuses()` คงเป็นภาษาอังกฤษตามนโยบาย
   - เทสต์ยังติด `#[ignore]` เพราะเคยล้มครั้งเดียวตอนเครื่องโหลดหนัก ควรรันด้วยมือก่อนออกเวอร์ชัน
8. **ค่าของ TH SarabunIT๙ อนุมาน ยังไม่ได้วัดจากไฟล์จริง**
   - ค่าเลขไทยอนุมานจากการออกแบบของฟอนต์
   - ถ้าได้ไฟล์ IT๙ มา ให้วัดและแก้ `thsarabun.rs`

## วิธีตรวจก่อน commit (ตามกฎใน CLAUDE.md)

- ใช้ `cargo xtask ci` ซึ่งรัน fmt, clippy -D warnings, test, assets, layers และ wasm
- ถ้าดิสก์น้อย ให้ใช้ `CARGO_INCREMENTAL=0` และทดสอบทีละแพ็กเกจ
  - หลังแต่ละแพ็กเกจ ให้ลบไฟล์ทดสอบใน `target/debug/deps`
  - build ทั้ง workspace ต้องใช้ดิสก์ราว 25 GB
- เทสต์ Myanmar จะข้ามไปเองเมื่อเครื่องไม่มีฟอนต์ที่มีกฎ `mym2`/`mymr` เช่นเครื่องที่มีแค่ Unifont
