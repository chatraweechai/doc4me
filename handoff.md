# Handoff: Doc4Me 1.1.1 และงานภาษาไทย (งานใหม่อยู่ใต้ `[Unreleased]` ห้ามออกเวอร์ชันจนกว่าผู้ใช้จะสั่ง)

อัปเดต: 2026-10-25 · สาขา `main` · รอบที่ 8 · เวอร์ชันยังเป็น 1.1.1 ห้ามออกรุ่นใหม่จนกว่าผู้ใช้สั่ง · อ่านหัวข้อ "สถานะปัจจุบัน" ก่อน ส่วนที่อยู่ใต้ลงไปเป็นประวัติ

## สถานะปัจจุบัน (2026-10-25 รอบที่ 8)

### การตัดสินใจถาวรของผู้ใช้ (ทำตามเสมอ ไม่ต้องถามหรือแจ้งอีก)

- ตอบและแสดงข้อความเป็นภาษาไทยเท่านั้น (ชื่อคำสั่ง ชื่อไฟล์ และโค้ดคงเดิมได้)
- ห้ามออกเวอร์ชันใหม่จนกว่าผู้ใช้สั่ง (`Cargo.toml` 1.1.1, งานใหม่อยู่ใต้ `[Unreleased]`, ห้ามสั่ง workflow พร้อม `release: true`)
- release workflow ไม่ตั้ง `CRAFT_FONTS_DIR` ข้ามฟอนต์ CJK ของ craft-fonts
- parity ที่ต้องใช้บริการเสียเงินให้ตัดทิ้ง
- ไม่เพิ่ม job ทดสอบบน Linux ใน CI เว้นแต่ผู้ใช้สั่ง
- ยังติดตาม WordCraft ต้นทางต่อ (เอกสารที่อ้าง PR/issue ของ WordCraft คงไว้)
- ระบบเสริมที่เป็นของ Microsoft (VBA, Office add-ins/Office Store, OneDrive/SharePoint, co-authoring ของ Microsoft) อยู่นอกเป้าหมาย ใช้ open-source แทน ถ้าหาตัวแทนไม่ได้ให้ตัดเงียบ ๆ
- ไม่มีผู้ให้บริการ AI ในโปรแกรม
- TH SarabunIT๙ ไม่มีค่าวัด: เครื่องที่ไม่มีฟอนต์นี้แสดงด้วย Sarabun และแจ้งผู้ใช้ตอนเปิดไฟล์; ไม่ต้องให้เจ้าของภาษาตรวจ UI ไทย
- `.gitignore` คง `/target` ฯลฯ ไว้ (ผู้ใช้เลือกลบแค่บรรทัด `/.claude/`)

### รอบที่ 8 ทำแล้ว (รายละเอียดและผลตรวจอยู่ใน `CHANGELOG.md` `[Unreleased]`)

