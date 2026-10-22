# บันทึกการเปลี่ยนแปลง (Changelog)

บันทึกการเปลี่ยนแปลงสำคัญทั้งหมดของ Doc4Me รูปแบบอิง [Keep a Changelog 1.1.0](https://keepachangelog.com/en/1.1.0/)
และเลขเวอร์ชันใช้ [Semantic Versioning](https://semver.org/) (แหล่งเดียวคือ `[workspace.package] version` ใน `Cargo.toml`)

กติกา (`CLAUDE.md` ข้อ 3):

- ทุกการเปลี่ยนแปลงโค้ด build packaging asset หรือเวอร์ชัน ต้องบันทึกใต้ `## [Unreleased]` ใน commit เดียวกัน
- ก่อน push tag `vX.Y.Z` ให้ย้ายรายการไปเป็น `## [X.Y.Z] - YYYY-MM-DD`
- workflow `windows-x64-installer.yml` นำเนื้อหาใต้หัวข้อนั้นไปเป็นบันทึกของ GitHub Release

## [Unreleased]

### เปลี่ยนแปลง

- **คู่มือออกรุ่น: ห้าม push การแก้ไฟล์ workflow ระหว่างที่ release จากแท็บ Actions กำลังรัน** (`docs/releasing.md` ข้อ 4)
  - เหตุผล: รอบแรกของ Release v1.1.1 (run 39783304886) ล้มด้วย `HTTP 403: Resource not accessible by integration` ตอนสร้าง tag ที่ `858b9bd` เพราะก่อน job `release` จะรัน (13:14:20 UTC) `main` ถูก push `e06edac` ซึ่งแก้ `.github/workflows/windows-x64-installer.yml` ไปแล้ว (13:11:49 UTC) tag ใหม่จึงชี้คอมมิตที่ไฟล์ workflow ต่างจาก `main` และ `GITHUB_TOKEN` ของ job ไม่มีสิทธิ์ `workflows`
  - รอบที่สอง (run 39788628618 ที่ `be5893e`) ผ่าน ทั้งที่ระหว่างรัน `main` ขยับไป `10a276a` (13:42:43 UTC แก้แค่ `handoff.md`) จึงสรุปว่าตัวการคือไฟล์ workflow ที่ต่างกัน ไม่ใช่การที่ `main` ขยับ ข้อสรุปนี้มาจากการเทียบสองรอบ ไม่ได้มาจากเอกสารของ GitHub
  - คู่มือบอกวิธีแก้ด้วย: สั่ง run ใหม่บน `main` ล่าสุด เพราะรอบที่ล้มไม่ได้สร้าง tag หรือ Release
  - ผู้ใช้: ไม่มีผลต่อโปรแกรม ไฟล์ .docx หรือ installer

### แก้ไข

- **ข้อมูลใน `[1.1.1]` ที่ล้าสมัย: Release v1.1.1 ออกจากแท็บ Actions ได้แล้ว** (บันทึกเป็นรายการใหม่ตามกติกา ไม่แก้หัวข้อ `[1.1.1]`)
  - หัวข้อ `[1.1.1]` › เปลี่ยนแปลง › ข้อจำกัดที่ทราบ บอกว่าออก Release ผ่าน workflow ไม่ได้และเจ้าของ repo ต้อง push tag เอง ตอนนี้ไม่จริงแล้ว: [Doc4Me v1.1.1](https://github.com/chatraweechai/doc4me/releases/tag/v1.1.1) ออกเมื่อ 2026-10-22 13:51 UTC โดย [run 39788628618](https://github.com/chatraweechai/doc4me/actions/runs/39788628618) (workflow_dispatch `release: true`)
  - tag `v1.1.1` ชี้ `be5893e` มีไฟล์ `doc4me-1.1.1-windows-x64.msi` (32,993,280 ไบต์) และ `doc4me-1.1.1-windows-x64-portable.zip` (43,584,012 ไบต์) บันทึกประจำรุ่นคือหัวข้อ `[1.1.1]` ของ `be5893e`
  - ตรวจสอบ: job `build` และ `release` ของ run 39788628618 ผ่าน; `get_release_by_tag v1.1.1` ได้ `target_commitish` = `be5893e`, `draft: false`, ไฟล์สองไฟล์สถานะ `uploaded`; `git fetch --tags` แล้ว `git rev-parse v1.1.1^{commit}` ได้ `be5893e`

## [1.1.1] - 2026-10-22

### เพิ่ม

- **ปุ่มจัดแนว "จัดกระจาย" และ "จัดกระจายแบบไทย" บนริบบอน และในกล่อง Paragraph** (`crates/ui-egui/src/ribbon.rs`, `icons.rs`, `dialogs.rs` `ALIGNS`, `crates/engine/src/cmd/para.rs`, `crates/engine/src/catalog.rs`, `docs/parity-checklist.md`)
  - เหตุผล: เอกสารรองรับ `w:jc="thaiDistribute"` และ `distribute` อยู่แล้ว (อ่าน เขียน จัดหน้า) แต่ผู้ใช้เลือกได้เฉพาะผ่านคำสั่ง `para.align` หรือ Ctrl+Shift+J
  - เพิ่มคำสั่ง `para.thaiDistribute` (Home › Paragraph, กดซ้ำกลับเป็นชิดซ้ายเหมือนปุ่มจัดแนวอื่น) และไอคอน `distribute` / `thaiDistribute`; กล่องตัวเลือกการจัดแนวในกล่อง Paragraph มี จัดกระจาย และ จัดกระจายแบบไทย
  - ผู้ใช้: ไฟล์ .docx เดิมไม่เปลี่ยน ค่าที่เลือกบันทึกเป็น `w:jc` ตามมาตรฐาน (RTF/ODT/HTML บันทึกเป็น justify ตามเดิม); แถบ Paragraph กว้างขึ้นสองปุ่ม
- **ระยะห่างตัวอักขระแบบ ปกติ / ขยาย / บีบ ปรับขนาดได้ 0–1584 pt** (กล่อง Font หน้า Advanced, `crates/ui-egui/src/dialogs.rs` `spacing_kind`, `spacing_value`)
  - เดิมเป็นช่องตัวเลขเดียว −20 ถึง 20 pt ตอนนี้เลือกชนิดแล้วกรอกขนาดได้ถึง 1584 pt ซึ่งเป็นค่าสูงสุดที่ `w:spacing` เก็บได้ (±31680 twips) เหมือน Word; เปลี่ยนจากปกติเป็นขยายหรือบีบเริ่มที่ 1 pt
  - คำสั่ง `format.spacing` รับ ±1584 pt (เดิมบีบไว้ ±100 pt ขณะที่ตัวอ่าน/เขียน .docx รับ ±1584 อยู่แล้ว)
  - คำแปลใหม่ใน 18 ภาษา: `Expanded`, `Condensed`, `By:` และ `spacing` › `Normal` (ไทย: ขยาย, บีบ, ขนาด:, ปกติ) ป้าย `Spacing (pt):` เปลี่ยนเป็น `Spacing:` ที่มีคำแปลอยู่แล้ว
  - ผู้ใช้: ค่าที่มีอยู่ในเอกสาร รวมถึงค่านอกช่วง ไม่ถูกเปลี่ยนเมื่อแค่เปิดหน้า Advanced ไฟล์ .docx เดิมไม่กระทบ

- **อภิธานศัพท์ภาษาไทย: คำของระยะห่างตัวอักขระ** (`docs/localization-th.md`)
  - เพิ่ม Expanded = ขยาย, Condensed = บีบ, By: = ขนาด:, Normal (context `spacing`) = ปกติ พร้อมเหตุผลที่เลือก และตัวเลขแค็ตตาล็อกเป็น 2,456 รายการ เป้าหมาย Doc4Me 1.1.1
  - ไม่มีผลต่อโปรแกรม ตรวจสอบ: นับรายการใน `th.tsv` ได้ 2,456 (`grep -vc '^#\|^$'`)

- **ภาพหน้าจอใน README ถ่ายใหม่ด้วย 1.1.1** (`docs/images/{hero,thai,dark,dark-th,font,font-th}.png`)
  - เหตุผล: แถบย่อหน้าบนริบบอนมีปุ่มจัดกระจายและจัดกระจายแบบไทยเพิ่ม และหน้าขั้นสูงของกล่องฟอนต์เปลี่ยนเป็นช่องระยะห่าง ปกติ/ขยาย/บีบ กับช่องขนาด
  - ถ่ายด้วยสคริปต์เดิมใน `docs/images/shots/` (ไม่ได้แก้สคริปต์) ภาพอื่นไม่ได้ถ่ายใหม่เพราะไม่แสดงแถบหน้าหลักหรือกล่องฟอนต์
  - ตรวจสอบ: `ui_shot` ทั้ง 6 สคริปต์จบด้วย exit 0 และเปิดดู `thai.png` กับ `font-th.png` แล้วเห็นปุ่มใหม่ และ ระยะห่าง: ปกติ / ขนาด: 0.0 pt (ปิดใช้งานเมื่อเป็นปกติ)

### แก้ไข

- **กล่อง Paragraph แสดงการจัดแนว "จัดกระจายแบบไทย" เป็น `thaidistribute`** (`crates/ui-egui/src/dialogs.rs` `align_value`)
  - สาเหตุ: ชื่อมาจาก `format!("{:?}").to_lowercase()` ซึ่งไม่ตรงกับค่า `thaiDistribute` ที่ `para.align` รับ กด OK แล้วคำสั่งตอบ error
  - แก้: แปลง `Align` เป็นค่าของ `para.align` โดยตรง

### เปลี่ยนแปลง

- **เวอร์ชัน 1.1.1** (`Cargo.toml`, `Cargo.lock` เฉพาะแพ็กเกจใน workspace ด้วย `cargo xtask version set 1.1.1`) installer ติดตั้งทับ 1.1.0 ได้ (`UpgradeCode` เดิม)
- **ออก Release จากแท็บ Actions ได้โดยไม่ต้อง push tag** (`.github/workflows/windows-x64-installer.yml`, `docs/releasing.md`)
  - เหตุผล: session บนคลาวด์ push tag ไม่ได้ (git proxy ตอบ HTTP 403) เจ้าของ repo สั่งให้ workflow สร้าง tag เองและมีสิทธิ์ `contents: write`
  - `workflow_dispatch` มีตัวเลือก `release` (boolean, ค่าเริ่มต้นปิด) เมื่อรันบน `main` และติ๊กไว้ job `release` จะอ่านเวอร์ชันจาก `Cargo.toml` (ต้องเป็นรูป x.y.z) แล้วสร้าง Release `Doc4Me v<เวอร์ชัน>` พร้อม tag ที่คอมมิตที่ build (`gh release create --target "$GITHUB_SHA"`) บันทึกประจำรุ่นมาจากหัวข้อของเวอร์ชันนั้นใน CHANGELOG เหมือนการ push tag
  - ความปลอดภัย: สิทธิ์ `contents: write` มีเฉพาะ job `release` (job build และค่าเริ่มต้นของ workflow ยังเป็น `contents: read`); หยุดถ้า tag มีอยู่แล้ว จึงไม่ย้ายหรือใช้ tag เดิมซ้ำ; รันจากสาขาอื่นไม่ได้; ตัวเลือกเป็น boolean ใช้แค่ในเงื่อนไข `if` ไม่ได้แทรกลงในสคริปต์; tag ที่สร้างด้วย `GITHUB_TOKEN` ไม่ทริกเกอร์ workflow ซ้ำ จึงไม่เกิด Release ซ้อน; ผู้ที่รัน workflow ได้ (สิทธิ์ write ของ repo) ออก Release ได้
  - กลุ่ม concurrency แยกตามชนิด event เพื่อไม่ให้ push ไป `main` ยกเลิกการออก Release ที่สั่งจากแท็บ Actions
  - การ push tag `v*` ทำงานเหมือนเดิม; ไม่มีผลต่อโปรแกรมหรือไฟล์ .docx
  - ตรวจสอบ: แปลง YAML ด้วย Ruby ได้ เงื่อนไข `if` และ input ถูกต้อง; คำสั่ง `sed` อ่านเวอร์ชันจาก `Cargo.toml` ได้ `1.1.1`; ผลรันจริงบันทึกในคอมมิตถัดไป
  - ตรวจบั๊กหลัง push: ขั้นตรวจว่ามี tag แล้วหรือไม่เดิมถือทุกความล้มเหลวของ `gh api` (สิทธิ์ผิด, rate limit, GitHub ล่ม) ว่า "ไม่มี tag" แล้วออก Release ต่อ ซึ่งอาจผูก Release กับ tag เดิมที่อยู่คนละคอมมิต แก้ให้ผ่านเฉพาะเมื่อได้ `HTTP 404` กรณีอื่นหยุดพร้อมข้อความ error; ยืนยันรูปแบบข้อความด้วย `gh api repos/chatraweechai/doc4me/git/ref/tags/v9.9.9` ได้ `gh: Not Found (HTTP 404)` และ YAML ยังแปลงได้
  - ตรวจโค้ดฝั่งแอปของ 1.1.1 ซ้ำ (ปุ่มจัดแนว, `para.thaiDistribute`, ช่องระยะห่าง) ไม่พบบั๊กหรือช่องโหว่เพิ่ม
  - ข้อจำกัดที่ทราบ: รันจริงครั้งแรก ([run 39783304886](https://github.com/chatraweechai/doc4me/actions/runs/39783304886), คอมมิต `858b9bd`, `release: true`) job build ผ่าน แต่ job `release` ล้มที่ `gh release create v1.1.1 --target` ด้วย `HTTP 403: Resource not accessible by integration` จึงไม่มี tag และ Release v1.1.1
    - การสร้าง Release บน tag ที่มีอยู่แล้วด้วยสิทธิ์เดียวกันเคยสำเร็จ (Release v1.1.0) จุดที่ต่างคือการสร้าง tag ใหม่ผ่าน `GITHUB_TOKEN` สาเหตุที่น่าจะเป็น (ยังไม่ยืนยัน): GitHub ไม่ให้ `GITHUB_TOKEN` สร้าง ref ที่ชี้คอมมิตซึ่งแก้ไฟล์ใน `.github/workflows` เพราะ token นี้ไม่มีสิทธิ์ `workflows` หรือการตั้งค่า Actions ของ repo จำกัดสิทธิ์ token
    - ทางที่ใช้ได้ตอนนี้: เจ้าของ repo push tag จากเครื่องตัวเอง (`git tag -a v1.1.1 origin/main -m "Doc4Me 1.1.1" && git push origin v1.1.1`) แล้ว job `release` จะสร้าง Release บน tag นั้นเหมือน 1.1.0

### การตรวจสอบ

- เทสต์ใหม่ `font_spacing_is_normal_expanded_or_condensed_up_to_1584_pt`, `paragraph_dialog_shows_and_sets_thai_distributed` และกรณีจัดกระจาย/จัดกระจายแบบไทยใน `paragraph_dialog_ok_keeps_what_it_shows`
- ผลการรันเทสต์บันทึกในคอมมิตถัดไป (commit ก่อนตรวจตามคำสั่งผู้ใช้)
- ผลตรวจหลัง push: `cargo test -p doc4me-ui-egui` ผ่าน 322 (ignore 1 คือ audit ภาษา); `cargo test -p doc4me-ui-egui i18n -- --include-ignored` ผ่าน 60 (รวม audit `untranslated_interface_text` และเทสต์ครอบคลุมคีย์ของทุกภาษา); `cargo test -p doc4me-engine` ผ่าน 372 + 13 + 8; `cargo clippy -p doc4me-ui-egui -p doc4me-engine --all-targets -- -D warnings` และ `cargo fmt --all --check` ผ่าน
- ตรวจทาน diff แล้วไม่พบบั๊กหรือช่องโหว่เพิ่ม: ค่าระยะห่างที่ป้อนผ่าน UI อยู่ใน ±1584 pt และตัวเขียน .docx บีบไว้ที่ ±1584 อยู่แล้ว; `docs/parity-checklist.md` แก้ตัวเลขด้วยมือ (484/499, Home 103/104) เพราะไม่ได้รัน `cargo xtask parity`

## [1.1.0] - 2026-10-21

### เปลี่ยนแปลง

- **ออกเวอร์ชัน 1.1.0** (`Cargo.toml` `[workspace.package] version`, `Cargo.lock`, `CHANGELOG.md`, `ROADMAP.md`, `docs/localization-th.md`)
  - เหตุผล: รวมงานภาษาไทยและความเที่ยงตรงแบบ Word หลัง 1.0.0 ไว้ในเวอร์ชันที่ติดตั้งได้ ได้แก่ UI ภาษาไทย ตัวตรวจตัวสะกดไทย เลขลำดับไทย ระยะบรรทัดแบบ Word บน Windows TH Sarabun ขนาดเท่าต้นฉบับ และ kerning/ligature ตามค่าในเอกสาร
  - ตั้งเลขด้วย `cargo xtask version set 1.1.0` ซึ่งเปลี่ยน `Cargo.lock` เฉพาะแพ็กเกจใน workspace ไม่ได้อัปเกรด dependency
  - ผู้ใช้: installer MSI ใช้ `UpgradeCode` เดิม จึงติดตั้งทับ Doc4Me 1.0.0 ได้ ไฟล์ .docx เดิมเปิดได้เหมือนเดิม แต่เอกสารที่ใช้ Sarabun จะแบ่งหน้าตามระยะบรรทัดแบบ Word บน Windows (ดูรายการถัดไป)
  - `ROADMAP.md`: แก้แถว Releases และย้ายแถว Progress log วันที่ 2026-10-11 สองแถวที่อยู่ระหว่างหัวตารางกับบรรทัด `|---|---|` ลงไปใต้หัวตาราง เพราะเดิมตารางนี้แสดงผลไม่ได้
  - ข้อจำกัด: GitHub ยังไม่มี tag `v1.0.0` (1.0.0 build ผ่าน Actions แต่ไม่ได้ tag) ลิงก์ `[1.1.0]` ท้ายไฟล์จึงชี้ไปหน้า Release แทนหน้าเปรียบเทียบ ส่วนลิงก์ `[1.0.0]` เดิมเปิดไม่ได้และคงไว้ตามกฎไม่แก้ประวัติของเวอร์ชันที่ออกไปแล้ว
  - ตรวจสอบ: workflow `windows-x64-installer.yml` บน `main` ที่คอมมิต 307a25b build MSI และ zip สำเร็จ; หลังตั้งเลข `cargo xtask version` แสดง 1.1.0
- **`.gitignore` ไม่ติดตามไฟล์ log ของ plugin** (`.anchor/`, `.harnesskit/`, `.viberevert/`)
  - เหตุผล: plugin ของ session สร้างไดเรกทอรีเหล่านี้ไว้ในเครื่อง ทำแบบเดียวกับ `.simple-workflow/` ที่มีอยู่แล้ว ไม่มีผลต่อโปรแกรม
- **ระยะบรรทัดแบบ Word บน Windows** (`crates/fonts/src/word.rs` `line_metrics`)
  - ระยะบรรทัดแบบ Single ใช้ค่าจาก OS/2 `usWinAscent` + `usWinDescent` บวก external leading (ส่วนของ line gap ใน hhea ที่ค่า win ยังไม่ครอบคลุม) ไว้เหนือ ascent ซึ่งเป็นวิธีที่ Word บน Windows วัด (GDI)
  - แต่ก่อนใช้ค่า hhea แบบ Word บน Mac ทำให้ Sarabun มีระยะบรรทัด 1.30 em แต่ Word บน Windows ใช้ 1.853 em เอกสารเดียวกันจึงยาวไม่เท่ากัน
  - ตอนนี้ใช้กติกาเดียวกันทุกแพลตฟอร์ม จำนวนบรรทัดต่อหน้า จำนวนหน้า และงานพิมพ์ จึงตรงกับ Word บน Windows ไม่ว่าจะเปิด แก้ไข หรือบันทึกด้วยโปรแกรมใด
  - ฟอนต์ที่ไม่มีตาราง OS/2 หรือค่า win เป็นศูนย์ หรือเกิน 4 em ยังใช้ค่า hhea เหมือนเดิม
  - ฟอนต์ที่ค่า win เท่ากับ hhea (เช่น Times New Roman 1.15 em, Calibri 1.22 em) ได้ผลเหมือนเดิม
  - ผลกระทบ: เอกสารที่ใช้ Sarabun มีระยะบรรทัดกว้างขึ้นเท่ากับใน Word บน Windows (Word บน Mac ยังวัดแบบ hhea จึงจะได้หน้าสั้นกว่า)
- **Kerning และ ligature ตามค่าในเอกสารแบบ Word** (`crates/layout/src/para.rs` `run_features`)
  - ใช้ pair kerning เฉพาะเมื่อตั้ง "Kerning for fonts" (`w:kern`) และขนาดตัวอักษรถึงเกณฑ์ที่ตั้งไว้
  - ใช้ ligature แบบ standard, contextual, historical และ discretional เฉพาะเมื่อ `w14:ligatures` สั่ง
  - ทั้งสองอย่างเป็นค่าเริ่มต้นของ Word (ปิด) แต่ก่อน Doc4Me เปิดไว้เสมอ ความกว้างบรรทัดจึงต่างจาก Word เล็กน้อย
  - เพิ่ม `CharProps::ligatures` / `ResolvedChar::{kern, ligatures}` และอ่าน/บันทึก `w14:ligatures` ใน .docx ได้ครบ ค่าที่ไม่รู้จักจะถูกทิ้งแทนการเขียนกลับ
- **ปรับรายการข้อจำกัดที่ทราบของ `[Unreleased]`** (`CHANGELOG.md`)
  - เอาออก 3 รายการที่แก้แล้วในรอบเดียวกัน: kerning ใน ODT/RTF/.doc, ช่อง Kerning/Ligatures ในกล่อง Font และข้อความ TH Sarabun ในแผนภูมิ SmartArt และ m:nor (ดูหมวด เพิ่ม) เพื่อไม่ให้บันทึกประจำรุ่นขัดกันเอง ไม่มีผลต่อโปรแกรม

### เพิ่ม

- **`README.md` และ `README.th.md` พร้อมภาพหน้าจอของ 1.1.0 ทั้งหน้าจอภาษาอังกฤษและภาษาไทย** (`README.md`, `README.th.md`, `docs/images/{hero,font,dark,thai,font-th,dark-th}.png`, `ATTRIBUTION.md`)
  - เหตุผล: เจ้าของ repo สั่งให้ทำ README พร้อมภาพหน้าจอ ทั้งแบบหน้าภาษาไทยและภาษาอังกฤษ (README เดิมถูกลบที่คอมมิต `3cccba7`)
  - สองหน้ามีลิงก์สลับภาษากันที่หัวหน้า เนื้อหาเดียวกัน: Doc4Me คืออะไร สถานะ alpha ที่ดาวน์โหลด (หน้า Releases) ความสามารถหลัก วิธี build สัญญาอนุญาต และข้อความว่าไม่เกี่ยวข้องกับ Microsoft; หน้าไทยมีหัวข้อภาษาไทยใน 1.1.0 และใช้ศัพท์เดียวกับ `th.tsv`
  - ภาพหน้าจอถ่ายด้วย `ui_shot` ขนาด 1440×900 pt ที่ 2× เป็นคู่ฉากเดียวกัน
    - หน้าต่างหลัก: `hero.png` (หน้าจออังกฤษ เอกสารตัวอย่าง) และ `thai.png` (หน้าจอไทย เอกสารภาษาไทยที่เขียนขึ้นสำหรับ repo นี้ ภาษาเอกสาร th-TH ซูม 90% เห็นระยะบรรทัดแบบ Word บน Windows และคำว่า อนุญาติ ที่ตัวตรวจตัวสะกดไทยขีดเส้นใต้)
    - หน้าขั้นสูงของกล่องฟอนต์ที่ตั้งเคิร์นนิง 12 pt และลิเกเจอร์แบบมาตรฐานและตามบริบท: `font.png` และ `font-th.png`
    - ธีมมืด: `dark.png` และ `dark-th.png`
    - ภาพทั้ง 11 ภาพใน `docs/images` ถ่ายใหม่ด้วยโค้ดสุดท้ายของรุ่นนี้ (รางตัวเลื่อนซูมแบบใหม่ ระยะบรรทัดของ 1.1.0) ฉากเดียวกับภาพเดิม
      - ภาพที่ README ใช้ (`hero`, `thai`, `font`, `font-th`, `dark`, `dark-th`) ต่างจากภาพเดิมเฉพาะบริเวณตัวเลื่อนซูม (เทียบพิกเซลแล้ว) ยกเว้น `hero.png` ที่มีเคอร์เซอร์ข้อความที่ต้นชื่อเรื่องเหมือน `dark.png`
      - ภาพที่ README ไม่ได้ใช้ (`backstage`, `design`, `layout`, `references`, `review`) เปลี่ยนตามระยะบรรทัดของ 1.1.0; `layout.png` และ `design.png` ถ่ายหลังแก้ drop cap และภาพตัวอย่าง style set (สองข้อนี้พบจากการถ่ายภาพชุดนี้ครั้งแรก) ตรวจด้วยตาแล้วว่า drop cap คลุมสามบรรทัดและแกลเลอรีเห็นครบสามย่อหน้า
      - `review.png` เลื่อนจอลงเพื่อให้เห็นข้อคิดเห็นทั้งสามในขอบ จึงไม่เห็นหัวกระดาษและชื่อเรื่องเหมือนภาพเดิม; `backstage.png` มีคำทักทายตามเวลา (ถ่ายก่อนเที่ยง UTC)
    - สคริปต์ `ui_shot` ของทุกภาพเก็บไว้ที่ `docs/images/shots/<ชื่อภาพ>.jsonl` (ใหม่) รันจากรากของ repo เช่น `target/debug/examples/ui_shot docs/images/shots/layout.jsonl` เขียนทับ `docs/images/layout.png`
      - สคริปต์ของภาพไทย (`thai`, `dark-th`, `font-th`) สร้างข้อความไทยชุดเดียวกับภาพเดิม (ถอดจากภาพ เพราะสคริปต์เดิมไม่ได้เก็บไว้)
      - ตรวจสอบ: รันทุกสคริปต์ด้วย build ล่าสุดแล้วเทียบพิกเซลกับภาพก่อนหน้า ตามข้อข้างบน; รัน `layout.jsonl` ซ้ำได้ไฟล์ตรงกันทุกไบต์
  - `README.th.md` ใช้คำตามอภิธานศัพท์และ `th.tsv`: เอเจนต์, ช่องทางควบคุม, ส่วนติดต่อผู้ใช้, ระยะห่างบรรทัด 1 เท่า, กล่องโต้ตอบฟอนต์; ตัวติดตั้งเรียกเฉพาะ MSI; ระยะบรรทัดให้จำนวนบรรทัดต่อหน้าเท่ากับ Word (ไม่สัญญาว่าจำนวนหน้าเท่ากัน เพราะการตัดคำไทยยังต่างได้)
    - ตรวจสอบ: agent ตรวจคำแปลเทียบ `README.md` พบ 14 ข้อ อีก agent หักล้างได้ 2 ข้อ (เป็นเรื่องรูปแบบ) แก้ 12 ข้อที่เหลือ
  - ผู้ใช้: portable zip ของ Windows มี `README.md` ด้วย เพราะ `packaging/windows/package.ps1` คัดลอกไฟล์นี้เมื่อมีอยู่ ไม่กระทบไฟล์ .docx
  - ตรวจสอบ: เปิดดูภาพทั้งหกหลังถ่าย; สคริปต์ตรวจลิงก์ยืนยันว่าลิงก์และภาพในทั้งสองหน้าชี้ไฟล์ที่มีอยู่ครบ; `cargo xtask assets` ผ่าน (56 ไฟล์มีที่มาใน `ATTRIBUTION.md`); ทุกข้อความเทียบกับโค้ดแล้ว เช่น รูปแบบไฟล์ตาม `crates/formats/src/lib.rs` และลิเกเจอร์ตาม `crates/layout/src/para.rs` `run_features`
- **แยกคำอังกฤษที่ใช้ร่วมกันสองที่ให้แปลไทยต่างกัน** (`crates/ui-egui/src/i18n/catalog.rs` `CONTEXTS`, `th.tsv`, `widgets.rs` `small`, `ribbon.rs`, `dialogs_refs.rs`, `options.rs`, `docs/localization-th.md`)
  - เหตุผล: Editor, Display, Open, Arabic มีสองความหมายแต่แปลได้คำเดียว (เช่น Editor ในช่องบรรณานุกรมอ่านว่า "ตัวช่วยตรวจแก้")
  - ผู้ใช้: ภาษาไทยเห็น บรรณาธิการ (ช่องบรรณานุกรม), แยกบรรทัด (ปุ่มสมการ), กว้าง (ระยะห่างย่อหน้า), อารบิก (ตัวเลือกตัวเลข); ภาษาอื่นไม่เปลี่ยน เพราะ context ที่ไม่มีรายการจะใช้คำแปลปกติ; ไม่กระทบเอกสาร
  - Letter ไม่ต้องแยก เพราะ UI ไม่แสดงชื่อกระดาษผ่านคีย์นี้
  - ตรวจสอบ: เทสต์ `thai_splits_the_english_words_two_places_share`; `cargo test -p doc4me-ui-egui` 314 ผ่านก่อนเพิ่มเทสต์ใหม่ และเทสต์ภาษาไทย 6 ผ่านหลังเพิ่ม; clippy `-D warnings` ผ่านทั้ง workspace
- **อ่านและเขียน "Kerning for fonts" ใน RTF, ODT และอ่านจาก .doc** (`crates/formats/src/model.rs` `Fmt::kern`, `rtf.rs` `\kerningN`, `odt.rs` `style:letter-kerning`, `crates/docbin/src/fmt.rs` sprmCHpsKern 0x484B)
  - เหตุผล: เดิมเปิดไฟล์เหล่านี้แล้วค่า kerning หายไป ตัวอักษรชิดต่างจาก Word
  - ผู้ใช้: RTF เก็บขนาดขั้นต่ำเป็นครึ่งพอยต์ (`\kerning0` = ปิด); ODF มีแค่เปิด/ปิด จึงอ่านว่าเปิด = ตั้งแต่ 1 pt และเขียน `true`/`false` ตามค่า; .doc อ่านเท่านั้น (ยังไม่มีตัวเขียน .doc) ข้อความที่ไม่ได้ตั้งค่าไม่เปลี่ยน ไฟล์ .docx เดิมไม่ได้รับผลกระทบ
  - ข้อจำกัด: markdown/html/latex ไม่มีค่านี้
  - ตรวจสอบ: เทสต์ `kerning_round_trips_through_rtf_and_odt` (formats) และ `kerning_for_fonts_is_read` (docbin); `cargo test -p doc4me-formats -p doc4me-docbin` 65 และ 69 ผ่าน
- **ข้อความในแผนภูมิ SmartArt และสมการ (m:nor) ที่ใช้ TH Sarabun ขนาดเท่าต้นฉบับ** (`crates/layout/src/display.rs` `text_glyphs`, `crates/layout/src/math.rs` `run_atoms`)
  - เหตุผล: เดิมวาดด้วย Sarabun ขนาดเต็ม ทำให้ตัวอักษรใหญ่กว่า Word ที่ใช้ TH Sarabun จริง
  - ผู้ใช้: แยกช่วงอักษรไทยกับละติน ย่อแต่ละช่วงด้วยสเกลของต้นฉบับ และในแผนภูมิวางแต่ละตัวบนความกว้างของ TH Sarabun เดิม ฟอนต์อื่นไม่เปลี่ยน ไฟล์ .docx ไม่เปลี่ยน (เป็นการวาดเท่านั้น)
  - `text_glyphs` คืน `Vec<Draw>` แทน `Option<Draw>` (หนึ่งรายการต่อช่วง)
  - ตรวจสอบ: เทสต์ `drawing_text_in_th_sarabun_is_sized_like_the_original`; `cargo test -p doc4me-layout` 222 ผ่าน; clippy `-D warnings` และ fmt ผ่าน เฉพาะ crate layout ส่วนสมการ m:nor ยังไม่มีเทสต์เฉพาะ
  - ต่อมาเพิ่มเทสต์ `normal_text_in_th_sarabun_is_sized_like_the_original` (`crates/layout/src/math.rs`) ยืนยันว่า m:nor ใน TH Sarabun New แยกเป็นช่วงละตินกับไทยที่ขนาด 20 × `scale(false)` และ 20 × `scale(true)` ส่วน Sarabun คงขนาด 20 pt; `cargo test -p doc4me-layout normal_text_in_th_sarabun` ผ่าน; `cargo test -p doc4me-layout` 223 ผ่าน และ clippy `-D warnings` ผ่าน
- **กล่อง Font: ช่อง "Kerning for fonts" และ "Ligatures"** (`crates/ui-egui/src/dialogs.rs`, `crates/engine/src/cmd/format.rs`, `th.tsv`)
  - เหตุผล: เดิมตั้ง `w:kern` และ `w14:ligatures` ได้เฉพาะผ่านคำสั่ง `format.set`
  - ผู้ใช้: ติ๊กเปิด kerning แล้วระบุขนาดขั้นต่ำ และเลือก ligature ได้ 5 แบบ ส่งเฉพาะค่าที่ผู้ใช้เปลี่ยน จึงไม่กระทบไฟล์ .docx เดิม
  - state ของ ribbon เพิ่มคีย์ `kern` และ `ligatures`; เพิ่มข้อความไทย 6 รายการ
  - ต่อมาแยกเป็นสองหน้าแบบ Word: Font (ฟอนต์ ขนาด สี เอฟเฟกต์) และ Advanced (ระยะห่างอักขระ Kerning Ligatures) ด้วยตัวเลือก `tab` ในกล่อง
  - ตรวจสอบ: เทสต์ `font_dialog_sets_kerning_and_ligatures_only_when_changed` ผ่าน; `cargo test -p doc4me-ui-egui` 315 ผ่าน; clippy `-D warnings` ผ่าน
- **TH Sarabun New / TH SarabunPSK / TH SarabunIT๙ ขนาดเท่าต้นฉบับเมื่อไม่ได้ติดตั้ง** (`crates/fonts/src/thsarabun.rs`, `word::MetricFit`)
  - เมื่อเครื่องไม่มีฟอนต์เหล่านี้ จะวาดด้วย Sarabun ที่ฝังในโปรแกรม โดยย่อตัวอักษรให้สูงเท่าต้นฉบับ
    - ตัวไทยใช้ความสูงของ ก: 400/586 (ตัวหนา 407/586)
    - ตัวละตินใช้ความสูงของ x: 340/500
  - แต่ละตัวอักษรใช้ความกว้างของ TH Sarabun New และวาง glyph ไว้กึ่งกลางช่อง
  - ใช้คู่ kerning ของต้นฉบับเมื่อเอกสารเปิด kerning
  - ระยะบรรทัดเท่ากับต้นฉบับ (1.331 em)
  - ผลคือบรรทัดและหน้าตัดตรงตำแหน่งเดียวกับเมื่อมีฟอนต์จริง ส่วนเมื่อติดตั้งฟอนต์จริงไว้ จะใช้ฟอนต์จริงเสมอ
  - ค่าทั้งหมดวัดจากไฟล์ TH Sarabun New 1.3 (2011) และ TH SarabunPSK ทั้ง 4 แบบ (ปกติ หนา เอียง หนาเอียง)
    - ความกว้างและ kerning ของสองฟอนต์นี้เท่ากันทุกตัว
    - ไม่ได้นำไฟล์ฟอนต์มาใส่ใน repo ใช้เฉพาะตัวเลขที่วัดได้
  - TH SarabunIT๙ แสดงเลขอารบิกเป็นเลขไทย โดยใช้ความกว้างของเลขไทย ส่วนนี้อนุมานจากการออกแบบของฟอนต์ ยังไม่ได้วัดจากไฟล์ IT๙ จริง
- **เลขลำดับแบบคำอ่านไทย (`thaiCounting`)** (`crates/doc/src/section.rs` `thai_counting`)
  - ตัวอย่าง: หนึ่ง สอง … สิบ สิบเอ็ด ยี่สิบ ยี่สิบเอ็ด … หนึ่งร้อย หนึ่งร้อยเอ็ด หนึ่งพันเอ็ด หนึ่งล้าน หนึ่งล้านเอ็ด สิบเอ็ดล้าน
  - อ่านตามหลักของราชบัณฑิตยสภา
    - เลข 1 ท้ายจำนวนที่มีหลักอื่นนำหน้าอ่าน "เอ็ด"
    - หลักสิบเป็น สิบ/ยี่สิบ (ไม่ใช่ หนึ่งสิบ/สองสิบ)
    - 0 อ่าน "ศูนย์"
  - ทดสอบครบช่วง u32
  - เพิ่มชนิดรายการ `thaiNumbers`, `thaiLetters`, `thaiCounting` ในคำสั่ง `para.numbering`, เมนู Numbering บนริบบอน และกล่อง Define New Multilevel List
- **ตัวตรวจตัวสะกดภาษาไทย** (`crates/proof/src/th`, `ProofLang::Th`)
  - เน้นความแม่นยำ: ทำเครื่องหมายเฉพาะข้อความที่ผิดแน่นอน ไม่ทำเครื่องหมายคำที่ไม่รู้จัก เช่น ชื่อเฉพาะหรือคำทับศัพท์ใหม่
  - ตรวจลำดับการพิมพ์ที่ไม่มีในคำไทยใด พร้อมคำแก้
    - เ สองตัวแทน แ
    - ํ กับ า แทน ำ
    - วรรณยุกต์หรือ ์ ก่อนสระบนหรือสระล่าง
    - วรรณยุกต์หลัง ำ
    - เครื่องหมายซ้ำ
    - วรรณยุกต์สองตัว
  - ตรวจคำที่มักเขียนผิด 105 คู่ โดยแนะนำตัวสะกดตามราชบัณฑิตยสภา (`assets/spelling/th-misspellings.tsv`)
    - ตัวอย่าง: อนุญาติ→อนุญาต, กฏหมาย→กฎหมาย, ปฎิบัติ→ปฏิบัติ, อินเตอร์เน็ต→อินเทอร์เน็ต, ฟังก์ชั่น→ฟังก์ชัน, กระทันหัน→กะทันหัน, มาตราฐาน→มาตรฐาน
    - ใส่เฉพาะคู่ที่รูปผิดไม่เป็นคำหรือชื่อที่ถูกต้องในบริบทใด
    - ตัดคำที่มักใช้ในชื่อเฉพาะออก เช่น เกมส์ (เอเชียนเกมส์) และ รีสอร์ท (ชื่อกิจการ)
    - เทสต์ `every_listed_correction_is_left_alone_in_a_sentence` ยืนยันว่าตัวสะกดที่ถูกทุกคำในรายการไม่ถูกทำเครื่องหมาย
    - ทำเครื่องหมายเฉพาะเมื่อตัวแบ่งคำตามพจนานุกรม (ICU4X) เห็นว่าเป็นคำทั้งคำ
    - ตัวอักษรชุดเดียวกันที่อยู่ในคำอื่นหรือคร่อมสองคำ (เช่น "กระเพราะ") จึงไม่ถูกทำเครื่องหมาย
  - ตรวจข้อความภาษาไทยทุกที่ ไม่ขึ้นกับภาษาที่กำหนดให้ run
  - ข้อความแจ้งเป็นภาษาไทย และเคารพพจนานุกรมผู้ใช้
  - เพิ่ม "Thai" ในเมนูภาษาตรวจคำและกล่อง Language

- **ส่วนติดต่อผู้ใช้ภาษาไทย** (`crates/ui-egui/src/i18n/th.tsv`, ภาษา `th` ใน `i18n/mod.rs`)
  - แค็ตตาล็อกครบ 2,433 รายการ เท่ากับทุกคีย์ของแค็ตตาล็อกภาษาอื่น ครอบคลุม
    - ริบบอน กล่องโต้ตอบ และข้อความแจ้ง
    - ชื่อสไตล์ในตัว และแกลเลอรีสมการ
  - เลือกได้ใน File › Options และเลือกเองอัตโนมัติเมื่อระบบตั้งภาษา `th`, `th-TH` หรือ `th_TH.UTF-8`
  - เขียนตามอภิธานศัพท์ 903 คำและคู่มือรูปแบบ 31 ข้อ เช่น
    - คำสั่งขึ้นต้นด้วยกริยา ป้ายชื่อเป็นคำนาม
    - Font = ฟอนต์ และแท็บ Home = หน้าหลัก (เพราะ หน้าแรก หมายถึง first page)
    - hyphenation = การใส่ยัติภังค์ (ไม่ใช่ การตัดคำ)
    - คำทับศัพท์สะกดตามราชบัณฑิตยสภา
    - เว้นวรรคหนึ่งครั้งระหว่างไทยกับอักษรละติน ตัวเลข และ placeholder
    - ตัวเลขในช่องกรอกเป็นเลขอารบิก
  - clean room: เขียนจากความหมายของข้อความภาษาอังกฤษ ไม่ได้ใช้หรือคัดลอกคำแปลของ Microsoft Office
  - เอกสารใหม่ยังใช้ภาษาแก้ไขเป็น US English เพราะตัวตรวจไทยตรวจข้อความไทยไม่ว่าจะตั้งภาษาอะไร และคำอังกฤษที่พิมพ์ปนยังถูกตรวจเป็นภาษาอังกฤษ
  - เทสต์ (`crates/ui-egui/src/i18n/tests.rs`)
    - `thai_locales_and_saved_preference_work_without_changing_the_document`
    - `thai_covers_the_entire_existing_interface_catalog`
    - `thai_catalog_is_typed_cleanly`: ไม่มีเลขไทย ไม่มี "..." ไม่มีอักษรอื่น และทุกข้อความผ่านตัวตรวจไทยของโปรแกรมเอง
    - `bundled_interface_fonts_cover_thai_without_system_fallbacks`
  - การตรวจกลไก interface audit ครอบคลุมภาษาไทยด้วย (`audit.rs`)
  - ยังไม่ได้ให้นักแปลมืออาชีพที่เป็นเจ้าของภาษาตรวจทาน
  - คีย์ที่ใช้ร่วมกันหลายที่ เช่น Editor ใช้ทั้งเป็นปุ่มตรวจแก้และเป็นฟิลด์บรรณาธิการ ยังแปลได้แบบเดียว ต้องแยกคีย์ในโค้ดภายหลัง
- **ชื่อภาษา "Thai" ในแค็ตตาล็อกส่วนติดต่อผู้ใช้ทั้ง 17 ภาษา** (`crates/ui-egui/src/i18n/*.tsv`)
  - เมนูภาษาตรวจคำ (Review › Language) และกล่อง Language มีตัวเลือก Thai เพิ่มขึ้น
  - แต่ก่อนชื่อนี้ไม่มีในแค็ตตาล็อกใดเลย จึงแสดงเป็นภาษาอังกฤษในทุกภาษา ตอนนี้แต่ละภาษาแสดงชื่อในภาษาของตัวเอง เช่น Thailändisch, Thaï, タイ語, 泰语
  - วางไว้คู่กับรายการ "Thai Distributed" ในแต่ละไฟล์

### แก้ไข

- **ตัวอักษรใหญ่ขึ้นต้นย่อหน้า (drop cap) เล็กเกินไปหลังเปลี่ยนระยะบรรทัดแบบ Word** (`crates/layout/src/para.rs` `layout_para`)
  - อาการ: drop cap 3 บรรทัดสูงแค่ราว 2 บรรทัด ยอดตัวอักษรต่ำกว่าบรรทัดแรก และบรรทัดแรกเหลือช่องว่างข้างหน้า (เห็นในภาพ `layout.png` ที่ถ่ายใหม่)
  - สาเหตุ: ขนาดของ drop cap คำนวณจากระยะบรรทัดตายตัว 1.2 em ขณะที่บรรทัดจริงใช้ความสูงบรรทัดแบบ Word ของฟอนต์ตั้งแต่รุ่นนี้ (Sarabun 1.853 em) และไม่ดูกฎระยะห่างบรรทัดของย่อหน้าเลย
  - แก้: ใช้ความสูงบรรทัดของฟอนต์ตัวแรก (ascent + descent ของ style เดียวกับที่ `break_lines` ใช้) ตามกฎระยะห่าง Multiple / AtLeast / Exactly และ snap กับเส้นตารางเอกสารเมื่อย่อหน้า snap
  - ผู้ใช้: drop cap ในไฟล์ .docx เดิมคลุมจากยอดบรรทัดแรกถึงเส้นฐานบรรทัดที่ N เหมือน Word ทั้งระยะบรรทัดหนึ่งเท่า หลายเท่า และแบบกำหนดค่า
  - ตรวจสอบ: เทสต์ใหม่ `drop_cap_spans_its_lines_at_their_pitch` (Sarabun ระยะ 1.0, 1.5 เท่า, AtLeast 40 pt และ Exactly 30 pt) ก่อนแก้ล้มด้วย `cap 62.000004 pt, want 88.12 pt (pitch 25.942001)` หลังแก้ผ่าน; `cargo test -p doc4me-layout` ผ่าน 224 รายการ และ `cargo test -p doc4me-engine` ผ่าน 372 + 13 + 8 รายการ
- **ภาพตัวอย่าง style set บนแท็บ Design ตัดบรรทัดเนื้อความทิ้ง** (`crates/ui-egui/src/previews.rs` `style_set_gallery`)
  - อาการ: ทุกภาพเห็นแค่ Title กับ Heading 1 ที่ชิดขอบล่าง บรรทัด "Body text…" หาย และเส้นใต้ Heading 1 ของชุด lines หาย
  - สาเหตุ: ภาพวาดพื้นที่สูงตายตัว 110 pt แต่ด้วยความสูงบรรทัดแบบ Word บรรทัดแรกของเนื้อความจบที่ 152.07 pt ในทุกชุด
  - แก้: วาดพื้นที่สูง 150 pt (`STYLE_SET_PAGE`) ที่สเกล 0.38 แทน 0.42 (`STYLE_SET_SCALE`) ภาพจึงสูง 59.3 pt พอดีช่อง 60 pt; แยก `snippet_doc` และ `style_set_sample` ออกมาให้เทสต์จัดหน้าเอกสารเดียวกับที่วาดได้
  - ตรวจสอบ: เทสต์ใหม่ `style_set_thumbnails_show_all_three_paragraphs` จัดหน้าภาพตัวอย่างทั้ง 7 ชุดแล้วตรวจว่าบรรทัดแรกของเนื้อความอยู่ในพื้นที่ที่วาด ก่อนแก้ล้มด้วยค่า 152.0744 pt ทุกชุด หลังแก้ผ่าน; `cargo test -p doc4me-ui-egui` ผ่าน 320 รายการ, clippy `-D warnings` ของ layout และ ui-egui และ fmt ผ่าน
- **ช่องตัวเลขที่มีช่วงค่า เปลี่ยนเอกสารเองเมื่อแค่แสดง** (เดิมบันทึกไว้เป็นข้อจำกัดที่ทราบก่อนออกรุ่นนี้)
  - อาการ: ถ้าค่าในเอกสารอยู่นอกช่วงของช่อง แค่วาดช่องนั้นค่าก็ถูกบีบเข้าช่วง เช่น เลือกภาพกว้าง 3000 pt แล้วแถบคำสั่ง Picture Format ย่อภาพเหลือ 2000 pt ทันที (ช่อง W/H ช่วง 4–2000 pt แต่ engine รับ 1–4000 pt) ช่องในกล่องโต้ตอบก็บีบค่าเช่นกันแล้วเขียนลงเอกสารเมื่อกดตกลงแม้ไม่ได้แตะช่องนั้น
    - ตัวอย่างช่องตัดภาพบนแถบคำสั่ง (0–45%) เกิดได้จริงกับไฟล์ .docx เพราะตัวอ่าน `a:srcRect` (`crates/docx/src/read/story.rs`) รับการตัดได้ถึง 100% ส่วนคำสั่ง `picture.crop` บีบไว้ที่ 45%
  - สาเหตุ: `egui::DragValue` ที่มี `range` บีบค่าที่มีอยู่ทุกเฟรมเป็นค่าเริ่มต้น และนับเป็นการเปลี่ยนค่า (`response.changed()` เป็นจริง) ตรวจจากซอร์ส egui 0.36.2 `widgets/drag_value.rs`
  - แก้: ใส่ `.clamp_existing_to_range(false)` ให้ทุกช่อง `DragValue` ที่มี `range` ใน `crates/ui-egui/src` ครบ 47 ช่อง ในไฟล์ `ribbon.rs` (11), `dialogs.rs` (14), `dialogs_layout.rs` (7), `dialogs_refs.rs` (3), `dialogs_insert.rs` (3), `dialogs_para.rs` (2), `dialogs_lists.rs` (2), `colors.rs` (2), `dialogs_notes.rs`, `options.rs`, `print_page.rs` ช่วงค่ายังใช้กับค่าที่ผู้ใช้ลากหรือพิมพ์เหมือนเดิม
    - ช่องที่ค่าไม่ได้มาจากเอกสาร (จำนวนสำเนาพิมพ์, แถว/คอลัมน์ของป้าย, ขนาดตารางใหม่, จำนวนเอกสารล่าสุด, สี RGB 0–255) ใส่ด้วยเพื่อให้เป็นกฎเดียวทั้ง crate ผลไม่ต่างจากเดิม เพราะโค้ดเดิมบีบค่าเองหลังวาด หรือค่าอยู่ในช่วงเสมอ
    - กล่อง Columns: เดิมความกว้างคอลัมน์ที่เกินความกว้างข้อความถูกบีบและสั่ง `rebalance` ทุกเฟรม ตอนนี้ปรับเฉพาะเมื่อผู้ใช้แก้ช่อง
  - ผลต่อผู้ใช้: เปิดไฟล์ .docx เดิมที่มีค่าเกินช่วงของช่อง (ภาพใหญ่, ระยะย่อหน้าหรือระยะห่างมาก, ช่องคอลัมน์มากกว่า 12) แล้วเลือกหรือเปิดกล่องโต้ตอบ ค่าเดิมจะคงอยู่จนกว่าผู้ใช้จะแก้เอง ไม่กระทบการติดตั้งทับหรือภาษาไทย
  - ตรวจสอบ: เทสต์ใหม่ `ribbon::tests::showing_picture_size_fields_changes_nothing` วาดกลุ่ม Size ของ Picture Format สองเฟรมกับภาพ 3000×2500 pt แล้วตรวจว่าขนาดไม่เปลี่ยน และ `widgets::tests::ranged_number_fields_keep_existing_values` ไล่ซอร์สทุกไฟล์ใน `crates/ui-egui/src` ว่า `DragValue` ที่มี `.range(` ต้องมี `.clamp_existing_to_range(false)` เพื่อกันช่องใหม่ที่ลืมใส่
    - ก่อนแก้ (เอา `.clamp_existing_to_range(false)` ของช่อง 4–2000 pt ออกชั่วคราว) เทสต์แรกล้มด้วยขนาด `(2000.0, 1666.6666)` แทน `(3000.0, 2500.0)` และเทสต์ตรวจซอร์สชี้ 3 ช่องใน `ribbon.rs` หลังแก้ผ่าน
    - `cargo test -p doc4me-ui-egui` ผ่าน 319 รายการ (ข้าม 1 คือ audit ภาษา `#[ignore]`), `cargo clippy -p doc4me-ui-egui --all-targets -- -D warnings` และ `cargo fmt --all -- --check` ผ่าน
- **ธีมมืด: แถบของตัวเลื่อนซูมที่มุมขวาล่างมองไม่เห็น เห็นแค่ปุ่มเลื่อน** (`crates/ui-egui/src/chrome.rs` `status_bar`)
  - สาเหตุ: egui วาดรางของ `Slider` ด้วย `widgets.inactive.bg_fill` ซึ่งธีมตั้งเป็นสีช่องป้อนข้อมูล (`input` 0x1E1E1E) แทบเท่าสีแถบสถานะ (`status_bar` 0x1F1F1F) ตรวจจากซอร์ส egui 0.36.2 `widgets/slider.rs`
  - แก้: วาดตัวเลื่อนซูมใน `ui.scope` ที่ตั้งสีรางเป็น `input_border` (มืด 0x7A7A7A, สว่าง 0x8A8A8A) ปุ่มเลื่อนตอนไม่ได้ชี้ก็ใช้สีนี้ด้วย (egui ใช้ค่าเดียวกันทั้งสองส่วน) จึงทำรางบางเหลือ 3 pt (`slider_rail_height`) แบบเส้นของ Word ให้ปุ่มเด่นกว่าราง ช่องและตัวเลื่อนอื่นไม่เปลี่ยน
  - ผลต่อผู้ใช้: ธีมสว่างรางเปลี่ยนจากแถบสีขาวหนา 8 pt บนพื้น 0xF0F0F0 เป็นเส้นสีเทา 3 pt ซึ่งใกล้ Word กว่า ไม่กระทบไฟล์ .docx
  - ตรวจสอบ: เทสต์ใหม่ `chrome::tests::zoom_slider_rail_shows_in_the_dark_theme` วาดแถบสถานะในธีมมืดแล้วหาสี่เหลี่ยมกว้างราว 110 pt สูงไม่เกิน 4 pt สี `input_border`
    - ก่อนแก้ล้มด้วย `no visible zoom rail: []` หลังแก้ผ่าน คอมมิตแรก `f2d21b5` มีเทสต์นี้ที่ panic ตอนทิ้ง `FullOutput` โดยไม่เรียก `drop_without_applying_deltas` แก้ในคอมมิตถัดมา
- **เปิดหน้าขั้นสูงของกล่องฟอนต์แล้วเปิดเคิร์นนิงเองและตัดระยะห่างตัวอักษร** (`crates/ui-egui/src/dialogs.rs`)
  - อาการ: แค่เปิดหน้าขั้นสูง ช่อง "เคิร์นนิงสำหรับฟอนต์" ก็ถูกติ๊กที่ 0.5 พอยต์ขึ้นไป และระยะห่างที่เกิน ±20 pt ถูกตัดเหลือ 20 กดตกลงแล้วค่าเหล่านี้ถูกเขียนลงข้อความที่ไม่เคยตั้งไว้
  - สาเหตุ: `egui::DragValue` ที่มี `range` จะบีบค่าที่มีอยู่ให้อยู่ในช่วงทุกครั้งที่วาด (ค่าเริ่มต้นของ egui) ค่า 0 ที่แปลว่าปิดเคิร์นนิงจึงกลายเป็น 0.5
  - แก้: ใส่ `.clamp_existing_to_range(false)` ให้ทั้งสองช่อง ช่วงจึงใช้กับค่าที่ผู้ใช้ป้อนเท่านั้น
  - พบจากการเปิดดูภาพหน้าจอของกล่องฟอนต์ระหว่างทำ README เทสต์เดิม `font_dialog_sets_kerning_and_ligatures_only_when_changed` เรียก `apply_font` โดยไม่ได้วาดหน้า จึงไม่เจอ
  - ตรวจสอบ: เทสต์ใหม่ `showing_the_font_advanced_page_changes_nothing` วาดหน้าขั้นสูงสองเฟรม ก่อนแก้ล้มด้วย `(0.5, 20.0)` แทน `(0.0, 25.0)` หลังแก้ผ่าน; เทสต์กล่องฟอนต์อื่นผ่านทั้งหมด
- **แค็ตตาล็อก ru/hu/tr/th/it ขาดข้อความของโหมดอ่านและข้อความสถานะ** (`crates/ui-egui/src/i18n/{ru,hu,tr,th,it}.tsv`, `audit.rs`, `pseudo.rs`, `lib.rs` `after_command`)
  - เหตุผล: รันเทสต์ audit `untranslated_interface_text` ด้วย `--ignored` แล้วล้ม เพราะ ru/hu/tr ขาด 15 และ th ขาด 14 ข้อความ ได้แก่ป้ายในแถบโหมดอ่าน (Reading Mode, Column width, Text size, Edit Document), ข้อความสถานะ 2 ประโยค ("Researcher needs an online service…", "These settings will be used for new documents.") และขนาดกระดาษ "{w} × {h} in" จึงแสดงเป็นภาษาอังกฤษ
  - แก้: เพิ่มคำแปล ru/hu/tr 9 รายการ, th 7 รายการ (รวมข้อความสถานะ "Column selection: …" และ "{w} × {h} mm" ที่ audit ไม่ได้เปิดถึง) และ it 9 รายการ เพราะเทสต์กำหนดให้อิตาลีครอบคลุมทุกคีย์ของ tr; th.tsv มี 2,452 รายการ
  - ข้อความ error ของคำสั่ง (ขึ้นต้นด้วยตัวพิมพ์เล็ก เช่น "bad parameters: …", "can't split at the first row") ยังเป็นภาษาอังกฤษตามนโยบายเดิม เพราะคำสั่งตอบสคริปต์และ MCP ด้วย: เพิ่ม `pseudo::note_status`/`statuses` (เฉพาะ `cfg(test)`) จดข้อความที่ส่งไปแถบสถานะ ให้ audit ยกเว้นข้อความเหล่านี้ โปรแกรมจริงไม่เปลี่ยน
  - ผู้ใช้: UI ภาษาไทย รัสเซีย ฮังการี ตุรกี และอิตาลีแสดงข้อความเหล่านี้เป็นภาษานั้น ไม่กระทบไฟล์ .docx และค่าที่บันทึกไว้
  - ตรวจสอบ: `cargo test -p doc4me-ui-egui i18n -- --include-ignored` 60 ผ่าน (รวม `untranslated_interface_text`, `thai_covers_the_entire_existing_interface_catalog`, `thai_catalog_is_typed_cleanly`); หลัง push รัน `cargo test -p doc4me-ui-egui` 315 ผ่าน (1 ignored คือ audit) และ clippy `-D warnings` กับ `cargo fmt --all --check` ผ่าน
- ปรับเทสต์ที่ผูกกับระยะบรรทัด 1.30 em ของ Sarabun ให้ใช้ค่าใหม่ ได้แก่
  - layout: แถวตารางข้อความหมุน, Table Grid, ตารางลอยหลังหน้าเต็ม, เชิงอรรถยาว
  - render: ข้อความหมุนในเซลล์
- **เทสต์อักษรพม่าข้ามไปเมื่อเครื่องไม่มีฟอนต์ที่จัดรูปอักษรพม่าได้** (`crates/layout/src/tests_complex.rs` `myanmar_face`)
  - ปัญหา: เทสต์เลือกฟอนต์ระบบตัวแรกที่มีตัวอักษรพม่าและมีตาราง GSUB บนเครื่องที่มีแค่ Unifont ฟอนต์นี้มีตาราง GSUB สำหรับอักษรอื่น แต่ไม่มีกฎของอักษรพม่า สระ ေ จึงไม่ถูกย้ายไปไว้หน้าพยัญชนะ และเทสต์ `myanmar_prevowel_is_drawn_before_its_consonant` ล้มเหลว
  - ปัญหานี้เกิดตั้งแต่ก่อนการแก้ไขครั้งนี้ และเกิดจากชุดฟอนต์ของเครื่อง ไม่เกี่ยวกับโค้ดของโปรแกรม
  - แก้โดยให้เทสต์ใช้เฉพาะฟอนต์ที่มีสคริปต์ `mym2` หรือ `mymr` ใน GSUB ซึ่งอ่านด้วย `skrifa` ที่เพิ่มเป็น dev-dependency ของ `doc4me-layout`
  - ไม่กระทบโค้ดที่ใช้งานจริง

- **แก้ข้อบกพร่องที่ผู้ตรวจพบก่อน commit** (ตรวจแบบ adversarial 4 ด้าน แต่ละข้อมีผู้ตรวจอีกคนยืนยัน)
  - **ลิเกเจอร์บังคับของภาษาอาหรับแตก** (`crates/layout/src/para.rs` `run_features`)
    - ปัญหา: การปิด `liga`/`clig` ตามค่าเริ่มต้นของ Word ไปมีผลกับอักษรซับซ้อนด้วย ฟอนต์อาหรับบางตัว (เช่น DejaVu Sans) เก็บรูป لأ لإ لآ ไว้ใน `liga` คำที่ใช้บ่อยอย่าง لأن และ الآن จึงแสดงผิด
    - แก้: ส่วนที่เป็นอักษรซับซ้อน (อาหรับ ฮีบรู ไทย อินดิก) ใช้ลิเกเจอร์ของฟอนต์ตามเดิม kerning ยังเป็นไปตาม `w:kern`
    - ผลข้างเคียง: ลิเกเจอร์ ฤๅ ของ Sarabun ใช้ได้เหมือนเดิม
  - **ค่า `w14:ligatures` ที่ .docx เก็บไม่ได้ ไม่เปิดลิเกเจอร์แล้ว** (`crates/doc/src/resolve.rs`)
    - ปัญหา: ถ้าตั้งค่าผ่าน `format.set` เป็นค่าที่ไม่ถูกต้อง เช่น "Standard" ลิเกเจอร์จะเปิด แต่ค่านี้หายไปเมื่อบันทึก เปิดไฟล์ใหม่จึงจัดบรรทัดต่างไป
    - แก้: ค่าที่ไม่อยู่ใน `LIGATURE_VALUES` ไม่เปิดอะไรเลย ตรงกับสิ่งที่ตัวบันทึกเก็บ
  - **ฟอนต์ที่ถูกแทนที่ใช้ระยะบรรทัดของฟอนต์ที่เอกสารขอ** (`crates/fonts/src/word.rs` `single_line`)
    - ปัญหา: การเปลี่ยนไปใช้ค่า win ทำให้ฟอนต์แทนที่บางตัวได้บรรทัดสูงกว่าฟอนต์จริง
      - Angsana New, Cordia New, Browallia New และ TH Niramit AS ที่วาดด้วย Sarabun ได้ 1.853 em แทนราว 1.3 em
      - Cambria ที่วาดด้วย Caladea ได้ 1.30 em แทน 1.172 em
      - Courier New ที่วาดด้วย JetBrains Mono (บนเว็บ) ได้ 1.565 em แทน 1.133 em
    - แก้:
      - ฟอนต์ที่เอกสารขอและมีอยู่จริงยังใช้ค่า win ของ Word บน Windows
      - Cambria และ Courier New ที่ถูกแทนที่ใช้ระยะบรรทัดของฟอนต์จริงตามตาราง
      - ฟอนต์แทนที่อื่นใช้ค่า hhea ของตัวแทน ซึ่งเท่ากับต้นฉบับสำหรับฟอนต์ที่ metric-compatible ส่วนฟอนต์ไทยที่วาดด้วย Sarabun ได้ 1.30 em เหมือนก่อนการแก้ไขนี้
  - **ฟอนต์ที่ค่า usWin เป็นศูนย์ใช้ค่า hhea ตามที่เอกสารของฟังก์ชันระบุ** (`crates/fonts/src/word.rs` `pick_line`)
    - ปัญหา: ฟอนต์แบบนี้ได้ descent 0 หางตัวอักษร (g p y) จึงล้นไปบรรทัดถัดไป
  - **เลือกเลขไทยบนรายการเลขอารบิกแล้วรายการถูกลบ** (`crates/engine/src/cmd/para.rs` `list`)
    - ปัญหา: การเลือก "๑. ๒. ๓." บนรายการ "1." ลบรายการทิ้ง และย่อหน้าหลังรายการเลขอารบิกไปต่อรายการเดิม เพราะการเทียบชนิดดูแค่ข้อความ "%1." ซึ่งเหมือนกัน
    - แก้: เทียบรูปแบบตัวเลขด้วย ตอนนี้รายการเปลี่ยนเป็นชนิดที่เลือกเหมือนใน Word
  - **ตัวตรวจไทยทำเครื่องหมายส่วนต้นของคำยาวผิด** (`crates/proof/src/th/mod.rs` `misspelled`)
    - ปัญหา: ICU ตัดคำที่ไม่รู้จักก่อนสระ เช่น คลินิค|ั|ล คำว่า "คลินิคัล" จึงถูกทำเครื่องหมาย และคำแนะนำจะเขียนเป็น "คลินิกัล"
    - แก้: คำผิดที่ตามด้วยสระหรือเครื่องหมาย หรืออยู่หลังสระหน้า (เ แ โ ใ ไ) ไม่ถูกทำเครื่องหมาย
  - **Ignore All และ Add to Dictionary ล้างเครื่องหมายลำดับการพิมพ์ผิดของไทยไม่ได้** (`th::check`)
    - แก้: คำในพจนานุกรมผู้ใช้ไม่ถูกทำเครื่องหมายแล้ว
  - **ข้อความที่ตั้งภาษาไทยแล้วพิมพ์ไทยติดกับอังกฤษถูกทำเครื่องหมายผิด** (`crates/proof/src/lib.rs` `words`)
    - ปัญหา: "ส่งemailหาครู" ถูกตรวจเป็นคำอังกฤษคำเดียวและถูกทำเครื่องหมาย
    - แก้: แยกคำตรงจุดที่อักษรไทยกับอักษรอื่นติดกัน คำอังกฤษตรวจแยก ส่วนคำไทยให้ `th::check` ตรวจ
  - เทสต์ที่เพิ่ม
    - `fonts::word::tests::substitutes_keep_the_requested_fonts_line` และส่วนที่เพิ่มใน `single_spacing_uses_word_for_windows_metrics`
    - `engine::tests::thai_numbering_replaces_decimal_numbering`
    - `proof::tests::thai_glued_to_latin_is_two_words`
    - `proof::th::tests::a_misspelling_inside_a_longer_word_is_not_marked`, `words_in_the_user_dictionary_get_no_typing_order_mark`
    - ส่วนที่เพิ่มใน `layout::tests_complex::ligatures_only_when_the_run_asks` (ค่าที่ไม่ถูกต้อง และ ฤๅ)

- **ป้ายขนาดซองในกล่อง Envelopes แปลได้** (`envelope_label` ใน `crates/ui-egui/src/dialogs_layout.rs`, `th.tsv`): หน่วยนิ้ว/มม. ผ่าน `tl!` และ `i18n::fmt` เดิมเขียนตายตัวเป็นอังกฤษ ทำให้เทสต์ audit `untranslated_interface_text` ล้ม ภาษาไทยแสดง "นิ้ว"/"มม." ไม่กระทบไฟล์ .docx หรือภาษาอื่น ยังรอผลเทสต์ i18n (commit ถัดไปจะบันทึกผล)

### การตรวจสอบ

- รันชุดตรวจเดียวกับ `cargo xtask ci` ทีละขั้นบน Linux ผ่านทั้งหมด
  - `cargo fmt --all -- --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test` ทีละแพ็กเกจครบทั้ง 20 แพ็กเกจ ผ่านทั้งหมด
    - ทำทีละแพ็กเกจเพราะดิสก์ของเครื่อง build มีจำกัด
    - ตัวอย่างจำนวนเทสต์: doc4me-ui-egui 313 (มี 1 เทสต์ที่ ignore อยู่แล้ว), doc4me-proof 73
  - เทสต์ audit ที่ปกติ ignore ไว้ (`untranslated_interface_text`) ล้มด้วยข้อความเดียวที่ไม่ได้ครอบ `tl!`
    - ข้อความนั้นคือขนาดซอง "#10 4.13 × 9.5 in" ในแท็บ Mailings
    - เป็นปัญหาเดิมก่อนรอบนี้ ไม่เกี่ยวกับภาษาไทย
  - `cargo xtask assets` (asset 53 ไฟล์มีรายการใน ATTRIBUTION.md ครบ)
  - `cargo xtask layers` (20 crate ไม่ละเมิดลำดับชั้น)
  - `cargo xtask wasm` (ทุก crate build เป็น wasm32 ได้)
- เทสต์ใหม่
  - ระยะบรรทัด: `fonts::word::tests::single_spacing_uses_word_for_windows_metrics`, `layout::tests_complex::sarabun_lines_are_spaced_like_word_for_windows`
  - TH Sarabun: `layout::tests_complex::th_sarabun_new_keeps_its_size_widths_and_line_spacing_when_drawn_with_sarabun`
  - kerning และ ligature: `letters_are_kerned_only_when_the_run_asks`, `ligatures_only_when_the_run_asks` และ roundtrip ของ `w14:ligatures` ใน `crates/docx/tests/roundtrip.rs`
  - รายการลำดับเลขไทย: `layout::tests::thai_list_labels` และ `doc::section::tests::number_formats` (คำอ่านไทย)
  - ตัวตรวจภาษาไทย: `crates/proof/src/th/tests.rs` 6 เทสต์ ครอบคลุมคำผิดทั่วไป การไม่ทำเครื่องหมายส่วนของคำอื่น การพิมพ์สลับลำดับ พจนานุกรมผู้ใช้ และข้อความแปลก ๆ ที่ต้องไม่ทำให้โปรแกรมล่ม

### ข้อจำกัดที่ทราบ

- glyph ของ Sarabun ที่ย่อแล้วกว้างหรือแคบกว่าต้นฉบับเล็กน้อยในบางตัว (เช่น ข กว้าง 0.405 em เทียบกับ 0.378 em) จึงอาจล้ำเข้าไปในช่องไฟข้างตัวได้ไม่เกินเศษเสี้ยวพอยต์ แต่ความกว้างบรรทัดเท่าต้นฉบับ
- ligature เสริมของ TH Sarabun (fi, ฤๅ) ไม่ได้จำลอง
- Word บน Windows อาจตัดคำไทยต่างจากพจนานุกรม ICU4X ในบางคำ จึงยังมีโอกาสที่บรรทัดตัดต่างกัน
- ตัวตรวจภาษาไทยยังไม่ตรวจไวยากรณ์

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

[Unreleased]: https://github.com/chatraweechai/doc4me/compare/v1.1.1...HEAD
[1.1.1]: https://github.com/chatraweechai/doc4me/compare/v1.1.0...v1.1.1
[1.1.0]: https://github.com/chatraweechai/doc4me/releases/tag/v1.1.0
[1.0.0]: https://github.com/chatraweechai/doc4me/releases/tag/v1.0.0
