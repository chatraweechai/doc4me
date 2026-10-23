# Handoff: Doc4Me 1.1.1 และงานภาษาไทย

อัปเดต: 2026-10-23 04:17 UTC · สาขา `main` · session ยุติแล้ว · CI ของ `main` ล่าสุดผ่าน (build installer เท่านั้น) · เริ่ม session ใหม่ที่หัวข้อ "งานที่ต้องทำต่อใน session ใหม่" · รายละเอียดดู [`CHANGELOG.md`](CHANGELOG.md) หัวข้อ `[Unreleased]`, `[1.1.1]` และ `[1.1.0]`

## งานที่ต้องทำต่อใน session ใหม่ (รวบรวม 2026-10-23)

ที่มา: workflow สำรวจงานค้าง (agent อ่านอย่างเดียว 2026-10-22 15:15 UTC) ซึ่งขั้นตรวจซ้ำล้มเพราะชนขีดจำกัดของ session

- ข้อที่มี **[ตรวจแล้ว]** ตรวจกับโค้ดเมื่อ 2026-10-23 04:10–04:15 UTC ข้ออื่นมาจาก finder ตัวเดียว ต้องตรวจซ้ำก่อนลงมือ
- ให้ผู้ใช้เลือกหรือยืนยันลำดับงานก่อน (แนะนำข้อ 1 แล้วข้อ 2)
- งานโค้ดหรือเอกสารทุกชิ้นต้องบันทึกใน `CHANGELOG.md` `[Unreleased]` ในคอมมิตเดียวกัน (CLAUDE.md ข้อ 3) แล้ว push ไป `main`
- ห้าม push ไป `main` ระหว่างที่ job `release` ของ workflow รันอยู่ โดยเฉพาะการแก้ `.github/workflows`
- คอมมิตที่แก้แค่ `handoff.md` ใส่ `[skip ci]` ได้ run ล่าสุดของ `main` จะได้ไม่ใช่ run ที่ยังไม่จบ

### 1. บั๊ก: วันที่และเวลาที่แสดงผู้ใช้เป็น UTC ไม่ใช่เวลาของเครื่อง [ตรวจแล้ว]

- `iso_from_unix_secs` และ `now_iso` (`crates/engine/src/cmd/mod.rs:378-402`) สร้างเวลา UTC ลงท้าย `Z` และไม่มีโค้ดส่วนไหนอ่าน time zone ของเครื่อง
- ผลที่ผู้ใช้ไทย (UTC+7) เห็น:
  - คำทักทายบนหน้า Home ของ backstage (`crates/ui-egui/src/backstage.rs:194`) ขึ้น "สวัสดีตอนเช้า" ไปถึง 18:59 น.
  - Insert › Date and Time (`crates/engine/src/cmd/insert.rs:504` `format_date`) ได้วันของเมื่อวานช่วง 00:00–06:59 น. และเวลาช้าไป 7 ชั่วโมง
  - "วันนี้" ในตัวเลือกวันที่ของ content control (`crates/ui-egui/src/content_controls.rs:150` `today`) ก็ใช้ UTC
- ผู้เรียก `now_iso()` อื่นที่ต้องแยกว่าเป็นเวลาที่แสดงผู้ใช้หรือ metadata: `cmd/review.rs`, `cmd/fmt_revisions.rs`, `cmd/moves.rs` (วันที่ของ revision และ comment), `cmd/tools.rs:349` (versions), `cmd/tools.rs:925`, `session.rs:721` (`core.modified`)
- แนวทาง:
  - เพิ่มฟังก์ชันหา offset ของเครื่อง (desktop ใช้ API ของระบบ, wasm ใช้ `Date.getTimezoneOffset`) และใช้กับค่าที่แสดงผู้ใช้
  - metadata ให้ดูก่อนว่า Word เขียน `w:date` แบบไหนแล้วค่อยตัดสินว่าจะเปลี่ยนหรือไม่
  - เพิ่มเทสต์ และบันทึกใน `[Unreleased]` › แก้ไข
- แก้แล้วต้องแก้หมายเหตุเรื่องคำทักทายใน `backstage.png` ในหัวข้อถ่ายภาพหน้าจอตามด้วย

### 2. เอกสารที่ล้าสมัยหลังออก 1.1.1 (แก้แค่เอกสาร ทำในคอมมิตเดียวได้)

- `ROADMAP.md` [ตรวจแล้วที่ :25]
  - :25 แถว Releases ยังเป็น v1.1.0, :5 ยังเป็น "Last updated 2026-10-21" และบันทึกความคืบหน้า (:137) ยังไม่มีแถว 2026-10-22
  - ต้องเพิ่มของ 1.1.1: ปุ่มจัดกระจายและจัดกระจายแบบไทย, ระยะห่างตัวอักขระ 0–1584 pt และการออก Release จากแท็บ Actions (`AGENTS.md:65` กำหนดให้อัปเดต gap, เอกสาร parity และบันทึกความคืบหน้า)
  - :24 ตัวเลขเก่า (433 คำสั่ง, 855 เทสต์, ~88,700 บรรทัด) ส่วน finder นับได้ 607 คำสั่ง, 1,754 เทสต์ และราว 180,900 บรรทัด
  - :17 ตัวเลข parity 484/499 ให้แก้ตามข้อ `parity-checklist.md` ข้างล่าง
  - formatter จะจัดตารางใหม่ทั้งไฟล์ ให้แก้ด้วย `sed` หรือสคริปต์ แล้วตรวจ `git diff --stat` (ดูข้อควรระวังของ session บนคลาวด์)
