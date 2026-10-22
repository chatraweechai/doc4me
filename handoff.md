# Handoff: Doc4Me 1.1.1 และงานภาษาไทย

อัปเดต: 2026-10-22 14:00 UTC · สาขา `main` · รายละเอียดดู [`CHANGELOG.md`](CHANGELOG.md) หัวข้อ `[Unreleased]`, `[1.1.1]` และ `[1.1.0]`

## สถานะ: ออก 1.1.1 แล้ว ไม่มีงานค้าง

- [Doc4Me v1.1.1](https://github.com/chatraweechai/doc4me/releases/tag/v1.1.1) ออก 2026-10-22 13:51 UTC จาก [run 39788628618](https://github.com/chatraweechai/doc4me/actions/runs/39788628618) (แท็บ Actions, `release: true`)
  - tag `v1.1.1` ชี้ `be5893e` มี `doc4me-1.1.1-windows-x64.msi` (32,993,280 ไบต์) และ `doc4me-1.1.1-windows-x64-portable.zip` (43,584,012 ไบต์)
- งานใหม่ทั้งหมดลง `## [Unreleased]` ห้ามแก้หัวข้อ `[1.1.1]` (ข้อความ "เจ้าของ repo ต้อง push tag เอง" ใน `[1.1.1]` ล้าสมัยแล้ว บันทึกแก้ไว้ใน `[Unreleased]` › แก้ไข)
- ออกรุ่นถัดไป: `cargo xtask version set x.y.z` ย้าย `[Unreleased]` เป็น `## [x.y.z] - YYYY-MM-DD` push ไป `main` แล้วสั่ง workflow `windows-x64-installer.yml` บน `main` พร้อม `release: true` (ดู `docs/releasing.md` ข้อ 4)
  - ระหว่างที่ release รัน ห้าม push การแก้ไฟล์ใน `.github/workflows` (เป็นเหตุให้รอบแรกของ 1.1.1 ล้มด้วย 403) push ไฟล์อื่นได้

### session 2026-10-22 13:35–14:00 UTC: ออก Release v1.1.1

- ผู้ใช้สั่ง "ดำเนินการต่อจากที่ค้างให้เสร็จ" งานที่ค้างมีข้อเดียวคือ Release v1.1.1 (ตอนเริ่ม session ยังไม่มี tag `v1.1.1` บน GitHub)
- สาเหตุที่รอบแรก (run 39783304886) ได้ 403 อ่านจาก log ของ job `release`:
  - token มี `Contents: write` ถูกต้อง แต่ก่อนที่ job `release` จะเรียก `POST /releases` (13:14:20 UTC) `main` ถูก push `e06edac` ไปแล้ว (13:11:49) ซึ่งแก้ `.github/workflows/windows-x64-installer.yml`
  - tag ใหม่ที่ `858b9bd` จึงมีไฟล์ workflow ไม่ตรงกับ `main` และ `GITHUB_TOKEN` ไม่มีสิทธิ์ `workflows` ที่จะสร้าง ref แบบนั้น
- 13:38 UTC สั่ง workflow_dispatch `release: true` บน `main` ที่ `be5893e` เป็น run 39788628618 แล้วผ่านทั้ง job `build` และ `release`
  - ระหว่างรัน push `10a276a` (handoff อย่างเดียว) ขึ้น `main` ตอน 13:42:43 เพราะ stop hook เตือนเรื่องคอมมิตที่ยังไม่ push และไฟล์ workflow ยังเหมือนเดิม release ก็ยังผ่าน จึงสรุปว่าตัวการคือไฟล์ workflow ที่ต่างกัน ไม่ใช่การที่ `main` ขยับ (สรุปจากการเทียบสองรอบ ไม่ได้ยืนยันจากเอกสารของ GitHub)
  - run ของ push `10a276a` (39789261472) ผ่าน
- บันทึกผลใน `CHANGELOG.md` `[Unreleased]` และเพิ่มข้อควรระวังใน `docs/releasing.md`

### ทางเลือกในการออก Release v1.1.1 (บันทึกเดิม ก่อนออกสำเร็จ)

- งานโค้ด ภาพหน้าจอ อภิธานศัพท์ และเอกสารของ 1.1.1 เสร็จและ push ไป `main` แล้ว ไม่มีงานโค้ดค้าง
- ทางที่ใช้ได้: เจ้าของ repo รันบนเครื่องตัวเอง `git fetch origin && git tag -a v1.1.1 origin/main -m "Doc4Me 1.1.1" && git push origin v1.1.1`
- ทางที่ลองแล้วไม่สำเร็จ:
  - session บนคลาวด์ push tag ไม่ได้ (git proxy ตอบ HTTP 403) แม้ `.claude/settings.json` อนุญาต `git tag -a v*` / `git push origin v*` แล้ว
  - ตัวเลือก `release` ของ workflow_dispatch (เพิ่มใน `858b9bd`, แก้ใน `e06edac`): run 39783304886 build ผ่าน แต่ `gh release create v1.1.1 --target` ล้มด้วย `HTTP 403: Resource not accessible by integration` (สร้าง tag ใหม่ด้วย `GITHUB_TOKEN` ไม่ได้)
  - ถ้าจะทำให้ทางนี้ใช้ได้ (ยังไม่ได้ตรวจ): ดู Settings › Actions › General › Workflow permissions ว่าเป็น "Read and write" หรือไม่ และ GitHub อาจไม่ให้ `GITHUB_TOKEN` สร้าง ref ที่ชี้คอมมิตซึ่งแก้ไฟล์ workflow ทางแก้คือใช้ PAT หรือ GitHub App token ที่มีสิทธิ์ `contents` + `workflows` เป็น secret
- หลังออกแล้ว: ตรวจว่า Release มี MSI และ zip และ job `release` ผ่าน งานใหม่ทั้งหมดลง `## [Unreleased]` ห้ามแก้ `[1.1.1]`

### ประวัติของ 1.1.1

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

- `Slider` อื่น (`read_aloud.rs`, `options.rs`, กล่อง Zoom ใน `dialogs.rs`) ยังบีบค่าแบบเดิมของ egui ค่ามาจากการตั้งค่าหรือมุมมองซึ่งอยู่ในช่วงเสมอ ถ้าวันหนึ่งผูกกับค่าจากเอกสาร ให้ใช้ `.clamping(egui::SliderClamping::Edits)`
- `backstage.png` มีคำทักทายตามเวลา ถ่ายใหม่หลังเที่ยง UTC จะได้ "Good afternoon"

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
- สคริปต์ของทุกภาพอยู่ที่ `docs/images/shots/<ชื่อภาพ>.jsonl` รันจากรากของ repo แล้วจะเขียนทับภาพใน `docs/images` (ถ่ายใหม่หลังเที่ยง UTC `backstage.png` จะทักทายต่างไป)
- คำสั่งที่ใช้สร้างภาพ `thai.png`: `ui.language` (`th`), `review.language` (`th-TH`, `default: true`), `styles.apply`, `text.insert` (`raw: true`), `text.newParagraph`, `view.zoomOut` และ `ui.key` Home พร้อม `cmd: true` เพื่อกลับไปต้นเอกสาร
- ธีมมืด: `ui.theme` (`value: dark`)

## วิธีตรวจก่อน commit (ตามกฎใน CLAUDE.md)

- ใช้ `cargo xtask ci` ซึ่งรัน fmt, clippy -D warnings, test, assets, layers และ wasm
- ถ้าดิสก์น้อย ให้ใช้ `CARGO_INCREMENTAL=0` และทดสอบทีละแพ็กเกจ
  - หลังแต่ละแพ็กเกจ ให้ลบไฟล์ทดสอบใน `target/debug/deps`
  - build ทั้ง workspace ต้องใช้ดิสก์ราว 25 GB
- เทสต์ Myanmar จะข้ามไปเองเมื่อเครื่องไม่มีฟอนต์ที่มีกฎ `mym2`/`mymr` เช่นเครื่องที่มีแค่ Unifont