- `f158368` ลบบรรทัด `/.claude/` ใน `.gitignore`, ลบ `package-lock.json`, บันทึกคอมมิต `9954ae0` และ `fc7bab5` ใน CHANGELOG
- `adb9db9` TH SarabunIT๙: ใช้ Sarabun ขนาดของ Sarabun เอง และหน้าต่าง "ไม่มีฟอนต์นี้ในเครื่อง" ตอนเปิดไฟล์ (`cmd::file::unmatched_fonts`, `font_notice.rs`)
- `83e1611` ระบบเสริมของ Microsoft และ AI ในโปรแกรมออกจากเป้าหมาย (gap #22, #25) ตัวทดแทน: แมโครของ Doc4Me, แค็ตตาล็อก Add-in ของ Doc4Me, โฟลเดอร์ที่ Nextcloud Desktop/Syncthing ซิงก์
- คอมมิตถัดไป: ฝัง Intos และ Intos Display (OFL, github.com/muglug/intos) แทน Aptos/Aptos Display — วัดเทียบไฟล์ Aptos จริงแล้วตรงทุกความกว้าง (ตัดบรรทัดตรง 387/387) ตัวอื่นที่ใกล้ที่สุดคือ Lato 35.7%; แผนความพร้อม 80% ใน `docs/roadmap.md`; handoff นี้

### งานที่ต้องทำต่อ: ความพร้อมใช้งานจริง 80% แบบวัดได้ (ไม่ใช่ค่าประมาณ)

ตัวเลข ~60% ใน `ROADMAP.md` เป็นค่าประมาณ (ผลรวมถ่วงน้ำหนักของค่าที่ประมาณรายมิติ) ต้องสร้างเครื่องวัดก่อน แล้วแก้ตามผลจนได้ 80% ฉบับภาษาอังกฤษอยู่ที่ `docs/roadmap.md` หัวข้อ "Path to 80% measured readiness" ทุกข้อต้องบันทึก `[Unreleased]` และ commit/push ไป `main`

**ขั้น A สร้างเครื่องวัด (ทำก่อน)**

1. `cargo xtask readiness` (คำสั่งใหม่ใน `xtask/src/main.rs`): อ่านผลของชุดตรวจทุกมิติ คำนวณ Σ น้ำหนัก × อัตราผ่าน ด้วยน้ำหนักเดิม (ฟีเจอร์ 0.30, รูปแบบไฟล์ 0.20, การจัดหน้า 0.15, UI/UX 0.10, ความเสถียร 0.10, ประสิทธิภาพ 0.05, ภาษา 0.04, แพลตฟอร์ม 0.03, ระบบเสริม 0.02, ฮาร์ดแวร์ 0.01) เขียนผลลง `docs/readiness.md` แล้วให้ `ROADMAP.md` ใช้ตัวเลขนี้แทนค่าประมาณ รายการตรวจเพิ่มได้แต่ห้ามลบ
2. คลัง DOCX จริงอย่างน้อย 200 ไฟล์ที่ใช้ซ้ำได้ (เอกสารราชการที่เป็นสาธารณสมบัติ หรือสัญญาอนุญาตเปิด) แต่ละไฟล์มี PDF อ้างอิงที่ Word สร้าง (ดู Producer ของ PDF ว่าเป็น "Microsoft Word") จึงไม่ต้องมี Word ในเครื่อง เก็บไฟล์ใน `/corpus` (gitignored) และ manifest (URL, สัญญาอนุญาต, SHA-256) ใน repo ต่อยอดจาก `cargo xtask corpus`
3. ตรวจรูปแบบไฟล์ต่อไฟล์: เปิดได้ไม่ error, บันทึกแล้วเปิดใหม่ ข้อความ สไตล์ ตาราง รูป และ part ที่ไม่รู้จักยังอยู่
4. ตรวจการแบ่งหน้าเทียบ PDF อ้างอิง: จำนวนหน้า และคำแรก/คำสุดท้ายของทุกหน้า (ใช้ตัวอ่าน PDF ของ repo หรือ `pdftotext`)
5. ตรวจความเสถียร: เปิด จัดหน้า บันทึก ทุกไฟล์ในคลังภายใต้ timeout ต้องไม่มี panic หรือค้าง, soak test ของ autosave, fuzz `hostile_params_never_panic` ที่มีอยู่
6. รายการตรวจฟีเจอร์และ UI: ตารางคงที่ของ Word for Windows (ฟีเจอร์ กล่องโต้ตอบ ตัวเลือกในกล่องหลัก และรายการคีย์ลัดเริ่มต้นที่ Microsoft เผยแพร่) แต่ละแถวผูกกับเทสต์ที่รันได้ หรือการตรวจด้วยมือบน Windows (เจ้าของ repo)
7. เกณฑ์คงที่ของประสิทธิภาพ (เปิดเอกสาร 500 หน้า, latency การพิมพ์, การเลื่อน), ภาษา (ความครบของ catalog ต่อภาษาเทียบ 30 ภาษาของ Word), แพลตฟอร์ม (build และติดตั้งได้ต่อแพลตฟอร์ม), ฮาร์ดแวร์และระบบเสริมแบบผ่าน/ไม่ผ่าน

**ขั้น B วัดครั้งแรก** แล้วบันทึกตัวเลขที่วัดได้ใน `ROADMAP.md` และ handoff

**ขั้น C แก้ตามผลวัด** เรียงจากน้ำหนักที่ได้คืนต่อชั่วโมงมากที่สุด จนกว่า `cargo xtask readiness` บนคอมมิตล่าสุดได้ ≥ 80.0%

1. gap #1 ไฟล์ DOCX จริงที่เปิดไม่ได้หรือ round-trip เสีย
2. gap #4 การแบ่งหน้า: compatibility options, ตารางและรูปลอย, เทียบกับ PDF อ้างอิง (Aptos ใช้ Intos ที่ความกว้างตรงแล้ว)
3. gap #3 crash/ค้าง/บันทึกไม่ได้บน Windows (#170, #77, #59, #31, #218)
4. gap #2 Word ink (`w14:contentPart`) round-trip
5. gap #6 การพิมพ์แบบ native บน Windows (เลือกเครื่องพิมพ์ พิมพ์สองหน้า)
6. gap #7 กล่องโต้ตอบและตัวเลือกที่ขาด และ gap #24 คีย์ลัดที่ขาด ตามรายการข้อ A6 ที่ไม่ผ่าน
7. gap #13 แก้ไขแผนภูมิและ SmartArt ที่มาจาก Word
8. รัน alpha gate 6 workflow ใหม่บน Windows (เจ้าของ repo)

### งานของเจ้าของ repo (session ทำไม่ได้)

- tag `v1.0.0` ที่คอมมิต `7946cbb` (คำสั่งในข้อ 5.1 ด้านล่าง)
- ใบรับรองลงนามโค้ดหรือ Azure Trusted Signing แล้วใส่ secret (workflow พร้อมแล้ว)
- ruleset ป้องกัน `main` (ข้อ 5.3), ลบ branch `claude/optimistic-fermat-z9uknk` ที่ merge แล้ว, ตัดสินใจ PR ของ Dependabot #1–#4
- ทดสอบบนเครื่องจริง: Dictate กับไมโครโฟน, Screen Clipping บน Windows, เปิดไฟล์ที่มีวิดีโอออนไลน์ใน Word for Windows, สมการลายมือจริง, แจ้งรุ่น/build ของ Word for Windows ที่ใช้เทียบ, วัด alpha gate บน Windows

## session 2026-10-25 รอบที่ 7: งานค้างใน handoff ข้อ 2–4

- **ผู้ใช้สั่ง (2026-10-25): "ต่อไปนี้ให้ตอบและแสดงข้อความเป็นภาษาไทยเท่านั้น"** ใช้กับทุกข้อความที่ผู้ใช้เห็น รวมถึงข้อความแจ้งความคืบหน้าระหว่างทำงาน (ชื่อคำสั่ง ชื่อไฟล์ และโค้ดคงเดิมได้)
- ผู้ใช้สั่ง "ทำต่อ" (ทำงานค้างที่ session ทำได้ ข้ามงานของเจ้าของ repo)
- ทำแล้ว: เอกสารตรงกับโค้ด (`b303ed9`: ตัวเลข 625 คำสั่ง · 1,816 เทสต์ · ~188,000 บรรทัด, ROADMAP Upcoming, gap #29, README สองภาษา); `cargo audit` ไม่มีช่องโหว่ (คำเตือน unmaintained 5 รายการ)
- `cargo xtask ci` ทั้ง workspace (หลัง `cargo clean`, `CARGO_INCREMENTAL=0`): fmt, clippy, test (ผ่าน 1,809 ล้ม 0 ignored 0) ผ่าน แต่ล้มที่ `assets` เพราะสคริปต์ `docs/images/shots/*.jsonl` 11 ไฟล์ไม่มีแถวใน `ATTRIBUTION.md` (ค้างมาตั้งแต่ 2026-10-21) แก้แล้วใน `c6a91fe`; `assets`, `layers`, `wasm` ผ่าน — CI บน GitHub ไม่รันขั้นเหล่านี้ ควรรัน `cargo xtask ci` เองก่อนออกรุ่น
- ถ่ายภาพใน `docs/images` ใหม่ทั้ง 11 ภาพ: เปลี่ยนจริงแค่ `review.png` (ปุ่ม Translate) ที่เหลือต่างแค่เคอร์เซอร์/วันที่จึงคืนค่าเดิม
- ยังเหลือเฉพาะงานของเจ้าของ repo (ข้อ 5.1–5.3: แท็ก v1.0.0, secret ลงนาม, ruleset, ลบสาขาเก่า, PR Dependabot #1–#4) และการทดสอบด้วยมือบนเครื่องจริง (Dictate กับไมโครโฟน, Screen Clipping บน Windows, เปิดไฟล์ที่มีวิดีโอออนไลน์ใน Word, วัด parity/alpha gate ใหม่บน Windows)

## session 2026-10-25 รอบที่ 6: parity ที่เหลือ 8 รายการด้วยของฟรี/โอเพนซอร์ส

- ผู้ใช้สั่ง: "ทำส่วน parity ที่เหลือ 8 รายการ โดยเลือกใช้รายการ open-source หรือฟรีเป็นอย่างแรก หากจำเป็นต้องใช้แบบมีค่าใช้จ่ายให้ตัด parity นั้นทิ้งไป และไม่ต้องแจ้งอีก" และ "ไม่ต้องให้ release workflow ตั้ง CRAFT_FONTS_DIR ต่อไปนี้ข้ามฟอนต์ CJK ของ craft-fonts ไม่ต้องแจ้งอีก"
- **การตัดสินใจถาวรของเจ้าของ repo (ไม่ต้องถามหรือแจ้งอีก)**: (1) release workflow ไม่ตั้ง `CRAFT_FONTS_DIR` ข้ามฟอนต์ CJK ของ craft-fonts; (2) parity ที่ต้องใช้บริการเสียเงินให้ตัดออกจาก catalog โดยไม่ต้องแจ้ง
- ทำแล้ว (รายละเอียดและผลตรวจใน `CHANGELOG.md` `[Unreleased]`):
  - บริการเรียกเว็บ `Services::fetch` (desktop: `curl` ของระบบ https เท่านั้น; เว็บ: `fetch` ของเบราว์เซอร์) — `caa9274`
  - Online Pictures ผ่าน Openverse (ฟรี CC) พร้อมบรรทัดเครดิต — `caa9274`
  - Translate ผ่าน MyMemory (ฟรี) หรือ LibreTranslate (โอเพนซอร์ส) — `98b4cce`
  - Get Add-ins: แค็ตตาล็อก `addins/catalog.json` ใน repo นี้ + 3 ชุดแมโคร ตรวจ SHA-256 และคำสั่งที่อนุญาต — `d9b7edd` (ตรวจบน GitHub จริงแล้ว hash ตรง)
  - Ink Equation + Ink to Math ด้วยตัวจดจำ $P ของเราเอง — `af604a3`
  - Dictate ด้วย whisper.cpp บนเครื่อง (โมเดลจาก Hugging Face ตรวจ SHA-256) — `8aa0dea`; ทดสอบจริงด้วยเสียงสังเคราะห์ espeak-ng ผ่าน; CI Windows run 40178908529 ล้มเพราะ bindings ที่มากับ crate เป็นของ Linux → แก้โดยให้ bindgen สร้างบนแต่ละแพลตฟอร์ม (`6f4ded8`) แล้ว [run 40179588066](https://github.com/chatraweechai/doc4me/actions/runs/40179588066) ผ่าน (build + MSI + zip; release ข้าม)
  - Block Authors ตัดออก (ต้องใช้ SharePoint ของ Microsoft 365 แบบเสียเงิน)
- สภาพแวดล้อม: ติดตั้ง `libasound2-dev` (cpal) และ `espeak-ng` (ทดสอบ Dictate) ใน container; build ใช้ `CARGO_INCREMENTAL=0` และ `-p` ทีละ package เพราะดิสก์จำกัด (เคย `cargo clean` ได้คืน 26 GB)
- ทดสอบกับบริการจริง: `DOC4ME_NET_TESTS=1 cargo test -p doc4me fetch` (Openverse, httpbin), `DOC4ME_WHISPER_TEST=<wav 16-bit mono> cargo test -p doc4me real_model` (ดาวน์โหลดโมเดล 60 MB)

## session 2026-10-24: ปุ่มแปลงเลขไทย/อารบิก, JPG/JPEG และงานค้างจาก handoff

- ผู้ใช้สั่ง (ตามลำดับ)
  - "ดำเนินการต่อจากที่ค้างให้เสร็จ และเพิ่มปุ่มฟังก์ชันในริบบอนแปลงเลขอารบิกเป็นเลขไทย และแปลงเลขไทยเป็นเลขอารบิก"
  - "เมื่อทำเสร็จทั้งหมดรวมทั้งงานที่ค้าง ห้ามออกเวอร์ชันใหม่ ให้เพิ่มฟังก์ชันการ save as และ export file เป็นนามสกุลไฟล์ jpg/jpeg ด้วย"
  - กฎของผู้ใช้: push ไป `main` เท่านั้น, commit/push ก่อนแล้วตรวจบั๊กทีหลังแล้ว push อีกครั้ง, บันทึก handoff ทุกครั้ง
- **ห้ามออกเวอร์ชันใหม่** (ผู้ใช้สั่ง 2026-10-24): `Cargo.toml` ยังเป็น 1.1.1 และทุกอย่างอยู่ใต้ `[Unreleased]` อย่าสั่ง workflow พร้อม `release: true` จนกว่าผู้ใช้จะสั่ง
- ทำแล้ว (รายละเอียดและผลตรวจอยู่ใน `CHANGELOG.md` `[Unreleased]`)
  - ปุ่ม "แปลงเป็นเลขไทย" / "แปลงเป็นเลขอารบิก" บนริบบอน หน้าหลัก › ฟอนต์ (`format.thaiDigits`, `format.arabicDigits`)
  - Save As และ File › Export เป็น JPG/JPEG (`file.exportJpeg`, `SAVE_EXTS`, CLI, MCP, เว็บ)
  - ข้อ 1 (เวลา UTC): `now_local` ใช้กับวันที่และเวลาที่แสดงผู้ใช้ เพิ่ม dependency `chrono` (desktop เท่านั้น) วันที่ที่เก็บในไฟล์ยังเป็น UTC
  - ข้อ 2 และ 5.6: เอกสารที่ล้าสมัยแก้ตามที่ตรวจกับโค้ดแล้ว, Target เป็น Word for Windows (ไม่มีเลขรุ่น), `cargo xtask parity` ได้ 485/500 (97.0%)
  - ข้อ 3: xtask USAGE มี `version`, PngSuite ตรวจ SHA-256, `.github/dependabot.yml`; ข้อ 4: เลขเวอร์ชันใน `packaging/` และ `apps/` ไม่มีที่เขียนตายตัว (อ่านจาก `Cargo.toml`)
  - ข้อ 5.2 ส่วนของ session: workflow แยก build (ไม่มี secret) กับขั้นแพ็ก/ลงนาม (ได้ secret) และ `docs/releasing.md`
  - ข้อ 5.4: เจ้าของ repo ลบ `.claude/settings.json` แล้วใน `e5214c4` บันทึกใน `[Unreleased]` › ความปลอดภัย
- **[เจ้าของ repo]** ต้องทำเอง
  - ข้อ 5.1: tag และ Release `v1.0.0` (คำสั่งอยู่ในข้อ 5.1 ข้างล่าง ยังไม่มี tag นี้ ตรวจ 2026-10-24)
  - ข้อ 5.2: ใส่ secret ลงนามที่ Settings › Secrets and variables › Actions (ชื่ออยู่ใน `docs/releasing.md` ข้อ 2)
  - ข้อ 5.3: ruleset ป้องกัน `main`
  - ข้อ 5.5: ลบ branch `claude/optimistic-fermat-z9uknk` ที่หน้า Branches (merge เข้า `main` แล้ว ตรวจด้วย `merge-base --is-ancestor`; `git push origin --delete` ถูก git proxy ตัดการเชื่อมต่อสองครั้ง) และ branch `claude/affectionate-ptolemy-92v2ut` ของ session นี้ถ้ามีบน remote (session ไม่ได้ push branch นี้)
  - PR ของ Dependabot ([#1](https://github.com/chatraweechai/doc4me/pull/1) rust-cache 2.9.2 → 2.9.3, [#2](https://github.com/chatraweechai/doc4me/pull/2) checkout 4.4.0 → 7.0.1, [#3](https://github.com/chatraweechai/doc4me/pull/3) download-artifact 4.3.0 → 8.0.2, [#4](https://github.com/chatraweechai/doc4me/pull/4) upload-artifact 4.6.2 → 7.0.2): session ไม่ได้ merge เพราะเป็นการขึ้นเวอร์ชันหลักที่แก้ไฟล์ workflow และ PR ไม่รัน build (ไม่มี trigger `pull_request`) ให้อ่าน release notes ใน PR ก่อน merge โดยเฉพาะ download-artifact ที่ job `release` ใช้ แล้วสั่ง run บน `main` (ไม่ติ๊ก `release`) หลัง merge ห้าม merge ระหว่างที่ job `release` รันอยู่
- รอบที่ 3 (ผู้ใช้สั่ง: แก้วันที่ในบานหน้าต่างข้อคิดเห็น, ให้ปุ่มแปลงเลขเปลี่ยนเลขหน้าและเลขลำดับรายการ, เปลี่ยนทางลัด ⌘ ในเอกสารเป็นแบบ Windows) ทำแล้ว ดู `[Unreleased]`
  - `w:date` ของข้อคิดเห็นและ revision ใหม่เป็นเวลาท้องถิ่นแบบ Word และอ่าน `w16cex:dateUtc` (ยังไม่เขียนกลับ และยังไม่อ่าน `w16du:dateUtc` ของ revision)
  - ปุ่มแปลงเลขเปลี่ยนรูปแบบ decimal ↔ thaiNumbers ของรายการ เลขหน้า เลขเชิงอรรถ และเลขอ้างอิงท้ายเรื่อง
  - ทางลัดในเอกสารใช้ Ctrl (ไม่ใช่ ⊞ เพราะ Word for Windows ใช้ Ctrl) และคำแนะนำ "Paste"/"Go To" ในโปรแกรมแสดงทางลัดตามแพลตฟอร์ม
  - ตรวจแล้ว: เทสต์ doc, docx, engine (382 + 13 + 8), cli, mcp, layout (224), pdf ผ่านทั้งหมด, ui-egui 324 (`--include-ignored`), clippy ผ่าน; CI ของ `637bd89` (run 40106198767) ผ่าน job `release` ข้ามตามปกติ
  - **ผู้ใช้สั่งซ้ำ 2026-10-24: ห้ามออกเวอร์ชันใหม่จนกว่าจะสั่ง**
- รอบที่ 4 (ผู้ใช้สั่ง "แก้ข้อจำกัดที่เหลือทั้งหมด") ทำแล้ว ดู `[Unreleased]`
  - เขียน `commentsIds.xml` / `commentsExtensible.xml` (เวลา UTC ของข้อคิดเห็น) และอ่าน-เขียน `w16du:dateUtc` ของ tracked change (`Revision.date_utc`); ค่าของ namespace, relationship และ content type ตรวจกับ MS-DOCX และเอกสารของ Paper Instruments แต่ยังไม่ได้เทียบกับไฟล์ที่ Word บันทึกเอง **ควรเปิดไฟล์ที่ Doc4Me บันทึก (มีข้อคิดเห็นและ tracked change) ใน Word for Windows เพื่อยืนยันว่าไม่มีคำเตือนไฟล์เสีย**
  - ตรวจแล้ว: clippy `-D warnings` `--all-targets` (doc, docx, layout, engine, ui-egui, pdf) ผ่าน; เทสต์ engine 383 + 13 + 8, ui-egui 324 (`--include-ignored`), pdf, mcp, cli, docx และ layout ผ่านทั้งหมด; CI run 40108607965 ผ่าน job `release` ข้ามตามปกติ
  - ปุ่มแปลงเลขสลับ `\* Arabic` ↔ `\* ThaiArabic` ในฟิลด์ PAGE/NUMPAGES/SECTIONPAGES/SECTION/SEQ และตัวจัดวางรู้จัก `ThaiArabic`, `ThaiCardText`, `ThaiLetter`
  - เอกสาร parity: เป้าหมาย Word for Windows อธิบายใน `docs/target-app-parity.md`, แถวเครื่องพิมพ์และสแกนเนอร์ใน `docs/hardware-parity.md`, แถว Platforms ตรงกับ Release จริง
- รอบที่ 5 (2026-10-25, ผู้ใช้สั่ง "ตรวจสอบ parity ที่เหลือ ทำให้เสร็จทั้งหมด และดำเนินการส่วนที่ค้างใน handoff ยกเว้นที่ผู้ใช้ต้องทำเอง")
  - parity ที่ขาด 15 รายการ แบ่งเป็น ทำได้เอง: View › Window (เสร็จแล้ว), Screenshot, Online Video, 3D Models; ต้องให้เจ้าของ repo ตัดสินใจหรือใช้บริการภายนอก (ข้าม): Dictate (โมเดลเสียง, gap #28), Online Pictures (Bing), Get Add-ins (Office Store, gap #22), Translate (บริการแปล), Ink Equation และ Ink to Math (โมเดลจำลายมือ), Block Authors (co-authoring ของ SharePoint, gap #22)
  - View › Window: `view.newWindow` และ `view.split` เดิมเป็น stub ตอนนี้ทำงานจริง และเพิ่ม `view.arrangeAll`, `view.sideBySide`, `view.syncScroll`, `view.switchWindows` (`crates/ui-egui/src/windows.rs`) parity 489/500
  - `cargo audit`: อัปเดต zip 3 → 9.1 (ช่องโหว่ 3 รายการใน `extract` ซึ่งโค้ดไม่เรียก), ctutils, tokio, clipboard-win; เหลือคำเตือน unmaintained 5 รายการ
  - ดิสก์ของ container เคยเต็ม (linker ล้มด้วย Bus error) แก้โดยลบไฟล์ทดสอบขนาดใหญ่ใน `target/debug/deps` และ `target/debug/examples`
  - Insert › Screenshot › Screen Clipping (`a47ba63`, parity 490): engine `insert.screenshot` (pure) → UI `screen_clip.rs` → host `apps/doc4me/src/screen_clip.rs` (Windows `ms-screenclip:` + `clipboard_win::raw::seq_num`, macOS `screencapture -i -x`, Linux XDG Screenshot portal แล้ว gnome-screenshot/spectacle/xfce4-screenshooter/flameshot) ตรวจ `cargo clippy -p doc4me --target x86_64-pc-windows-msvc` และ `x86_64-apple-darwin` ผ่าน; CI Windows ผ่าน
  - Insert › Online Video (`4cb88d6`, parity 491): `WebVideo` (`crates/doc/src/video.rs`) ในฟิลด์ใหม่ `InlineObject::Image.video`, .docx `wp15:webVideoPr` ที่ ext uri `{C809E66F-F1BF-436E-B5F7-EEA9579F0CBA}` (ยืนยันจากตัวอย่าง OOXML จริง ไม่ใช่เดา), กล่อง Insert Video, ดับเบิลคลิก/`picture.playVideo` เปิดเบราว์เซอร์, โปสเตอร์วาดเอง
  - Insert › 3D Models (`4cb88d6` + `46ef579`, parity 492): `crates/engine/src/model3d.rs` อ่าน .glb/.stl/.obj วาดเป็นรูปด้วย z-buffer; หลัง push ตรวจพบและแก้ node glTF ที่วนอ้างตัวเอง + จำกัดงานอ่านรวม (`MAX_WORK`)
  - parity ที่เหลือ 8 รายการ (ข้ามตามคำสั่ง เพราะต้องใช้บริการภายนอกหรือโมเดล ML ที่เจ้าของ repo ต้องตัดสินใจ): Dictate, Online Pictures, Get Add-ins, Ink Equation (อยู่ทั้ง Insert และ Draw จึงนับ 2), Ink to Math, Translate, Block Authors (co-authoring ของ SharePoint) ตาม `docs/parity-checklist.md`
  - งานค้างใน handoff ที่ทำแล้ว: wasm check (`rustup target add wasm32-unknown-unknown`; `cargo clippy -p doc4me-web --target wasm32-unknown-unknown -- -D warnings` ผ่านหลังแก้ lint 3 จุด), สาเหตุ audit ภาษาที่ติด `#[ignore]` (ข้อความผิดพลาด Read Aloud ไม่แปลและมาถึงแบบ async; แก้แล้วเอา `#[ignore]` ออก ทดสอบ 24/24 ภายใต้โหลด), เอกสารล้าสมัย (file-format-parity การเข้ารหัส, roadmap M1/M2/M3/M8/M11/M12 + 618 คำสั่ง, package.ps1 อ้าง windows-arm64.yml, env.sh/ATTRIBUTION อ้างว่า release ฝัง craft-fonts — ตรวจไฟล์ workflow ที่ tag v1.1.1 แล้วไม่มี, docs/mcp.md "over 270")
  - ดิสก์ของ container เต็มซ้ำ 2 ครั้ง: `cargo build --workspace --all-targets` สร้าง dependency ชุดที่สอง (feature ต่างกัน) และ `target/debug/incremental` ใหญ่ ~11 GB — ใช้ `CARGO_INCREMENTAL=0` และ build/test ทีละ package (`-p`) แทน
- ยังไม่ได้ทำ / ข้อสังเกต
  - ภาพ `hero`, `thai`, `dark`, `dark-th`, `font`, `font-th` ถ่ายใหม่แล้ว ภาพอื่นใน `docs/images` (backstage, design, layout, references, review) ไม่เห็นกลุ่มฟอนต์ของหน้าหลักจึงไม่ได้ถ่ายใหม่
  - ตัวเลข parity และ alpha gate ยังเป็นค่าที่วัดกับ Word for Mac ต้องวัดใหม่บนเครื่อง Windows ที่มี Word (session บนคลาวด์ทำไม่ได้)
  - ข้อ 3 ที่เหลือ: job ทดสอบบน Linux ใน CI รอเจ้าของ repo ตัดสินใจ (`cargo audit` รันแล้วในรอบ 5; audit ภาษาที่ติด `#[ignore]` หาสาเหตุและแก้แล้ว)
  - **[เจ้าของ repo]** เพิ่มเติมจากรอบ 5: เปิดไฟล์ .docx ที่ Doc4Me บันทึกซึ่งมีวิดีโอออนไลน์ใน Word for Windows เพื่อยืนยันว่า Word เล่นได้; ทดสอบ Screen Clipping และ Dictate (ไมโครโฟน) บน Windows/macOS/เดสก์ท็อป Linux ด้วยมือ
  - ข้อ 5.7 ทั้งหมดยังรอเจ้าของ repo
- CI ของ `main`: run ของ `21bb34a` (40100455417) และ `b65c320` (40100747357) ถูกยกเลิกโดย concurrency เพราะมี push ใหม่ (ไม่ใช่การล้ม); [run 40101348687](https://github.com/chatraweechai/doc4me/actions/runs/40101348687) ของ `0cdf2bb` (คอมมิตโค้ดล่าสุด) **ผ่าน** (ตรวจ 14:26 UTC): job `build` สำเร็จทุกขั้น รวมขั้นใหม่ "Build doc4me + doc4me-cli" (14:11:45–14:22:23) และ "Sign and package MSI + portable zip" (14:22:23–14:23:05) ส่วน job `release` ถูกข้ามตามปกติของ push; คอมมิตหลังจากนั้นแก้แค่เอกสารและภาพ จึงใส่ `[skip ci]` run นี้จึงเป็น build ล่าสุดของโค้ด
- ผลตรวจในเครื่องรอบสุดท้าย: engine 378 + 13 + 8, cli/mcp/render ผ่าน, ui-egui 324 (`--include-ignored`), clippy `-D warnings` `--all-targets` ของ engine, ui-egui, cli, mcp, render, xtask ผ่าน, `cargo fmt --all --check` ผ่าน, `cargo xtask parity` 485/500; ภาพหน้าจอยืนยันปุ่มแปลงเลขและผลการแปลง
- ยังไม่ได้รัน `cargo xtask ci` ทั้ง workspace (wasm target ไม่ได้ติดตั้ง และดิสก์จำกัด) ส่วน wasm ของ `local_offset_secs` ตรวจแค่ว่า `js_sys::Date::new_0` และ `get_timezone_offset` มีใน js-sys 0.3.106

## งานที่ต้องทำต่อใน session ใหม่ (รวบรวม 2026-10-23)

**ประวัติ (2026-10-25 รอบที่ 8)**: ข้อ 1–4 และ 5.4–5.7 ปิดแล้ว ส่วน 5.1–5.3 เหลือเฉพาะงานของเจ้าของ repo ซึ่งสรุปไว้ในหัวข้อสถานะปัจจุบัน

ที่มา: workflow สำรวจงานค้าง (agent อ่านอย่างเดียว 2026-10-22 15:15 UTC) ซึ่งขั้นตรวจซ้ำล้มเพราะชนขีดจำกัดของ session

- ข้อที่มี **[ตรวจแล้ว]** ตรวจกับโค้ดเมื่อ 2026-10-23 04:10–04:15 UTC ข้ออื่นมาจาก finder ตัวเดียว ต้องตรวจซ้ำก่อนลงมือ
- ลำดับที่แนะนำ (ผู้ใช้ปรับได้):
  1. แจ้งงานที่ **[เจ้าของ repo]** ต้องทำเองในข้อ 5.1–5.3 ตั้งแต่ต้น session
  2. ทำข้อ 5.4 และ 5.5 (งานเล็ก)
  3. ทำข้อ 1 (บั๊ก UTC)
  4. ทำข้อ 2 รวมกับข้อ 5.6 (เอกสาร)
  5. ทำส่วนของ session ในข้อ 5.2 แล้วต่อด้วยข้อ 3
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

**สถานะ 2026-10-25 รอบที่ 7: ปิดแล้ว** ตรวจทุกข้อกับโค้ดแล้ว ส่วนใหญ่แก้ในรอบ 5 (2026-10-24) ที่เหลือแก้ใน `b303ed9`: ตัวเลข 625 คำสั่ง · 1,816 เทสต์ · ~188,000 บรรทัด, ROADMAP Upcoming ข้อ 1 (PR ที่เปิดอยู่มีแค่ Dependabot #1–#4), gap #29, README ทั้งสองภาษา รายการข้างล่างเก็บไว้เป็นประวัติ

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

**สถานะ 2026-10-25 รอบที่ 7**: xtask USAGE และ checksum ของ PngSuite แก้แล้ว (รอบ 5), `.github/dependabot.yml` มีแล้ว (PR #1–#4 รอเจ้าของ repo), `cargo audit` รันซ้ำหลังเพิ่ม whisper-rs/cpal: ไม่มีช่องโหว่ เหลือคำเตือน unmaintained 5 รายการ (`rustybuzz`, `ttf-parser` ผ่าน krilla; `zbus_names`, `zvariant`, `zvariant_derive` ผ่าน zbus/accesskit บน Linux), ผล `cargo xtask ci` ทั้ง workspace ดูหัวข้อรอบที่ 7 ด้านบน; job CI บน ubuntu และการลงนามยังรอเจ้าของ repo

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
  - ผู้ใช้ตัดสินใจแล้วว่าจะลงนาม ดูขั้นตอนในข้อ 5.2
- `xtask/src/main.rs`: ข้อความ USAGE (:16-30) ไม่มีคำสั่ง `version` และ `corpus --download` โหลด PngSuite ผ่าน `http://` โดยไม่ตรวจ checksum (:202, :226-231)

### 4. ส่วนที่ยังไม่ได้สำรวจ (agent ล้มก่อนทำ)

**สถานะ 2026-10-25 รอบที่ 7**: เลขเวอร์ชันใน `packaging/` และ `apps/` มาจาก workspace ทั้งหมด (ไม่มีเลขตายตัว); ข้อความ "isn't available/supported" ที่ผู้ใช้เจอได้เป็นข้อจำกัดที่ตั้งใจ (เช่นเว็บ, รหัสผ่านชนิดที่ไม่รองรับ) ไม่ใช่ stub; คีย์แปลที่ขาดมีเทสต์ i18n คุม; ภาพใน `docs/images` ถ่ายใหม่ในรอบที่ 7

- โค้ดและเทสต์
  - stub, `unimplemented!` หรือ `todo!` ที่ผู้ใช้เรียกถึงได้
  - คำสั่งใน catalog ที่ไม่มี handler หรือ UI
  - คีย์แปลที่ขาดใน `crates/ui-egui/src/i18n/*.tsv` ทั้ง 18 ไฟล์
  - เลขเวอร์ชันใน `packaging/` และ `apps/` ตรงกับ 1.1.1 หรือไม่
- ตรวจความครบ: ข้อความใน `README.md` และ `README.th.md`, `AGENTS.md`, ความใหม่ของภาพใน `docs/images`, `apps/doc4me-web`, `docs/mcp.md`, `ATTRIBUTION.md` และ `NOTICE`, `contributors/`
- ผล grep ตอนเริ่ม session: ไม่พบ `TODO`, `FIXME`, `todo!(` หรือ `unimplemented!(` ใน `crates`, `apps` และ `xtask` และพบ `#[ignore]` จุดเดียวคือ audit ในข้อ 3

### 5. งานที่ผู้ใช้ตัดสินใจแล้ว ให้ลงมือได้เลย (สั่ง 2026-10-23)

ผู้ใช้สั่งว่า "tag / release v1.0.0, ใบรับรองลงนามโค้ด, การตั้งค่าป้องกัน main, สิทธิ์ push tag ใน .claude/settings.json, การลบ branch เก่า, จะเทียบกับ Word for Windows ด้วยเลยไม่ต้องรอการตัดสินใจ"

- บางขั้น session บนคลาวด์ทำเองไม่ได้ เพราะ push tag ไม่ได้ (git proxy ตอบ 403) และไม่มีสิทธิ์ admin ของ repo
- ขั้นเหล่านั้นติดป้าย **[เจ้าของ repo]** ให้แจ้งผู้ใช้ตั้งแต่ต้น session พร้อมคำสั่งหรือเมนูที่ต้องกด

#### 5.1 tag และ Release `v1.0.0` [ตรวจแล้ว]

- ใช้คอมมิต `7946cbb57a0f4618711e765a844f46194150ec1f` (2026-10-15, [run 38742771852](https://github.com/chatraweechai/doc4me/actions/runs/38742771852) ผ่าน)
  - `Cargo.toml` ของคอมมิตนี้เป็น 1.0.0 และ CHANGELOG ของคอมมิตนี้มีหัวข้อ `## [1.0.0] - 2026-10-15`
  - โค้ดเหมือน `ca0e607` ([run 38734047702](https://github.com/chatraweechai/doc4me/actions/runs/38734047702)) ซึ่ง `[1.0.0]` › ตรวจสอบ อ้างว่าเป็น build ของ 1.0.0
  - `git diff --stat ca0e607 7946cbb` ต่างกันแค่ workflow, `CHANGELOG.md`, `CLAUDE.md`, `AGENTS.md` และ `docs/releasing.md`
  - คอมมิตถัดไปคือ `1d05c27` (run 8, 2026-10-19) เริ่มมีฟีเจอร์ของ 1.1.0 จึงใช้ไม่ได้
  - ทั้งสองคอมมิตไม่อยู่ในประวัติของ `main` เพราะประวัติปัจจุบันเริ่มที่ `a4ae748` (2026-10-20) แต่ยังดึงจาก GitHub ด้วย SHA เต็มได้ (ลองแล้ว 2026-10-23)
- **[เจ้าของ repo]** รันบนเครื่องตัวเอง (workflow_dispatch `release: true` ติด tag ได้แค่ HEAD ของ `main` ตามเวอร์ชันใน `Cargo.toml` จึงใช้กับ 1.0.0 ไม่ได้):
  - `git fetch origin 7946cbb57a0f4618711e765a844f46194150ec1f`
  - `git tag -a v1.0.0 7946cbb57a0f4618711e765a844f46194150ec1f -m "Doc4Me 1.0.0"`
  - `git push origin v1.0.0`
  - ถ้า push ถูกปฏิเสธเรื่อง `workflow` scope ให้ใช้ token ที่มีสิทธิ์นี้ เพราะไฟล์ workflow ของคอมมิตนั้นต่างจาก `main`
- การ push tag จะรัน workflow ของคอมมิตนั้น (มี trigger `tags: ["v*"]` และ job `release`): build ใหม่จากซอร์ส 1.0.0 แล้วสร้าง Release โดยใช้หัวข้อ `[1.0.0]` เป็นบันทึก
  - job `release` อาจได้ 403 แบบรอบแรกของ 1.1.1 เพราะไฟล์ workflow ของคอมมิตต่างจาก default branch ยังไม่รู้ว่าเงื่อนไขนี้ใช้กับ tag ที่มีอยู่แล้วด้วยหรือไม่
  - ถ้า job ใดล้ม **[เจ้าของ repo]** ให้สร้าง Release เองที่ Releases › Draft a new release
    - เลือก tag `v1.0.0` และใช้หัวข้อ `[1.0.0]` ของ `7946cbb` เป็นบันทึก
    - แนบ MSI และ zip จาก artifact `doc4me-windows-x64` ของ run 38742771852 (69,888,334 ไบต์ หมดอายุ 2027-01-13)
    - ไม่ต้องติ๊ก "Set as the latest release"
- หลังออก:
  - ตรวจว่าป้าย Latest ยังอยู่ที่ v1.1.1 ถ้าย้ายไป v1.0.0 ให้แก้ Release v1.1.1 กลับเป็น latest
    - เหตุผล: `gh release create` ของคอมมิตนั้นไม่ได้ใส่ `--latest=false` และค่าเริ่มต้นของ GitHub อาจตั้ง Release ที่เผยแพร่ใหม่เป็น latest (ยังไม่ได้ตรวจ)
  - ตรวจไฟล์เหมือนตอน 1.1.1: sha256 ตรงกับ digest และ `doc4me.exe` กับ MSI เป็น 1.0.0
  - session บันทึกใน `CHANGELOG.md` `[Unreleased]` › แก้ไข ว่าลิงก์ `[1.0.0]` ใช้ได้แล้ว ห้ามแก้หัวข้อ `[1.0.0]` และ `[1.1.0]`
  - ลิงก์ `[1.1.0]` ให้ชี้หน้า Release ต่อไป เพราะ `v1.0.0` กับ `v1.1.0` อยู่คนละประวัติ หน้าเปรียบเทียบจึงใช้ไม่ได้

#### 5.2 ลงนามโค้ด

- ส่วนของ session (ทำได้เลย แต่ห้ามทำขณะ job `release` รันอยู่):
  - เพิ่ม `env:` ใน step "Build MSI + portable zip" ของ `.github/workflows/windows-x64-installer.yml` ให้รับค่าจาก `secrets.*`
    - ชื่อตาม `packaging/windows/sign.ps1:7-10`: `WINDOWS_CERTIFICATE`, `WINDOWS_CERTIFICATE_PASSWORD`, `AZURE_TENANT_ID`, `AZURE_CLIENT_ID`, `AZURE_CLIENT_SECRET`, `AZURE_SIGNING_ENDPOINT`, `AZURE_SIGNING_ACCOUNT`, `AZURE_CERT_PROFILE`
  - secret ที่ยังไม่ได้ใส่จะได้ค่าว่าง `sign.ps1:48` ใช้ `[bool]$env:WINDOWS_CERTIFICATE` จึงยังข้ามการลงนามพร้อมคำเตือนเหมือนเดิม [ตรวจแล้ว] (ให้ตรวจการเช็ก `AZURE_*` ที่ :49-53 ด้วย)
  - แก้ `docs/releasing.md:14-15` ให้บอกชื่อ secret และวิธีตั้ง แล้วบันทึก `[Unreleased]`
- **[เจ้าของ repo]** สมัครบริการลงนาม แล้วใส่ secret ที่ Settings › Secrets and variables › Actions
  - ตามข้อกำหนดของ CA/Browser Forum ใบรับรอง code signing ที่ออกตั้งแต่ 2023-06-01 ต้องเก็บกุญแจในฮาร์ดแวร์ (token หรือ HSM) ส่วนใหญ่จึงส่งออกเป็น .pfx มาใส่ `WINDOWS_CERTIFICATE` ไม่ได้
  - ให้ถามผู้ขายว่ามีการลงนามบนคลาวด์ที่ใช้ใน CI ได้หรือไม่ หรือใช้ Azure Trusted Signing ซึ่ง `sign.ps1` รองรับอยู่แล้ว
  - ตรวจเงื่อนไขประเทศและประเภทผู้สมัครของ Azure Trusted Signing ก่อนสมัคร (session นี้ยังไม่ได้ตรวจ)
- หลังใส่ secret ให้สั่ง run บน `main` (ไม่ต้องติ๊ก `release`) แล้วตรวจว่า log ไม่มีคำเตือน "leaving unsigned" และ `doc4me.exe`, `doc4me-cli.exe` กับ MSI มีลายเซ็น

#### 5.3 ป้องกัน `main`

- **[เจ้าของ repo]** ตั้งที่ Settings › Rules › Rulesets › New ruleset › New branch ruleset (session ไม่มีสิทธิ์ admin และไม่มีเครื่องมือตั้ง ruleset)
  - Enforcement status: Active และ Target branches: Include default branch
  - เปิด "Restrict deletions" และ "Block force pushes"
  - อย่าเปิด "Require a pull request before merging" เพราะ session push ตรงไป `main` ตาม `CLAUDE.md`
  - อย่าเปิด "Require status checks to pass" จนกว่าจะมี job ทดสอบ (ข้อ 3) และอย่าเปิด "Restrict updates"
- ถ้าจะกัน tag ด้วย ให้สร้าง tag ruleset สำหรับ `v*` เปิด "Restrict updates" และ "Restrict deletions"
  - อย่าเปิด "Restrict creations" เพราะ job `release` ใช้ `GITHUB_TOKEN` สร้าง tag ของรุ่นใหม่
- immutable releases ยังไม่ต้องเปิด เพราะจะแก้บันทึกหรือไฟล์ของ Release หลังเผยแพร่ไม่ได้ (เคยต้องเพิ่มหมายเหตุท้าย v1.1.1) ถ้าจะเปิดให้รอออก v1.0.0 เสร็จก่อน
- หลังตั้ง session ตรวจแค่ว่า push ปกติไป `main` ยังผ่าน (ห้ามทดสอบด้วย force-push) แล้วบันทึกใน handoff

#### 5.4 ถอนสิทธิ์ push tag ใน `.claude/settings.json` [ตรวจแล้ว]

- ไฟล์นี้มีแค่ `permissions.allow` สองรายการ คือ `Bash(git tag -a v*)` และ `Bash(git push origin v*)`
- session ลบสองรายการนี้ ถ้าไฟล์ไม่เหลือค่าอื่นให้ลบทั้งไฟล์ แล้วบันทึก `[Unreleased]` › ความปลอดภัย
- เหตุผล:
  - ตอนนี้ออกรุ่นผ่าน workflow_dispatch และ `v1.0.0` เจ้าของ repo push เอง session บนคลาวด์ก็ push tag ไม่ได้อยู่แล้ว
  - การอนุญาตแบบไม่ถามจึงไม่ได้ใช้ แต่เปิดทางให้ push tag สาธารณะ (ซึ่งสร้าง Release) โดยไม่มีการยืนยัน
- `.gitignore:9` (`/.claude/`) ไม่กระทบการแก้ไฟล์นี้ เพราะไฟล์ถูก track อยู่แล้ว

#### 5.5 ลบ branch เก่า [ตรวจแล้ว]

- ผล `git ls-remote --heads origin` เมื่อ 2026-10-23: มี `main` และ `claude/optimistic-fermat-z9uknk` (`be5893e` คือคอมมิตของ tag `v1.1.1` merge เข้า `main` แล้ว) ส่วน `claude/modest-maxwell-3q1mya` ไม่มีบน remote แล้ว
- session ตรวจทุก branch ด้วย `git merge-base --is-ancestor origin/<branch> origin/main` ก่อน แล้วรัน `git push origin --delete <branch>`
  - ถ้า proxy ตอบ 403 **[เจ้าของ repo]** ให้ลบที่หน้า Branches ของ GitHub
- ห้ามลบ branch ที่ยัง merge ไม่หมด หรือ branch ของ session ที่ยังทำงานอยู่ และ session ใหม่อาจสร้าง branch `claude/*` เพิ่ม จึงต้องตรวจใหม่ทุกครั้ง

#### 5.6 ใช้ Word for Windows เป็นเป้าหมาย parity

- เปลี่ยนหัวเอกสาร "Target: Microsoft Word (Microsoft 365) for Mac 16.113.4" ใน 11 ไฟล์ [ตรวจแล้ว]
  - ไฟล์: `ROADMAP.md`, `docs/architecture.md`, `docs/file-format-parity.md`, `docs/gaps.md`, `docs/hardware-parity.md`, `docs/layout-parity.md`, `docs/localization-parity.md`, `docs/roadmap.md`, `docs/target-app-parity.md`, `docs/typing-parity.md`, `docs/ui-parity.md`
  - ถามผู้ใช้ว่าเทียบกับ Word for Windows รุ่นและ build ไหน (ดูได้ที่ File › Account › About Word) ถ้ายังไม่รู้ให้เขียน "Microsoft Word (Microsoft 365) for Windows" โดยไม่ใส่เลขรุ่น
- ตรวจแถวที่อิงพฤติกรรมของ Mac ใหม่:
  - ทางลัด Cmd เทียบกับ Ctrl (`docs/typing-parity.md`)
  - เมนูของ Mac เทียบกับ ribbon และ backstage ของ Windows (`docs/ui-parity.md`, `docs/target-app-parity.md`)
  - `docs/hardware-parity.md`
  - ระยะบรรทัดทำตาม Windows อยู่แล้ว (`docs/layout-parity.md:24`)
- alpha gate ใน `docs/roadmap.md:28-29` ระบุ "main platform (macOS desktop)" ให้เปลี่ยนเป็น Windows
  - การรัน alpha gate ใหม่ต้องใช้เครื่อง Windows ซึ่ง session บนคลาวด์ทำไม่ได้ ให้ระบุว่ายังไม่ได้วัดใหม่
  - ตัวเลข readiness (~60%/55%/63%) ยังเป็นค่าที่วัดใน WordCraft
- ทำพร้อมข้อ 2 ในคอมมิตเดียวได้ และบันทึก `[Unreleased]` › เปลี่ยนแปลง

#### 5.7 ยังรอเจ้าของ repo ตัดสินใจ

**สถานะ 2026-10-25 รอบที่ 8: ผู้ใช้ตัดสินแล้วทุกข้อ** (ดู "การตัดสินใจถาวร" ในหัวข้อสถานะปัจจุบัน) ฟอนต์แทน Aptos คือ Intos ที่ฝังแล้ว

- `.gitignore:9` มี `/.claude/` ไฟล์ใหม่ใต้ `.claude/` จึงถูกละเว้น, `package-lock.json` เป็นไฟล์ว่าง (`fc7bab5`) และคอมมิต `9954ae0` กับ `fc7bab5` ไม่มีรายการใน CHANGELOG
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

### session 2026-10-22 15:15 – 2026-10-23 04:30 UTC: สำรวจงานค้าง (ยกเลิกกลางทาง)

- ผู้ใช้สั่งตามลำดับ: "ตรวจสอบงานค้างและงานเบื้องหลังทั้งหมด", "ยกเลิกงานทั้งหมด บันทึก handoff สำหรับเริ่ม session ใหม่" และ "บันทึก handoff สิ่งที่ต้องทำต่อใน session ใหม่ทั้งหมด"
- งานเบื้องหลัง (ตรวจ 2026-10-22 15:13 UTC): ไม่มี scheduled task, task list, PR หรือ issue ที่เปิด และไม่มี run ค้าง
  - branch `claude/*` บน remote ถูก merge เข้า `main` หมดแล้ว
  - session อื่นในบัญชี "รวมโค้ด doc-scanner-pdf" เป็นคนละ repo ตามสรุปของ session นั้น รอเจ้าของใส่ secret `SITE_DEPLOY_KEY` และ publish เว็บเอง
- สั่ง workflow ให้ agent อ่านอย่างเดียว 4 ด้าน (handoff/CHANGELOG, roadmap/gaps, โค้ด/เทสต์, CI/infra) แต่ชนขีดจำกัดของ session
  - finder ด้านโค้ด/เทสต์, ตัวตรวจซ้ำทั้ง 4 ตัว และตัวตรวจความครบล้มหมด
  - ผลที่ได้ย้ายไปไว้ในหัวข้อ "งานที่ต้องทำต่อใน session ใหม่" ข้างบน และตรวจข้อสำคัญกับโค้ดแล้ว (2026-10-23)
- แก้แค่ `handoff.md` (คอมมิต `92e1929`, `30e8da4` และคอมมิตล่าสุด ใส่ `[skip ci]` ทุกคอมมิต) จึงไม่ได้แก้ `CHANGELOG.md`
  - `30e8da4` ทำหมายเหตุไว้ที่ข้อมูลเก่าในหัวข้อประวัติของ handoff
- 2026-10-23 ผู้ใช้ตัดสินใจ 6 เรื่องที่เคยรอเจ้าของ repo (v1.0.0, ลงนามโค้ด, ป้องกัน `main`, สิทธิ์ push tag, ลบ branch เก่า, เทียบกับ Word for Windows)
  - เขียนข้อ 5 ใหม่เป็นขั้นตอนที่ลงมือได้
  - ตรวจข้อเท็จจริงที่ใช้วางขั้นตอนแล้ว: คอมมิตและ artifact ของ 1.0.0 (ผ่าน GitHub API และ `git fetch` ด้วย SHA), workflow ของ `7946cbb`, `.claude/settings.json`, การอ่าน secret ของ `sign.ps1`, branch บน remote และไฟล์ที่ยังตั้งเป้าเป็น Word for Mac
  - ยังไม่ได้ลงมือทำงานใดในข้อ 5

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