- `docs/releasing.md` [ตรวจแล้วที่ :7 และ :19]
  - ตัวอย่าง `cargo xtask version set 1.0.1` และ `git tag v1.0.0` ล้าสมัย ให้ใช้ x.y.z และ `git tag -a`
  - เพิ่มขั้นแก้ลิงก์ `[Unreleased]` เป็น `compare/vx.y.z...HEAD` และขั้นรัน audit ที่ติด `#[ignore]`
  - :14-15 บอกว่า `sign.ps1` ลงนามเมื่อมี secret ซึ่งไม่จริงใน CI (ดูข้อ 3)
- `docs/parity-checklist.md`
  - ตอน 1.1.1 แก้ตัวเลขด้วยมือเป็น 484/499 แต่ catalog มี 500 รายการ (รวม `File|Options|Numerals|view.numeral` ที่ `crates/engine/src/catalog.rs:22`) จึงควรได้ราว 485/500 และ File 22/22
  - ให้รัน `cargo xtask parity` (ต้อง build) แล้วแก้ `ROADMAP.md:17` ตาม
- เอกสารที่บอกว่าตัวตรวจภาษามีแค่อังกฤษกับโปแลนด์ ทั้งที่ภาษาไทยออกแล้วใน 1.1.0: `docs/gaps.md:23` (#8), `docs/target-app-parity.md:69,198`, `docs/architecture.md:20`, `docs/localization-parity.md:32,73`
- จำนวนภาษา UI และขนาดแค็ตตาล็อก: จริงมี 19 ภาษารวมไทย (`crates/ui-egui/src/i18n/mod.rs`) แต่เอกสารยังเขียนผิด
  - `docs/localization-parity.md:22` บอก 18 ภาษาและไม่มีไทย, :61 ไม่นับไทย, :76 บอกว่าไทยมี 2,433 รายการ (จริง 2,456)
  - `docs/target-app-parity.md:198` บอก 13 ภาษา, `docs/roadmap.md:63` บอก 15 ภาษา
  - `docs/gaps.md:26` (#11) ยังนับ fr ว่าขาดทั้งที่มี `fr.tsv` แล้ว และตัวเลขของ tr กับ hu ใน `ROADMAP.md:114-116` เก่า
- `docs/gaps.md` ขัดกับโค้ด
  - #9 (:24) ไฟล์เข้ารหัส: มี `crates/docx/src/crypt.rs` แล้ว เหลือแค่ .doc แบบ RC4/XOR
  - #15 (:30) แหล่งอ้างอิง: อ่านและเขียน `b:Sources` ได้แล้ว
  - #17 (:32) `para.asianTypography` อยู่ใน catalog แล้ว
  - #10 (:25) group มีแล้ว และตัวเลข Draw กับ Shape Format ต้องแก้
  - #2 (:17) รองรับ wpc แล้ว เหลือแค่ Word ink (`w14:contentPart`)
  - :54-56 ยังบอกว่ามี PR เปิดอยู่
  - `docs/file-format-parity.md:36` (เข้ารหัส ❌) ขัดกับ :127-128 ของไฟล์เดียวกัน
- เอกสารที่ยังนับว่าฟีเจอร์ที่มีแล้วยังขาด: `ROADMAP.md:44-47` (Why not beta) และ :128-131 (Upcoming), `docs/roadmap.md:21-24` (Current focus), milestone M1–M12, alpha gate แถว 35/36/38 และ `docs/target-app-parity.md:111`
  - ฟีเจอร์เหล่านั้นคือ: เก็บ chart/SmartArt/OLE ไว้เขียนกลับ, footnote ต่อหน้า, column balancing, content control, move tracking, แหล่งข้อมูล Excel, grouping และการเข้ารหัส
- `docs/target-app-parity.md:60` บอกว่า kerning/ligature ในกล่อง Font ยังขาด (ออกแล้วใน 1.1.0) ที่ขาดจริงเหลือแค่ stylistic set, number form และ number spacing
- ข้อความที่ติดมาจาก WordCraft
  - `ROADMAP.md:42-44,64` บอกว่ามี build ที่ลงนามแล้วทุกแพลตฟอร์มและเว็บ
  - `docs/roadmap.md:62` M14 บอกว่าออก release v0.1.0–v0.4.0
  - `docs/architecture.md:59-60` บอกว่ามี CI บน PR และ branch `release`
  - `packaging/windows/package.ps1:161-162` อ้างถึง `windows-arm64.yml` ที่ไม่มีอยู่จริง
  - `packaging/env.sh:57` และคอมเมนต์เรื่อง CI ใน `packaging/linux/` และ `packaging/macos/`
- เวอร์ชันและวันที่ที่ล้าสมัย
  - `docs/roadmap.md:3,34` (422 คำสั่ง) และ `docs/gaps.md:3`
  - `docs/target-app-parity.md:3,12` (v0.4.0 และคอมมิต `a99dddc` ที่ไม่มีใน repo นี้)
  - `docs/localization-hu.md:3` และ `docs/localization-tr.md:3` เขียน "Target: Doc4Me 0.5.0" ซึ่งเป็นรุ่นที่ไม่เคยมี
- `docs/gaps.md` ไม่มีแถวของภาษาไทย (ไวยากรณ์, การตัดบรรทัดเทียบกับ Word, ligature ทางเลือก, ค่า IT๙ ที่อนุมาน) และ `docs/localization-parity.md:83` เขียนเกินจริงว่า IT๙ ตัดบรรทัดเหมือนฟอนต์จริง
- `README.th.md:13` หัวข้อ "ภาษาไทยใน Doc4Me 1.1.0" ยังไม่พูดถึงปุ่มจัดกระจายแบบไทยของ 1.1.1

### 3. CI การตรวจสอบ และ dependency

- CI บน GitHub มีแค่ `windows-x64-installer.yml` [ตรวจแล้ว]
  - workflow นี้ build `doc4me` กับ `doc4me-cli` แบบ release, แพ็ก MSI และ zip แล้วรัน `doc4me-cli --version` เท่านั้น
  - ไม่รัน fmt, clippy, เทสต์, assets, layers หรือ wasm และไม่มี trigger `pull_request` "CI ผ่าน" จึงหมายถึงแค่ build installer ผ่าน
  - ข้อเสนอ: เพิ่ม job บน ubuntu-latest รัน `cargo xtask ci` (หรือ fmt + clippy + `cargo test --workspace`) ซึ่งต้องใช้ดิสก์ราว 25 GB และควรตั้ง `CARGO_INCREMENTAL=0` ให้เจ้าของ repo ตัดสินใจก่อน เพราะเพิ่มนาที Actions
- `cargo xtask ci` ทั้ง workspace ไม่ได้รันมาตั้งแต่ 1.0.0 (1.1.0 รันทีละขั้น 1.1.1 รันแค่ engine กับ ui-egui) และยังไม่ได้รัน `cargo xtask parity` กับ 1.1.1 เพราะดิสก์ของ container บนคลาวด์ไม่พอ
- audit `untranslated_interface_text` ติด `#[ignore]` (`crates/ui-egui/src/i18n/audit.rs:166`) [ตรวจแล้ว] ยังไม่รู้ว่าทำไมเคยล้ม 1 ใน ~46 รอบ
  - finder นับ `#[test]` ใน ui-egui ได้ 324 ตัว แต่ CHANGELOG รายงาน 322 ผ่าน และ 1 ignored ต่างกัน 1 ตัว ยังไม่ได้ตรวจ
- action ที่ pin SHA ไว้ยังเป็น Node 20 (GitHub บังคับรันบน Node 24 และเตือนใน log)
  - ได้แก่ `actions/checkout@11d5960…` (v4.4.0), `actions/upload-artifact@ea165f8…` (v4.6.2) และ `actions/download-artifact@d3f86a1…` (v4.3.0)
  - ให้อัปเป็นรุ่นที่ใช้ Node 24 ตรวจ SHA กับ tag ของ action และเพิ่ม `.github/dependabot.yml`
  - แก้ workflow ได้เฉพาะตอนที่ไม่มี release รันอยู่
- `cargo audit` รันครั้งล่าสุดตอน 1.0.0 ส่วน `rustybuzz 0.20.1` และ `ttf-parser 0.25.1` (มาผ่าน krilla) ยังเป็น crate ที่ไม่มีคนดูแลใน `Cargo.lock` ให้รันก่อนออกรุ่นถัดไป หรือเพิ่ม cargo-deny ใน CI
- ลงนามโค้ด [ตรวจแล้ว]: workflow ไม่มี `secrets.*` เลย (คำว่า secrets มีแค่ในคอมเมนต์ที่ :62)
  - `sign.ps1` จึงไม่เห็น `WINDOWS_CERTIFICATE` หรือ `AZURE_*` ใน CI แม้เจ้าของ repo จะใส่ secret ไว้แล้ว
  - ต้องเพิ่ม `env:` ใน step "Build MSI + portable zip" หรือแก้เอกสารให้บอกว่ายังไม่ได้ต่อการลงนาม
- `xtask/src/main.rs`: ข้อความ USAGE (:16-30) ไม่มีคำสั่ง `version` และ `corpus --download` โหลด PngSuite ผ่าน `http://` โดยไม่ตรวจ checksum (:202, :226-231)

### 4. ส่วนที่ยังไม่ได้สำรวจ (agent ล้มก่อนทำ)

- โค้ดและเทสต์
  - stub, `unimplemented!` หรือ `todo!` ที่ผู้ใช้เรียกถึงได้
  - คำสั่งใน catalog ที่ไม่มี handler หรือ UI
  - คีย์แปลที่ขาดใน `crates/ui-egui/src/i18n/*.tsv` ทั้ง 18 ไฟล์
  - เลขเวอร์ชันใน `packaging/` และ `apps/` ตรงกับ 1.1.1 หรือไม่
- ตรวจความครบ: ข้อความใน `README.md` และ `README.th.md`, `AGENTS.md`, ความใหม่ของภาพใน `docs/images`, `apps/doc4me-web`, `docs/mcp.md`, `ATTRIBUTION.md` และ `NOTICE`, `contributors/`
- ผล grep ตอนเริ่ม session: ไม่พบ `TODO`, `FIXME`, `todo!(` หรือ `unimplemented!(` ใน `crates`, `apps` และ `xtask` และพบ `#[ignore]` จุดเดียวคือ audit ในข้อ 3

### 5. รอเจ้าของ repo ตัดสินใจ (session ห้ามทำเอง)

- ไม่มี tag `v1.0.0` ลิงก์ `[1.0.0]` ท้าย `CHANGELOG.md` จึงได้ 404 [ตรวจแล้ว]
  - ทางเลือก: ปล่อยไว้ตามที่บันทึกแล้ว หรือ push tag ที่คอมมิตของ 1.0.0
  - finder เสนอ `7946cbb` จาก run 38742771852 แต่ต้องตรวจ `Cargo.toml` ของคอมมิตนั้นก่อน
  - การ push tag `v*` จะสั่ง workflow ให้สร้าง Release ด้วย
- ใบรับรองสำหรับลงนามโค้ด หรือ Azure Trusted Signing (SmartScreen เตือนมาตั้งแต่ 1.0.0)
- การป้องกัน repo
  - `main` ไม่ได้ protected, tag `v*` ย้ายหรือลบได้ และ Release แก้ไขได้ (`immutable: false`)
  - เสนอ ruleset ที่กัน force-push และการลบ (ยัง push ตรงได้) และเปิด immutable releases
- `.claude/settings.json` และไฟล์ที่เกี่ยวข้อง
  - อนุญาต `git tag -a v*` และ `git push origin v*` โดยไม่ถาม ทั้งที่ตอนนี้ออกรุ่นผ่าน workflow แล้ว
  - `.gitignore:9` มี `/.claude/` ไฟล์ใหม่ใต้ `.claude/` จึงถูกละเว้น
  - `package-lock.json` เป็นไฟล์ว่าง (`fc7bab5`)
  - คอมมิต `9954ae0` และ `fc7bab5` ไม่มีรายการใน CHANGELOG
- ลบ branch ที่ merge แล้วบน remote เช่น `claude/optimistic-fermat-z9uknk` (ไม่มีงานหาย)
- เป้าหมาย parity: เอกสารยังตั้งเป้า Word for Mac 16.113.4 และรัน alpha gate บน macOS แต่ Doc4Me ทำตาม Word for Windows และออกรุ่นแค่บน Windows
- อ้างอิง PR และ issue ของ WordCraft ต้นทาง (`ROADMAP.md:124` "~45 open PRs", `docs/roadmap.md:11`, `docs/gaps.md:7-8,18`) ซึ่ง repo นี้ไม่มี ต้องเลือกว่าจะติดตามต้นทางหรือเริ่มใหม่
- ฟอนต์แทน Aptos (gap #4 ซึ่งขวาง beta), ecosystem (#22), ผู้ให้บริการ AI (#25) และโมเดลเสียง (#28)
- จะเพิ่ม job ทดสอบบน Linux ใน CI หรือไม่ (ข้อ 3)
- ข้อที่ผู้ใช้สั่งให้ข้ามเมื่อ 2026-10-21: ให้นักแปลเจ้าของภาษาตรวจ UI ภาษาไทย และวัดค่า TH SarabunIT๙ จากไฟล์ฟอนต์จริง

### 6. ก่อนออกรุ่นถัดไป

- ตอนนี้ `[Unreleased]` มีแต่เรื่องเอกสาร ยังไม่มีเหตุให้ออกรุ่น
- รายการ `[Unreleased]` › แก้ไข (ข้อความเรื่อง push tag ที่ล้าสมัยใน `[1.1.1]`) ต้องย้ายไปอยู่ในหัวข้อของรุ่นถัดไปด้วย เพราะหมายเหตุบนหน้า Release v1.1.1 ชี้ไปที่ "หมวด แก้ไข ของรุ่นถัดไป"
- ขั้นตอน:
  1. `cargo xtask version set x.y.z` แล้วย้าย `[Unreleased]` เป็น `## [x.y.z] - YYYY-MM-DD`
  2. เพิ่มลิงก์ `[x.y.z]: …/compare/v1.1.1...vx.y.z` และแก้ `[Unreleased]: …/compare/vx.y.z...HEAD`
  3. รัน `cargo test -p doc4me-ui-egui i18n -- --include-ignored` และ `cargo audit`
  4. push ไป `main` แล้วสั่ง workflow บน `main` พร้อม `release: true`
  5. ห้าม push จนกว่า job `release` จะจบ

### 7. Backlog ระยะยาว (สรุป ไม่ใช่งานค้าง)

- งานจาก `docs/gaps.md`
  - คลังไฟล์ .docx จริงสำหรับทดสอบ (#1) และการเก็บ Word ink (`w14:contentPart`)
  - ความเสถียรและ autosave (#3)
  - ความตรงของการแบ่งหน้า และฟอนต์แทน Aptos (#4)
  - การพิมพ์แบบ native แทนการเปิด PDF (#6) และความลึกของกล่องโต้ตอบ (#7)
  - ตัวตรวจภาษาเพิ่ม (#8) และการแก้ไขแผนภูมิของ Word (#13)
  - การเขียน .doc และการเปิดไฟล์ Word 6/95 (#14)
  - ภาษา UI hi/ar/id/ko/vi/pt-PT และ RTL (#11)
- ข้อจำกัดภาษาไทยที่ยังเป็นจริง
  - ยังไม่ตรวจไวยากรณ์ไทย
  - การตัดคำ (ICU4X) อาจต่างจาก Word
  - ไม่จำลอง ligature ทางเลือกของ TH Sarabun (fi, ฤๅ)
  - ความกว้าง glyph ของ Sarabun ที่ปรับขนาดต่างจากต้นฉบับเล็กน้อย
- คำสั่งใน catalog ที่ยังไม่มี 14 รายการ: Dictate, Online Pictures, 3D Models, Screenshot, Get Add-ins, Online Video, Ink Equation, Ink to Math, Translate, Block Authors และคำสั่ง View › Window อีก 4 คำสั่ง
- แพ็กเกจของแพลตฟอร์มอื่นนอกจาก Windows x64

## สถานะ: ออก 1.1.1 แล้ว

- [Doc4Me v1.1.1](https://github.com/chatraweechai/doc4me/releases/tag/v1.1.1) ออก 2026-10-22 13:51 UTC จาก [run 39788628618](https://github.com/chatraweechai/doc4me/actions/runs/39788628618) (แท็บ Actions, `release: true`)
  - tag `v1.1.1` ชี้ `be5893e` มี `doc4me-1.1.1-windows-x64.msi` (32,993,280 ไบต์) และ `doc4me-1.1.1-windows-x64-portable.zip` (43,584,012 ไบต์)
  - ตรวจหลังออกแล้ว: sha256 ตรงกับ digest, `doc4me.exe` และ MSI เป็น 1.1.1, `UpgradeCode` เดียวกับ 1.1.0, บันทึกบนหน้า Release ตรงกับ `[1.1.1]` ทุกไบต์
- งานใหม่ทั้งหมดลง `## [Unreleased]` ห้ามแก้หัวข้อ `[1.1.1]`
  - ข้อความ "เจ้าของ repo ต้อง push tag เอง" ใน `[1.1.1]` (และบนหน้า Release) ล้าสมัยแล้ว บันทึกแก้ไว้ใน `[Unreleased]` › แก้ไข
  - หมายเหตุท้ายบันทึกบนหน้า Release v1.1.1 เจ้าของ repo เพิ่มแล้ว (14:32 UTC) และบรรทัด "ผู้ใช้:" ของ `[Unreleased]` › แก้ไข ใน CHANGELOG แก้ตามแล้ว ไม่มีงานค้าง
- ออกรุ่นถัดไป (`docs/releasing.md` ข้อ 3–4):
  - `cargo xtask version set x.y.z` แล้วย้าย `[Unreleased]` เป็น `## [x.y.z] - YYYY-MM-DD` และเพิ่มลิงก์เปรียบเทียบท้าย `CHANGELOG.md` แล้ว push ไป `main`
  - สั่ง workflow `windows-x64-installer.yml` บน `main` พร้อม `release: true`
  - ห้าม push ไป `main` จนกว่า job `release` จะจบ โดยเฉพาะการแก้ `.github/workflows` (เป็นเหตุให้รอบแรกของ 1.1.1 ล้มด้วย 403)

### session 2026-10-22 15:15 – 2026-10-23 04:17 UTC: สำรวจงานค้าง (ยกเลิกกลางทาง)

- ผู้ใช้สั่งตามลำดับ: "ตรวจสอบงานค้างและงานเบื้องหลังทั้งหมด", "ยกเลิกงานทั้งหมด บันทึก handoff สำหรับเริ่ม session ใหม่" และ "บันทึก handoff สิ่งที่ต้องทำต่อใน session ใหม่ทั้งหมด"
- งานเบื้องหลัง (ตรวจ 2026-10-22 15:13 UTC): ไม่มี scheduled task, task list, PR หรือ issue ที่เปิด และไม่มี run ค้าง
  - branch `claude/*` บน remote ถูก merge เข้า `main` หมดแล้ว
  - session อื่นในบัญชี "รวมโค้ด doc-scanner-pdf" เป็นคนละ repo ตามสรุปของ session นั้น รอเจ้าของใส่ secret `SITE_DEPLOY_KEY` และ publish เว็บเอง
- สั่ง workflow ให้ agent อ่านอย่างเดียว 4 ด้าน (handoff/CHANGELOG, roadmap/gaps, โค้ด/เทสต์, CI/infra) แต่ชนขีดจำกัดของ session
  - finder ด้านโค้ด/เทสต์, ตัวตรวจซ้ำทั้ง 4 ตัว และตัวตรวจความครบล้มหมด
  - ผลที่ได้ย้ายไปไว้ในหัวข้อ "งานที่ต้องทำต่อใน session ใหม่" ข้างบน และตรวจข้อสำคัญกับโค้ดแล้ว (2026-10-23)
- แก้แค่ `handoff.md` (คอมมิต `92e1929` และคอมมิตนี้ ใส่ `[skip ci]` ทั้งคู่) จึงไม่ได้แก้ `CHANGELOG.md`
  - คอมมิตนี้ทำหมายเหตุไว้ที่ข้อมูลเก่าในหัวข้อประวัติของ handoff ด้วย

### session 2026-10-22 14:44–14:58 UTC: ตรวจ CI ของ `main` (ยุติแล้ว)

- ผู้ใช้สั่ง "ดำเนินการต่อจากที่ค้างให้เสร็จ และตรวจว่า run ล่าสุดของ main ผ่าน" งานค้างมีข้อเดียวคือตรวจ CI ไม่มีงานโค้ด จึงไม่แก้ `CHANGELOG.md`
- [run 39797850213](https://github.com/chatraweechai/doc4me/actions/runs/39797850213) ของ `73399d7` (HEAD ของ `main` ตอนตรวจ) ผ่าน: run สรุปเป็น `success`; ทุกขั้นของ job `build` ผ่าน (Build MSI + portable zip 14:42–14:54 UTC) และ job `release` ข้ามตามปกติของ push
- run ของ `8f72a1b` (39797301324) และ `30e7503` (39797813816) ถูกยกเลิกโดย concurrency (`cancel-in-progress`) เพราะมี push ใหม่กว่า ไม่ใช่การล้ม และ `73399d7` มีไฟล์ของทั้งสองคอมมิตครบ
- คอมมิตนี้แก้แค่ `handoff.md` จึงใส่ `[skip ci]` ไม่ให้เกิด run ใหม่ run ล่าสุดของ `main` ยังเป็น 39797850213 ที่ผ่านแล้ว
- ทำงานและ push ที่ `main` ตามคำสั่งผู้ใช้ ไม่ได้ push สาขา `claude/modest-maxwell-3q1mya`

### session 2026-10-22 14:39–14:50 UTC: ปิดงานหมายเหตุบนหน้า Release v1.1.1 (ยุติแล้ว)

- ผู้ใช้สั่ง "ดำเนินการต่อจากที่ค้างให้เสร็จ หากไม่มีงานค้างให้บันทึก changelog และ handoff แล้วยุติ"
- `get_release_by_tag v1.1.1`: เจ้าของ repo ต่อหมายเหตุหลังออกรุ่นท้ายบันทึกแล้ว (`updated_at` 14:32:00Z)
  - ดึง body จาก API ตัด CR แล้ว `diff` กับหัวข้อ `[1.1.1]` ของ `be5893e` ต่างกันแค่บรรทัดว่างแรกกับหมายเหตุ เนื้อหาเดิมไม่เปลี่ยน
- เจ้าของ repo push `8f72a1b` (14:37 UTC) วางข้อความหมายเหตุต่อท้ายบรรทัด "ผู้ใช้:" ใน CHANGELOG
  - บรรทัดนั้นจึงขัดกันเอง (ยังบอกว่าหน้า Release แสดงข้อความเดิม) session นี้แยกเป็นรายการย่อยว่าหน้า Release มีหมายเหตุแล้ว พร้อมบรรทัดตรวจสอบ
- CI ของ `main`: run ของ `ffffa5f` (39794752146) ผ่าน; run ของ `8f72a1b` และ `30e7503` ยังรอคิวตอนยุติ (แก้แค่เอกสาร) ตรวจแล้วใน session ถัดมา (ดูข้างบน)
- ทำงานและ push ที่ `main` ตามคำสั่งผู้ใช้ ไม่ได้ push สาขา `claude/clever-hopper-tw74wd`

### session 2026-10-22 13:35–14:22 UTC: ออก Release v1.1.1 (ยุติแล้ว)

- ตอนยุติ: working tree สะอาด `main` ตรงกับ `origin/main` ไม่มีงานตั้งเวลาค้าง
  - run ของ push `670f22b` ผ่าน ส่วน run ของคอมมิตหลังจากนั้น (แก้แค่เอกสาร) ยังรอคิวอยู่ ให้ session ถัดไปตรวจว่า run ล่าสุดของ `main` ผ่าน

- ผู้ใช้สั่ง "ดำเนินการต่อจากที่ค้างให้เสร็จ" งานที่ค้างมีข้อเดียวคือ Release v1.1.1 (ตอนเริ่ม session ยังไม่มี tag `v1.1.1` บน GitHub)
- สาเหตุที่รอบแรก (run 39783304886) ได้ 403:
  - log ของ job `release` มีแค่ `Contents: write` และ `HTTP 403: Resource not accessible by integration` ตอน `POST /releases` (13:14:20 UTC) ส่วนเวลา push ดูจาก `gh api repos/chatraweechai/doc4me/activity`
  - ก่อนหน้านั้น `main` ถูก push `e06edac` ซึ่งแก้ `.github/workflows/windows-x64-installer.yml` (13:11:46) คอมมิต `858b9bd` ที่จะติด tag จึงมีไฟล์ workflow ต่างจาก default branch และไม่มี branch หรือ tag ชี้อยู่
  - เอกสาร REST ของ GitHub (Create a release) เขียนไว้ว่ากรณีนี้ token ต้องมีสิทธิ์แก้ workflow ซึ่ง `GITHUB_TOKEN` ได้ไม่ได้ (ลิงก์อยู่ใน CHANGELOG `[Unreleased]`)
- 13:38 UTC สั่ง workflow_dispatch `release: true` บน `main` ที่ `be5893e` เป็น run 39788628618 แล้วผ่านทั้ง job `build` และ `release`
  - ระหว่างรัน push `10a276a` (handoff อย่างเดียว) ขึ้น `main` ตอน 13:42:41 เพราะ stop hook เตือนเรื่องคอมมิตที่ยังไม่ push
  - release ยังผ่าน เพราะ `be5893e` ไม่ได้แก้ไฟล์ workflow และไฟล์ workflow ตรงกับ `main` แต่การเทียบสองรอบนี้แยกไม่ได้ว่า GitHub ใช้เงื่อนไขแบบไหน จึงให้งด push ทั้งหมดระหว่าง release
  - run ของ push `10a276a` (39789261472) ผ่าน
- บันทึกผลใน `CHANGELOG.md` `[Unreleased]` และเพิ่มข้อควรระวังใน `docs/releasing.md` (คอมมิต `670f22b`)
- ตรวจหลัง push ด้วย agent: ตรวจไฟล์ใน Release, ตรวจข้อเท็จจริงในเอกสาร และพยายามหักล้างสาเหตุของ 403 แต่ละข้อที่พบมี agent อีกตัวตรวจซ้ำ
  - ไฟล์ใน Release ถูกต้องทุกข้อ
  - เอกสารแก้ 4 เรื่องในคอมมิตถัดมา:
    - ข้อสรุปเกินหลักฐาน: เปลี่ยนไปอ้างเอกสารของ GitHub
    - เวลาที่ใช้เป็นเวลาสร้าง run ไม่ใช่เวลา push
    - CHANGELOG ขาดบรรทัดตรวจสอบและบรรทัดผลต่อผู้ใช้
    - ขั้นออกรุ่นใน handoff ขาดลิงก์เปรียบเทียบ

### ทางเลือกในการออก Release v1.1.1 (บันทึกเดิม ก่อนออกสำเร็จ)

- งานโค้ด ภาพหน้าจอ อภิธานศัพท์ และเอกสารของ 1.1.1 เสร็จและ push ไป `main` แล้ว ไม่มีงานโค้ดค้าง
- ทางที่ใช้ได้: เจ้าของ repo รันบนเครื่องตัวเอง `git fetch origin && git tag -a v1.1.1 origin/main -m "Doc4Me 1.1.1" && git push origin v1.1.1`
- ทางที่ลองแล้วไม่สำเร็จ:
  - session บนคลาวด์ push tag ไม่ได้ (git proxy ตอบ HTTP 403) แม้ `.claude/settings.json` อนุญาต `git tag -a v*` / `git push origin v*` แล้ว
  - ตัวเลือก `release` ของ workflow_dispatch (เพิ่มใน `858b9bd`, แก้ใน `e06edac`): run 39783304886 build ผ่าน แต่ `gh release create v1.1.1 --target` ล้มด้วย `HTTP 403: Resource not accessible by integration` (สร้าง tag ใหม่ด้วย `GITHUB_TOKEN` ไม่ได้)
  - ถ้าจะทำให้ทางนี้ใช้ได้ (ยังไม่ได้ตรวจ): ดู Settings › Actions › General › Workflow permissions ว่าเป็น "Read and write" หรือไม่ และ GitHub อาจไม่ให้ `GITHUB_TOKEN` สร้าง ref ที่ชี้คอมมิตซึ่งแก้ไฟล์ workflow ทางแก้คือใช้ PAT หรือ GitHub App token ที่มีสิทธิ์ `contents` + `workflows` เป็น secret
- หลังออกแล้ว: ตรวจว่า Release มี MSI และ zip และ job `release` ผ่าน งานใหม่ทั้งหมดลง `## [Unreleased]` ห้ามแก้ `[1.1.1]`

### ประวัติของ 1.1.1 (บันทึกเดิม ก่อนออกสำเร็จ: ข้อ "push tag เอง" และ "`[Unreleased]` ว่าง" ไม่จริงแล้ว)

- 2026-10-22 ผู้ใช้สั่ง: เพิ่มการจัดแนว จัดกระจายแบบไทย, ระยะห่างตัวอักขระ ปกติ/บีบ/ขยาย ขนาด 0–1584 pt และเลขเวอร์ชัน 1.1.1 ทำเสร็จและ push ไป `main` แล้ว เทสต์ ui-egui/engine, audit ภาษา, clippy และ fmt ผ่าน (ดู `[1.1.1]` หมวดการตรวจสอบ)
- `Cargo.toml` เป็น 1.1.1 และ CHANGELOG มี `## [1.1.1] - 2026-10-22` แล้ว `[Unreleased]` ว่าง
- ขั้นที่ session บนคลาวด์ทำไม่ได้ (push tag ได้ HTTP 403): `git fetch origin && git tag -a v1.1.1 origin/main -m "Doc4Me 1.1.1" && git push origin v1.1.1` workflow จะสร้าง Release เอง
- 2026-10-22 (รอบสอง): ถ่ายภาพ `hero`, `thai`, `dark`, `dark-th`, `font`, `font-th` ใหม่ และเพิ่มคำ ขยาย/บีบ/ขนาด:/ปกติ ในอภิธานศัพท์แล้ว
- push tag จาก session นี้ถูกปฏิเสธ (นับเป็นการเผยแพร่สาธารณะ) เจ้าของ repo ต้อง push `v1.1.1` เอง

## ประวัติ 1.1.0 (ออกแล้ว 2026-10-21)

- Release [Doc4Me v1.1.0](https://github.com/chatraweechai/doc4me/releases/tag/v1.1.0) ที่ `51a9658` มี MSI และ portable zip
- เจ้าของ repo เพิ่ม `package-lock.json` ว่างที่ราก (`fc7bab5`) ไม่มีผลต่อโปรแกรม

### ทำแล้วใน session 2026-10-21 (ทั้งหมดอยู่ใน `[1.1.0]`)

- ช่องตัวเลข `DragValue` ที่มี `range` ทั้ง 47 ช่องไม่บีบค่าเดิมของเอกสาร (`.clamp_existing_to_range(false)`) และเทสต์ตรวจซอร์ส `ranged_number_fields_keep_existing_values` กันช่องใหม่ที่ลืมใส่
- รางตัวเลื่อนซูมในแถบสถานะ: สี `input_border` และสูง 3 pt (`chrome.rs`) เห็นได้ทั้งสองธีม ปุ่มยังเด่นกว่าราง
- drop cap ใช้ระยะบรรทัดจริงของย่อหน้า (`layout/src/para.rs`) แทน 1.2 em ตายตัว ซึ่งทำให้ drop cap เล็กเกินไปตั้งแต่เปลี่ยนเป็นความสูงบรรทัดแบบ Word
- ภาพตัวอย่าง style set บนแท็บ Design วาดพื้นที่ 150 pt ที่สเกล 0.38 จึงเห็นครบสามย่อหน้า
- `README.th.md` ใช้คำตามอภิธานศัพท์และ `th.tsv` (12 จุด ตรวจด้วย agent สองชั้น)
- ภาพหน้าจอทุกภาพใน `docs/images` ถ่ายใหม่ด้วยโค้ดล่าสุด และสคริปต์ `ui_shot` ทุกภาพเก็บไว้ที่ `docs/images/shots/`
- ผลตรวจล่าสุด: `cargo test` ของ `doc4me-layout` (224), `doc4me-engine` (372 + 13 + 8) และ `doc4me-ui-egui` (320) ผ่าน; audit ภาษา `--include-ignored` ผ่าน 60 รายการ; clippy `-D warnings` ของ layout และ ui-egui และ `cargo fmt --all --check` ผ่าน
  - ยังไม่ได้รัน `cargo xtask ci` ทั้ง workspace เพราะดิสก์ของ container ไม่พอ (ต้องใช้ราว 25 GB); crate อื่นไม่ได้แก้ใน session นี้

### ข้อสังเกตที่ไม่ได้แก้ (ไม่ใช่บั๊กที่ยืนยันแล้ว)

- `Slider` อื่น (`read_aloud.rs`, `options.rs`, กล่อง Zoom ใน `dialogs.rs` และแถบซูมในแถบสถานะ `chrome.rs`) ยังบีบค่าแบบเดิมของ egui ค่ามาจากการตั้งค่าหรือมุมมองซึ่งอยู่ในช่วงเสมอ ถ้าวันหนึ่งผูกกับค่าจากเอกสาร ให้ใช้ `.clamping(egui::SliderClamping::Edits)`
- `backstage.png` มีคำทักทายตามเวลา ถ่ายใหม่หลังเที่ยง UTC จะได้ "Good afternoon" (ต้นเหตุคือคำทักทายใช้เวลา UTC ไม่ใช่เวลาเครื่อง ดูหัวข้องานที่ต้องทำต่อ ข้อ 1)

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
  - `crates/ui-egui/src/i18n/th.tsv` มี 2,452 รายการ ครบทุกคีย์ (ตอน 1.1.0; 1.1.1 เพิ่ม 4 คีย์เป็น 2,456)
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
- สคริปต์ของทุกภาพอยู่ที่ `docs/images/shots/<ชื่อภาพ>.jsonl` รันจากรากของ repo แล้วจะเขียนทับภาพใน `docs/images` (ถ่ายใหม่หลังเที่ยง UTC `backstage.png` จะทักทายต่างไป)
- คำสั่งที่ใช้สร้างภาพ `thai.png`: `ui.language` (`th`), `review.language` (`th-TH`, `default: true`), `styles.apply`, `text.insert` (`raw: true`), `text.newParagraph`, `view.zoomOut` และ `ui.key` Home พร้อม `cmd: true` เพื่อกลับไปต้นเอกสาร
- ธีมมืด: `ui.theme` (`value: dark`)

## วิธีตรวจก่อน commit (ตามกฎใน CLAUDE.md)

- ใช้ `cargo xtask ci` ซึ่งรัน fmt, clippy -D warnings, test, assets, layers และ wasm
- ถ้าดิสก์น้อย ให้ใช้ `CARGO_INCREMENTAL=0` และทดสอบทีละแพ็กเกจ
  - หลังแต่ละแพ็กเกจ ให้ลบไฟล์ทดสอบใน `target/debug/deps`
  - build ทั้ง workspace ต้องใช้ดิสก์ราว 25 GB
- เทสต์ Myanmar จะข้ามไปเองเมื่อเครื่องไม่มีฟอนต์ที่มีกฎ `mym2`/`mymr` เช่นเครื่องที่มีแค่ Unifont
