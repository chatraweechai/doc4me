# บันทึกการเปลี่ยนแปลง (Changelog)

บันทึกการเปลี่ยนแปลงสำคัญทั้งหมดของ Doc4Me รูปแบบอิง [Keep a Changelog 1.1.0](https://keepachangelog.com/en/1.1.0/)
และเลขเวอร์ชันใช้ [Semantic Versioning](https://semver.org/) (แหล่งเดียวคือ `[workspace.package] version` ใน `Cargo.toml`)

กติกา (`CLAUDE.md` ข้อ 3):
- ทุกการเปลี่ยนแปลงโค้ด build packaging asset หรือเวอร์ชัน ต้องบันทึกใต้ `## [Unreleased]` ใน commit เดียวกัน
- ก่อน push tag `vX.Y.Z` ให้ย้ายรายการไปเป็น `## [X.Y.Z] - YYYY-MM-DD`
- workflow `windows-x64-installer.yml` นำเนื้อหาใต้หัวข้อนั้นไปเป็นบันทึกของ GitHub Release

## [Unreleased]

ยังไม่มีรายการ

## [1.0.0] - 2026-10-15

Doc4Me รุ่นแรก พัฒนาต่อจาก WordCraft 0.6.0 ของทีม ArtCraft (Apache-2.0 / MIT) และเน้นการใช้งานภาษาไทย

### ที่มา

- **2026-10-14 · `12b8c79`** นำซอร์สของ WordCraft 0.6.0 เข้า repo นี้
  - เพิ่ม workflow `.github/workflows/windows-x64-installer.yml` สำหรับสร้างตัวติดตั้ง Windows x64 (MSI) และไฟล์ zip แบบ portable บน GitHub Actions
- **2026-10-15 · `e43d1a4`** เปลี่ยนชื่อเป็น Doc4Me, ใช้ Sarabun เป็นฟอนต์หลัก และรองรับภาษาไทย (รายละเอียดอยู่ในหัวข้อด้านล่าง)
- **2026-10-15 · `ca0e607`** เปลี่ยนลิงก์ทั้งหมดไปที่ repo ใหม่ `chatraweechai/doc4me` (29 ไฟล์)
- **2026-10-15** เพิ่ม changelog นี้และกฎข้อ 3 ใน `CLAUDE.md`; GitHub Release ใช้ changelog เป็นบันทึกประจำรุ่น

### เพิ่ม

#### ฟอนต์ Sarabun เป็นฟอนต์หลักของระบบ
- ฝังฟอนต์ Sarabun ของ Cadson Demak (SIL OFL 1.1) ไว้ในโปรแกรม
  - น้ำหนักที่ฝัง: Regular, Italic, Medium, SemiBold, Bold, BoldItalic
  - ไฟล์อยู่ที่ `assets/fonts/Sarabun-*.ttf` พร้อมสัญญาอนุญาต `assets/fonts/OFL-Sarabun.txt`
  - เพิ่มแถวใน `ATTRIBUTION.md` ครบทุกไฟล์
- เอกสารใหม่ใช้ Sarabun เป็นฟอนต์ตั้งต้น
  - ครอบคลุม Normal, หัวเรื่อง, ฟอนต์ธีม (major/minor) และฟอนต์สำรอง
  - กำหนดที่ `BODY_FONT` / `HEADING_FONT` ใน `crates/doc/src/styles.rs`
- Sarabun อยู่ลำดับแรกในรายการฟอนต์สำรองของภาษาไทย
- ฟอนต์ไทยที่ไม่มีในเครื่องจะวาดด้วย Sarabun แทน (`crates/fonts/src/word.rs`) ได้แก่
  - TH Sarabun New / TH SarabunPSK และฟอนต์ไทยแห่งชาติอื่นๆ
  - ฟอนต์ตระกูล UPC ของ Windows
  - ฟอนต์ไทยของ Google
  - ชื่อที่มีน้ำหนักหรือ " Italic" ต่อท้าย เช่น "Sarabun Light"
- หน้าจอโปรแกรม (เมนู ริบบอน กล่องโต้ตอบ) ใช้ Sarabun ระดับ regular, medium และ semibold ก่อน Inter และ Source Sans 3

#### การรองรับภาษาไทย
- **การตัดบรรทัดตามคำในพจนานุกรม** สำหรับไทย ลาว และเขมร
  - ใช้ ICU4X `icu_segmenter` 2.3 พร้อมข้อมูลพจนานุกรมที่คอมไพล์ในตัว (`crates/doc/src/cluster.rs`)
  - การดับเบิลคลิกเลือกคำ, Ctrl+ลูกศร และการนับคำ ใช้ขอบเขตคำชุดเดียวกัน
  - ตัดเฉพาะเมื่ออักขระทั้งสองฝั่งอยู่ในคลาส SA ของ UAX #14 เท่านั้น ตัวเลข "฿" และเครื่องหมายวรรคตอนไทย/เขมรจึงอยู่ติดกับคำของมัน
  - ย่อหน้ายาวมากถูกตัดเป็นช่วงที่มีขนาดจำกัดที่ช่องว่าง และไม่ถือรอยต่อระหว่างช่วงเป็นจุดตัดคำปลอม
  - หน่วยความจำแคชคำมีขนาดจำกัดแบบสองรุ่น รุ่นละ 4 MiB
- **วรรณยุกต์และเครื่องหมาย** (่ ้ ๊ ๋ ็ ์ …) นับเป็นอักขระของคำ "ง่าย" หรือ "พิมพ์" จึงเป็นหนึ่งคำ
- **การจัดแนวแบบกระจายไทย (Thai Distributed)**
  - อ่านและบันทึก `w:jc="thaiDistribute"` ได้ครบ
  - บรรทัดเต็มกระจายระยะระหว่างอักขระจนชิดขอบ และไม่ยืดบรรทัดสุดท้าย เหมือน Word
  - เพิ่มคำแปล "Thai Distributed" ในแคตตาล็อกภาษาของหน้าจอทั้ง 17 ภาษา
- **เลขลำดับรายการแบบไทย**
  - เลขไทย ๑ ๒ ๓ (`thaiNumbers`)
  - พยัญชนะไทย ก ข ค ง … ฮ (`thaiLetters`) 41 ตัว โดยไม่ใช้ ฃ ฅ ฆ แล้วต่อด้วย กก ขข …
- **คุณสมบัติอักษรซับซ้อน (complex script)**
  - ในเอกสารที่สร้างด้วย Doc4Me ฟอนต์ ขนาด ตัวหนา และตัวเอียงของข้อความไทยตามค่าฝั่งละติน เช่น ข้อความ 36 pt แสดงตัวไทยที่ 36 pt ด้วย
  - เมื่อบันทึกเป็น .docx จะเขียน `w:cs`, `w:szCs`, `w:bCs` และ `w:iCs` ให้ครบ Word จึงแสดงข้อความไทยตรงกัน
  - ไฟล์ .docx ที่เปิดมาจาก Word ใช้ค่าเดิมของไฟล์ (`Document::complex_script_from_file`)
- **ช่องกรอกข้อความในหน้าจอ** (ทุกจุดที่ใช้ `widgets::text_line` / `text_box`) มีความสูงเผื่อวรรณยุกต์ที่อยู่เหนือสระบน เช่น ที่ น้ำ สิทธิ์ ไม่ให้ถูกตัดหัว
  - ค่าที่ใช้คือ `TEXT_BOX_ROOM = 1.45`
- ภาพหน้าจอใน `docs/images` ถ่ายใหม่ด้วย Doc4Me และ Sarabun และเพิ่มหน้าตัวอย่างภาษาไทย `docs/images/thai.png`

#### การออกรุ่นและเอกสาร
- เพิ่ม `CHANGELOG.md` (ไฟล์นี้)
- เพิ่มกฎข้อ 3 ใน `CLAUDE.md` และย่อหน้าเรื่อง changelog ใน `AGENTS.md`
- workflow ตรวจว่า tag ตรงกับเวอร์ชันใน `Cargo.toml` ก่อน build หากไม่ตรงจะหยุด
- job release อ่านเนื้อหาใต้ `## [X.Y.Z]` ของ `CHANGELOG.md` มาเป็นบันทึกของ GitHub Release
  - ถ้าไม่พบหัวข้อของเวอร์ชันนั้น จะใช้บันทึกที่ GitHub สร้างจาก commit แทน
- `docs/releasing.md` เพิ่มขั้นตอนอัปเดต changelog ก่อนออกรุ่น

### เปลี่ยนแปลง

#### ชื่อโปรแกรม (WordCraft → Doc4Me)
- เปลี่ยนชื่อในทุกส่วน
  - crate (`doc4me-*`) และไฟล์โปรแกรม (`doc4me`, `doc4me-cli`, `doc4me-web`)
  - ตัวแปรสภาพแวดล้อม (`DOC4ME_*`) และโฟลเดอร์ตั้งค่า
  - ข้อความหน้าจอและแคตตาล็อกคำแปล
  - เอกสาร
  - ภาพพื้นหลังและเลย์เอาต์ Finder ของ DMG บน macOS
- App id เปลี่ยนเป็น `io.github.chatraweechai.doc4me` และลิงก์ทั้งหมดชี้ไปที่ `github.com/chatraweechai/doc4me`
- เงื่อนไขเครื่องหมายการค้าของ WordCraft กำหนดให้รุ่นดัดแปลงเอาเครื่องหมาย ArtCraft ออก จึงทำดังนี้
  - ลบ `docs/brand`
  - ลบปุ่ม เมนู ไอคอน และลิงก์ของ Discord
  - หน้า About แสดง "Based on WordCraft by the ArtCraft team"
  - เก็บประกาศลิขสิทธิ์ของผู้พัฒนา WordCraft ไว้ตามสัญญาอนุญาต
- ข้อมูลที่อยู่ผู้ดูแลแพ็กเกจ Linux ตั้งต้นเป็น `Doc4Me contributors <https://github.com/chatraweechai/doc4me/issues>` ไม่ใช้ที่อยู่ของต้นทางอีกต่อไป

#### ตัวติดตั้ง Windows
- MSI ใช้ `UpgradeCode` และ GUID ของ component ชุดใหม่ (`packaging/windows/doc4me.wxs`)
  - อัปเกรดทับ Doc4Me รุ่นก่อนได้ และไม่แตะ WordCraft ที่ติดตั้งอยู่
- ผู้ผลิตเป็น "Doc4Me contributors" และลิงก์ความช่วยเหลือชี้ไปที่หน้า issues
- MSI, zip แบบ portable และแพ็กเกจ Linux / macOS / เว็บ แนบไฟล์ต่อไปนี้ไปด้วย
  - `NOTICE`
  - ข้อความ OFL ของฟอนต์ที่ฝังในโปรแกรม (Sarabun, Inter, Source Sans 3, Source Serif 4, JetBrains Mono)
  - `LICENSE-UNICODE` (Unicode-3.0) ของข้อมูลพจนานุกรม ICU4X
- workflow ทำงานเฉพาะเมื่อ push `main`, push tag `v*` หรือสั่งรันเอง

#### เวอร์ชัน
- เปลี่ยนจาก 0.6.0 เป็น 1.0.0 (`cargo xtask version set 1.0.0`)

#### ความเข้ากันได้ของไฟล์
- ไฟล์ .docx ที่ WordCraft บันทึกพร้อม canvas ที่หมุนหรือพลิก ยังเปิดได้ถูกต้อง
  - ตัวอ่านรับทั้ง URI เดิม `urn:wordcraft:canvas-xfrm` และ URI ใหม่
- เอกสาร .docx ที่สร้างจาก Doc4Me เปิดใน Word แล้วข้อความไทยใช้ฟอนต์และขนาดเดียวกับข้อความละติน (ดูหัวข้อ complex script ด้านบน)

### แก้ไข

- **Design › Themes**: เมื่อหัวเรื่องและเนื้อความใช้ฟอนต์เดียวกัน (Sarabun) สไตล์เนื้อความไม่ถูกเปลี่ยนเป็นฟอนต์หัวเรื่องอีก
  - ไฟล์ `crates/engine/src/cmd/design.rs`
  - เทสต์ `theme_fonts_keep_body_styles_in_the_body_font`
- **แท็บ Layout**: ไม่เรียก `add_space` ของ egui ภายใน grid อีก ก่อนหน้านี้ debug assertion ทำงานเมื่อเปิดแท็บ (`gap_label` ใน `crates/ui-egui/src/ribbon.rs`)
- **บานหน้าต่าง Navigation**: หัวเรื่องชิดซ้ายเหมือนเดิม การเยื้องตามระดับจึงมองเห็น
- **การจัดแนวแบบกระจาย / กระจายไทย**: บรรทัดที่ลงท้ายด้วยช่องว่างยังกระจายจนชิดขอบขวา
  - เทสต์ `distributed_lines_ending_in_a_space_still_reach_the_margin`
- **การตัดคำไทย**: ตัวเลขและเครื่องหมายวรรคตอนไม่ถูกแยกออกจากคำ
  - เทสต์ `digits_and_punctuation_stay_with_their_word`
  - ไม่มีจุดตัดปลอมที่รอยต่อของช่วงในย่อหน้ายาว
- **เลขลำดับพยัญชนะไทย**: ลำดับที่ 4 เป็น "ง" ไม่ใช่ "ฆ"
- **ฟอนต์ไทยตามชื่อน้ำหนัก** เช่น "Sarabun Light" หรือ "TH Sarabun New Bold" ถูกจัดเป็นฟอนต์เอกสารภาษาไทย
  - เทสต์ `thai_families_named_by_weight_are_thai_document_fonts`
- ลบส่วนที่ค้างจาก Discord ใน `icons.rs`, `theme.rs` และ `i18n/audit.rs`

### ลบ

- `docs/brand` และองค์ประกอบ Discord ทั้งหมด (ตามเงื่อนไขเครื่องหมายการค้าของต้นทาง)
- `README.md` ถูกลบบน `main` โดยเจ้าของ repo (`3cccba7`) และยังคงลบอยู่

### ความปลอดภัย

- **Actions**
  - action ภายนอกใน workflow ตรึงด้วย commit SHA
  - checkout ไม่เก็บ credential (`persist-credentials: false`)
  - job build มีสิทธิ์อ่านอย่างเดียว เฉพาะ job release ของ tag ที่เขียน (สร้าง release) ได้
  - ชื่อ tag ส่งผ่านตัวแปรสภาพแวดล้อม ไม่แทรกลงสคริปต์โดยตรง
- **ข้อมูลจากไฟล์ถือว่าไม่น่าเชื่อถือ**
  - การตัดคำแบ่งช่วงที่ขอบอักขระ (`char_floor`) และใช้ `get()` แทนการ index
  - แคชมีขนาดจำกัด
  - ไม่มี `unwrap` / `expect` / `panic` ในโค้ดใหม่ที่ไม่ใช่เทสต์ ตามกฎ never-crash
- **RustSec (`cargo audit`)**: ไม่พบช่องโหว่
  - มีคำเตือนว่าไม่ได้รับการดูแลแล้ว (unmaintained) สองรายการ คือ `rustybuzz` 0.20.1 และ `ttf-parser` 0.25.1 ซึ่งมาทาง `krilla`
- **สัญญาอนุญาต**: เพิ่ม `LICENSE-UNICODE` และบรรทัด Unicode-3.0 / ICU4X ใน `NOTICE`
  - ทุก asset มีแถวใน `ATTRIBUTION.md` (`cargo xtask assets`: 52 ไฟล์ผ่าน)

### การตรวจสอบ

- `cargo xtask ci` (fmt, clippy `-D warnings`, เทสต์, assets, layers, wasm)
  - ผ่าน 1707 เทสต์ ล้มเหลว 1 เทสต์ คือ `myanmar_prevowel_is_drawn_before_its_consonant`
  - เทสต์นี้ล้มเหลวมาตั้งแต่ต้นทาง ไม่เกี่ยวกับภาษาไทย
- GitHub Actions run 38734047702 (commit `ca0e607`) สร้าง `doc4me-1.0.0-windows-x64.msi` (30.1 MB) และ zip แบบ portable (40.2 MB) สำเร็จ

### ข้อจำกัดที่ทราบ

- MSI และ exe ยังไม่ได้เซ็นดิจิทัล Windows SmartScreen จึงอาจเตือนเมื่อติดตั้ง
- ถ้ามี WordCraft ติดตั้งอยู่ ต้องถอนการติดตั้งเอง เพราะ `UpgradeCode` เป็นชุดใหม่
- ความสูงบรรทัดของ Sarabun ใน Doc4Me ใช้ค่า hhea (1.30 em) แต่ Word บน Windows ใช้ usWin (1.85 em) หน้าเอกสารจึงอาจยาวไม่เท่ากัน (`docs/layout-parity.md`)
- เมื่อ TH Sarabun New ถูกแทนด้วย Sarabun ขนาดตัวอักษรที่เห็นอาจต่างจากต้นฉบับ เพราะ TH Sarabun New ออกแบบให้ตัวเล็กกว่า
- ยังไม่มีสิ่งต่อไปนี้
  - หน้าจอภาษาไทย
  - ตัวตรวจตัวสะกดภาษาไทย
  - เลขลำดับแบบคำอ่านไทย (`thaiCounting`)

[Unreleased]: https://github.com/chatraweechai/doc4me/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/chatraweechai/doc4me/releases/tag/v1.0.0
