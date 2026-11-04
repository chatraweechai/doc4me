# Handoff: Doc4Me 1.1.2 · readiness 87.7% (วัดบน Windows รอบที่ 21 ด้วย workflow `readiness-windows.yml` run 41608625168 ยืนยันผลของ run 41596836985, 2026-11-03; ก่อนหน้า 87.6%, 87.5%, 87.3%, 87.2%, 87.1%, 87.0%, 86.9%, 86.8%, 86.7% และ 86.0% รอบที่ 17) · ออก Release v1.0.0, v1.1.0, v1.1.1, v1.1.2 แล้วใน fork `veevie55/doc4me` (v1.1.2 เป็น Latest) · งานใหม่อยู่ใต้ `[Unreleased]` ห้ามออกเวอร์ชันถัดไปจนกว่าผู้ใช้จะสั่ง

อัปเดต: 2026-11-03 · สาขา `main` ของ fork `veevie55/doc4me` · รอบที่ 22 (บันทึกผลวัด readiness ล่าสุด และแก้การวัดบน Windows) · เวอร์ชัน 1.1.2 · อ่านหัวข้อ "สถานะปัจจุบัน" ก่อน ส่วนที่อยู่ใต้ลงไปเป็นประวัติ

## สถานะปัจจุบัน (2026-11-02 รอบที่ 20: ออกรุ่นครบใน fork `veevie55/doc4me` แล้วทำงานต่อด้าน pagination)

### ความคืบหน้าสู่ 100% ไม่นับ deferred (รอบที่ 22 ต่อ, 2026-11-03/04)

- **เจ้าของเขียนทับ `main` ของ fork อีกครั้ง (2026-11-04 02:02–02:37 UTC)**: `main` ใหม่ = `d084ccc` (เนื้อไฟล์เท่า `c9ff26b` เดิม, tag `v1.1.3` ชี้ที่นี่) + 6 commit "Release 1.1.3"/"Initial commit" ที่แก้แค่ `Cargo.toml` (1.1.3) และ `Cargo.lock`; commit 65 รายการหลัง `c9ff26b` (dialogs รอบที่ 21, 40 ภาษา, ตัดบรรทัด LB15, แก้ readiness xtask ฯลฯ) หายจาก `main` จึง rebase ต่อบนงานของเจ้าของแบบ fast-forward (ไม่ force; ต้นไม้ต่างจากก่อน rebase แค่เลขเวอร์ชัน) และแก้ `Cargo.lock` ให้ตรง 1.1.3 (`3030a22`); hash ของ commit รอบนี้จึงเปลี่ยนทั้งหมด ถ้าเจ้าของตั้งใจตัดงานหลัง `c9ff26b` ทิ้ง ต้องตัดสินใหม่
- ลองแก้ "ด้านที่ `w:tblCellMar`/`w:tcMar` ไม่ระบุให้สืบทอดจาก style" แล้ว**ย้อนออก**: zenodo-154 แสดงว่า Word ใช้ 0 กับด้านที่ `w:tblCellMar` ไม่ระบุ (บรรทัด "LEVEL 1 USE means that" ตรงเมื่อใช้ 0) และไม่แก้ govuk-006; สาเหตุจริงของ govuk-006 (หน้า 5 มีแค่ย่อหน้า "© Crown copyright" ที่ Word ใส่ท้ายหน้า 4) ยังไม่พบ — แถวสูงของเราเกิน Word ~16 pt ที่หน้า 4
- Windows run 41673874009 (`4556010`, 16 ภาษาใหม่): **88.4%**, **90.9%** เมื่อไม่นับ deferred, ภาษา 30 → 46/95 (มิติอื่นเท่าเดิม); หลัง `a128314` มี 62/95 (ha ky tt uz lb gd mi rm เพิ่ม)
- `24dcde5`: ตัดบรรทัดหลัง `” ` ก่อน `(` (Unicode 15.1 ตัด LB15 แต่ `unicode-linebreak` 0.1.5 ยังใช้) แก้ zenodo-025 (Linux: pages 172 → 173, breaks 91 → 92 ไม่มีเอกสารแย่ลง) และเพิ่มอีก 8 ภาษา (kk mk ms mt nn pt-pt cy sw → 54/95) สั่ง Windows run ถัดไปแล้ว
- กำลังแปล (subagent): ha ky tt uz lb gd mi rm; ที่เหลือซึ่ง Inter วาดได้: ig xh zu rw nso tn wo yo sr-cyrl-ba (ekavian/ijekavian?) ca-valencia; ภาษาที่ต้องฟอนต์/shaping ใหม่ (ar he fa ur ps hi mr ne kok as bn-BD bn-IN gu pa or ta te kn ml si hy ka ko) ยังไม่เริ่ม — ต้องให้ UI วาดได้ก่อน
- เครื่องมือแปลอยู่ใน scratchpad ของ session (หายเมื่อ session จบ): `BRIEF.md`, `check_catalog.py` (ตรวจคีย์ตรง `en-gb.tsv`, placeholder, "…", glyph ใน Inter), `register.py` (เพิ่มใน `LANGUAGES`, `WHOLE_CATALOGS`, `languages.tsv` และแก้ความยาว array)
- วิธีวัด pagination บน Linux: แก้ `take(40)` ใน `xtask/src/readiness.rs` เป็นค่าใหญ่ชั่วคราว (อย่า commit) แล้ว `cargo xtask readiness --corpus --jobs 8 --cli <cli ที่ build ใน CARGO_TARGET_DIR=target-fix>` แล้ว `git checkout docs/readiness.md`; baseline บน Linux ที่ `f65cff1`: pages 172, breaks 91

### Readiness ล่าสุด (บันทึก 2026-11-03 รอบที่ 22, ยืนยันด้วย run 41608625168): **87.7%**, **90.3%** เมื่อไม่นับ 26 แถว deferred

- **วัดซ้ำ 2026-11-03 (run 41608625168, `windows-latest`, วัด `main` ที่ `3418736`, commit ผล `51dab2f`, job 09:08–09:25 UTC)**: ได้ **87.7%** / 90.3% เท่าเดิมทุกมิติ (ตารางด้านล่างยังถูกต้อง); หัวรายงานเป็น `3418736` **ไม่มี `+changes` แล้ว** ยืนยันการแก้ใน `83b1e0c`; ดาวน์โหลดคลังครบในรอบนี้ (zenodo กลับมาตอบปกติ)
- ที่มาเดิม: `docs/readiness.md` จาก Windows run 41596836985 (`windows-latest`, วัด `main` ที่ `24ad50b`, commit ผล `bc99ca1`; คืนไฟล์นี้ใน `83b1e0c` หลัง run 41599589867 commit ผลที่ไม่ครบ 60.8% ทับ) ห่างเป้าหมาย 100% อยู่ 12.3 จุด
- รายมิติ (คะแนน / ไม่นับ deferred / ชุดตรวจ):

| มิติ | น้ำหนัก | คะแนน | ไม่นับ deferred | ชุดตรวจ |
|---|---|---|---|---|
| Features | 0.30 | 94.7% | 97.2% | ribbon 499/499; dialogs 204/228 (deferred 12) |
| File formats | 0.20 | 100.0% | 100.0% | corpus-open 222/222; corpus-roundtrip 222/222 |
| Layout and pagination | 0.15 | 60.1% | 60.1% | corpus-pages 171/222; corpus-breaks 96/222 |
| UI/UX | 0.10 | 95.7% | 98.2% | shortcuts 268/280 (deferred 7) |
| Stability | 0.10 | 100.0% | 100.0% | corpus-stability 222/222; stability 5/5 |
| Performance | 0.05 | 100.0% | 100.0% | performance 6/6 |
| Localization | 0.04 | 31.6% | 31.6% | languages 30/95 |
| Platforms | 0.03 | 80.0% | 100.0% | platforms 4/5 (deferred 1) |
| Ecosystem | 0.02 | 83.3% | 100.0% | ecosystem 5/6 (deferred 1) |
| Hardware | 0.01 | 37.5% | 100.0% | hardware 3/8 (deferred 5) |

- ที่ยังไม่ผ่านและไม่ deferred: dialogs 12 แถว (รอเจ้าของตัดสินว่าจะ defer หรือไม่ ดู "ข้อค้นพบที่ต้องให้เจ้าของ repo ตัดสิน"), shortcuts 5 รายการ (`ks-165`, `ks-177`, `ks-203`, `ks-219`, `ks-329`), corpus-pages 51 และ corpus-breaks 126 เอกสาร, ภาษา 65 ภาษา
- แก้ข้อมูล: deferred 26 แถวแบ่งเป็น dialogs 12, shortcuts **7**, ecosystem 1, hardware 5, platforms 1 (บรรทัดเดิมด้านล่างที่เขียน shortcuts 6 รวมได้ 25 ไม่ตรง)
- ป้าย `24ad50b+changes` ของรายงานก่อนหน้ามาจากขั้น `--download` ที่เขียน `docs/readiness.md` ก่อนขั้นวัด ไม่ได้แปลว่าวัดจากโค้ดที่ไม่ตรง commit (แก้ใน `83b1e0c` และยืนยันแล้วด้วย run 41608625168)

ผู้ใช้สั่ง (2026-11-02): "อ่าน handoff.md แล้วดำเนินการต่อ" (งานที่ค้างคือการออก v1.0.0, v1.1.0, v1.1.1 แล้วจึง v1.1.2 ของรอบที่ 19) ระหว่างทำ ผู้ใช้เลือกให้ออก Release ใน fork และเปิด Actions ของ fork เอง; หลังออกรุ่นผู้ใช้สั่งไม่เปลี่ยนลิงก์ Releases ใน README แล้วสั่ง "ทำงานต่อจากงานต่อไปใน handoff"

### ประวัติผลวัด Windows รอบที่ 21–22 (2026-11-03; เริ่มที่ **86.7%** จาก run 41565289539 ที่ `8b685b2` ยืนยันตัวเลขของ run 41559739417; ล่าสุดอยู่ในหัวข้อด้านบน)

- run 41596836985 (เช็กเอาต์ `main` ตอนเริ่ม job คือ `24ad50b` ซึ่งรวม `f5f32df` Web Options `dlg-232` และ `4c69617` Hangul Hanja `dlg-194`; `head_sha` ของ run เป็น `3a0372e` แต่ workflow ใช้ `ref: main` จึงวัด `main` ณ เวลาที่ job เริ่ม ไม่ใช่ commit ที่สั่ง): **87.7%**, **90.3%** เมื่อไม่นับ deferred, ฟีเจอร์ 94.7%, dialogs 202 → 204/228; คลังเท่าเดิม (corpus-pages 171, corpus-breaks 96, open/round-trip 222/222); commit ผล `bc99ca1`
- รอบที่ 22 (2026-11-03, session ต่อจาก handoff): `f05eeb3` (1) xtask พิมพ์รายการ `git status --porcelain` ลง stderr เมื่อหัวรายงานเป็น `+changes` เพื่อหาสาเหตุที่ทุกรายงานจาก Windows runner เป็น `+changes` (ตรวจในโค้ดแล้วไม่พบตัวเขียนไฟล์ที่ git ติดตาม: xtask เขียนแค่ `docs/readiness.md` หลังอ่านสถานะ, build script เขียนลง `OUT_DIR`, ไม่มี symlink/CRLF ใน index/ชื่อไฟล์ที่ Windows ใช้ไม่ได้, `cargo metadata --locked` ผ่าน; log ของ runner มีแค่คำเตือน LF→CRLF ของ `docs/readiness.md` ตอน commit) (2) `cargo fmt --all` ของโค้ดรอบก่อนที่ทำให้ `cargo fmt --all --check` ไม่ผ่าน (CI ไม่ได้ตรวจ fmt); สั่ง run 41599589867 ต่อคิวไว้ (วัด `main` หลัง `f05eeb3`) ผล run 41599589867: **60.8% ไม่ใช่ผลจริง** — zenodo.org ตอบ 503 ไป 344 ไฟล์แต่ `--download` คืนสำเร็จ จึงวัดแค่ 50 เอกสาร และ bot commit `77fd3b8` ทับ; log ยืนยันว่า `+changes` มาจาก `docs/readiness.md` ที่ขั้น `--download` เขียนไว้ก่อน; แก้ในรอบนี้ (ดู `CHANGELOG.md`): `--download` ล้มเมื่อไม่ครบ (workflow จึงลองซ้ำ/ล้มก่อน commit), สั่งเดี่ยวไม่เขียนรายงาน, การตรวจ `+changes` ไม่นับรายงานเอง, และคืน `docs/readiness.md` เป็นผลของ `bc99ca1` (87.7%); ตัวเลขปัจจุบันจึงยังเป็น 87.7%; รอบถัดไปที่สั่ง workflow ควรได้หัวรายงานไม่มี `+changes`; ยังไม่ได้แก้ `.github/workflows/readiness-windows.yml` ให้ปฏิเสธการ commit รายงานที่มี "Not final" (เป็นชั้นป้องกันเพิ่ม ไม่จำเป็นหลังแก้ `--download`)
- run 41594385545 (`d37e836`, รวม `bc4fe56` Edit Links to Files `dlg-021`): **87.6%**, 90.1% เมื่อไม่นับ deferred, ฟีเจอร์ 94.3%, dialogs 201 → 202/228; คลังเท่าเดิม (corpus-pages 171, corpus-breaks 96, open/round-trip 222/222); รอวัดใน run 41596836985: `f5f32df` (Web Options, `dlg-232`, เปลี่ยนการเขียน .docx: เพิ่ม `webSettings.xml`) และ `4c69617` (Hangul Hanja Conversion, `dlg-194`) (บน Linux dialogs 204/228)
- run 41592240746 (`40b5402`, รวม `dd16ed6` Copy File/Find Files `dlg-007`/`dlg-032`/`dlg-152` และ `0a41629` Letter Wizard `dlg-115`): **87.5%**, **90.1%** เมื่อไม่นับ deferred, ฟีเจอร์ 94.1%, dialogs 197 → 201/228; คลังเท่าเดิม (corpus-pages 171, corpus-breaks 96, open/round-trip 222/222)
- run 41590861975 (`bdfba2d`, รวม `7da7541` Show Repairs `dlg-153`): **87.3%**, 89.8% เมื่อไม่นับ deferred, ฟีเจอร์ 93.2%, dialogs 196 → 197/228; คลังเท่าเดิม (corpus-pages 171, corpus-breaks 96)
- run 41589001124 (`d40ff9b`, รวม `ddb1c52` File locations `dlg-209`): **87.2%**, 89.7% เมื่อไม่นับ deferred, ฟีเจอร์ 93.0%, dialogs 195 → 196/228; คลังเท่าเดิม (corpus-pages 171, corpus-breaks 96)
- run 41586915428 (`895d0cc`, รวม `8b92537` Bidirectional text `dlg-205` และ `5d6add8` Manage Fields `dlg-200`): **87.1%**, 89.6% เมื่อไม่นับ deferred, ฟีเจอร์ 92.8%, dialogs 193 → 195/228; คลังเท่าเดิม (corpus-pages 171, corpus-breaks 96)
- run 41585979571 (`5914683`, รวม `1ff4c31` Japanese Consistency Checker, `dlg-003`): **87.0%**, 89.5% เมื่อไม่นับ deferred, ฟีเจอร์ 92.3%, dialogs 192 → 193/228; คลังเท่าเดิม (corpus-pages 171, corpus-breaks 96, round-trip 222/222)
- run 41584013643 (`b3bf68f`, Japanese Find: Sounds like (Japanese) และ Find Options (Japanese), `dlg-210`): **86.9%**, 89.4% เมื่อไม่นับ deferred, ฟีเจอร์ 92.1%, dialogs 191 → 192/228; คลังเท่าเดิม (corpus-pages 171, corpus-breaks 96, round-trip 222/222)
- run 41582181218 (`0ebfb7d`, Options › Typography, `dlg-218`): **86.9%**, 89.4% เมื่อไม่นับ deferred, ฟีเจอร์ 91.9%, dialogs 190 → 191/228; คลังเท่าเดิม
- run 41579621418 (`293f915`, Fit Text และ Two Lines in One): **86.8%**, 89.3% เมื่อไม่นับ deferred, dialogs 188 → 190/228; คลังเท่าเดิม (corpus-pages 171, corpus-breaks 96, round-trip 222/222); `0ebfb7d` (Options › Typography, `dlg-218`) push แล้ว รอผลวัด Windows run ถัดไป (บน Linux dialogs 191/228)
- งานรอบนี้ (Opus): `e184943` freeform ในกลุ่มรูปร่าง, `293f915` Asian Layout (Fit Text `dlg-047`, Two Lines in One `dlg-229`; Horizontal in Vertical `dlg-083` เก็บค่าได้แต่ยังไม่วาด เพราะ layout ไม่รู้ทิศทางข้อความ), `0ebfb7d` Typography (`dlg-218`) ทั้งหมดมีเทสต์และคำแปล 28 ภาษา รายละเอียดใน `CHANGELOG.md`
- หมายเหตุ build ใน session: ดิสก์ของ session เต็มได้จาก `target/debug/incremental` และไฟล์ทดสอบซ้ำหลายรุ่น ให้ใช้ `CARGO_INCREMENTAL=0` และลบ `target/debug/incremental` เมื่อเต็ม
- run 41569447798 (`e184943`, หลังแก้ freeform ในกลุ่มรูปร่าง) วัดได้ 86.7% เท่าเดิม (corpus-pages 171/222, corpus-breaks 96/222) เพราะการแก้เปลี่ยนการวาด ไม่ใช่การจัดหน้า
- รายการ deferred ทั้ง 26 แถวพร้อมเหตุผลอยู่ใน `docs/readiness.md` หัวข้อ `## Deferred rows (26)` (dialogs 12, shortcuts 6, ecosystem 1, hardware 5, platforms 1)
- run 41559739417 ล้มครั้งแรกที่ขั้น poppler (feed ของ Chocolatey ตอบ 406) แก้ด้วยการลองซ้ำ `95e46b1` แล้วผ่าน

- ที่มา: workflow `.github/workflows/readiness-windows.yml` (สั่งเองจากแท็บ Actions ด้วย workflow_dispatch) รัน `cargo xtask readiness --download` (ลองซ้ำได้ 3 ครั้ง) แล้ว `--corpus --jobs 4`; run สำเร็จ ใช้เวลาราว 22 นาที; commit ผล `cab15d4` บน `main` (`docs/readiness.md`)
- คะแนน: 86.7% รวม, 89.2% เมื่อไม่นับ 26 deferred (เดิม 86.0% และ 88.5%); ฟีเจอร์ 91.2% (dialogs 188/228), การจัดหน้า 60.1% (corpus-pages 171/222, corpus-breaks 96/222), UI/UX 95.7%, ภาษา 31.6% (30/95)
- ข้อสังเกต: หัวรายงานเขียน `at commit b6d3056+changes` ซึ่งหมายความว่า xtask เห็นต้นไม้งานไม่สะอาดตอนวัด (น่าจะเป็นไฟล์ที่ดาวน์โหลด/สร้างใน runner) ยังไม่ได้ตรวจว่าเกิดจากอะไร จึงควรดูก่อนถือว่าตัวเลขสะอาด
- ROADMAP.md และ docs/roadmap.md อัปเดตตามตัวเลขนี้แล้ว

### รอบที่ 21 (2026-11-03, Haiku): govuk-005 ตรวจแล้ว ยังไม่แก้ส่วนที่เหลือ

- ผลของ `a351c88` (ฟิลด์ตัดบรรทัดได้) ทำให้ govuk-005 จาก 40 หน้าเหลือ 37 หน้า (Word 36) จากการวัดก่อน/หลังบน Linux; หน้า 1–12 ของผลเราตรงกับ PDF อ้างอิงทั้งลำดับหัวข้อและหน้า
- ส่วนที่เหลือ: หน้า 12 ของผลเราไม่ตรงกับอ้างอิงในส่วน 11B (Section 11B "Monetary assets abroad") ข้อความ "Provide details here" กับ "Yes ☐ No ☐" ซ้อนทับกันบางแถว และมีบรรทัด "Provide details here" ตกค้างสามบรรทัดขึ้นหน้า 13 ที่อ้างอิงไม่มี
- สาเหตุที่ตรวจแล้ว: ย่อหน้าเหล่านี้เป็นข้อความธรรมดา ไม่มี FORMTEXT/ffData/sdt ในไฟล์ จึงไม่เกี่ยวกับการแก้ฟิลด์; ตารางของส่วน 11B ตามด้วยย่อหน้าหัวข้อที่มี `wp:anchor` เป็น `wpg:wgp` (กลุ่มรูปร่าง, `wrapNone`, `positionV` ติดย่อหน้า) ซึ่งวาดเส้นกรอบทับตาราง; การซ้อนทับน่าจะมาจากการวางกลุ่มรูปร่างนี้ ยังไม่ได้พิสูจน์
- ข้อค้นพบเพิ่มเติม (รอบที่ 21, หลังวัด Windows): ใน govuk-005 ข้อความ "Provide details here" ของส่วน 11B เป็น **กล่องข้อความลอย** (`wps` ใน `wp:anchor`, `wrapTopAndBottom`, `positionV` ติดย่อหน้า) สามกล่องเรียงกันที่ x≈4003675 EMU ห่างกันประมาณ 548k EMU สูงกล่องละ 346075 EMU; กล่องแบบนี้ควรดันบรรทัดลงตามที่ Word ทำ แต่ผลของเรายังซ้อนทับ ทำให้หน้า 12 แน่นกว่า Word จนมีบรรทัดตกไปหน้า 13 งานต่อไปคือเปรียบเทียบ y ของกล่องเหล่านี้กับ PDF อ้างอิง (`pdftotext -bbox`) แล้วดูว่า `wrapTopAndBottom` ของ anchor ที่ติดย่อหน้าในเซลล์ตารางถูกคิดความสูงถูกหรือไม่
- ตำแหน่งที่วัดด้วย `pdftotext -bbox` (รอบที่ 21, ผลของเราบน Linux เทียบ PDF อ้างอิง, หน่วย pt): หน้า 1 ของเราเริ่มส่วน 11B ที่ y≈491 ส่วนอ้างอิง y≈477 (ต่าง 14 pt ตั้งแต่ก่อนหน้านั้น); แถว "Do you have a PayPal account" ของเราอยู่ y≈591 อ้างอิง y≈561 และ "Yes ☐ No ☐" ของเราอยู่ที่ x≈322/389 (คอลัมน์ Provide) ขณะที่อ้างอิงอยู่ x≈192/258 (คอลัมน์ Yes/No) จึงซ้อนกับ "Provide details here" ที่ x≈319; กลุ่มรูปร่าง `wpg` ที่ตำแหน่ง x≈192.6 pt (`positionH` 2446020 EMU) ตรงกับคอลัมน์ Yes ของอ้างอิงพอดี ควรตรวจว่า `wpg` นี้วาดหรือบังคับให้เซลล์ของแถวนี้ย้ายคอลัมน์หรือไม่ ก่อนแก้ `wrapTopAndBottom`
- แก้แล้วใน `e184943`: สมาชิกกลุ่มรูปร่าง (`wpg`) ที่มี `a:custGeom` ถูกอ่านเป็น freeform (เดิมวาดเป็นกรอบสี่เหลี่ยมเต็ม bounding box สูงเกือบทั้งหน้า พร้อมเส้นแนวตั้งพาดทับตาราง ซึ่งเป็นสาเหตุของการซ้อนทับใน govuk-005 หน้า 12); จำนวนหน้าไม่เปลี่ยน (37, Word 36) เพราะเป็นการวาด ไม่ใช่การจัดหน้า
- ส่วนต่างการจัดหน้าที่เหลือของ govuk-005 (วัดจาก `pdftotext -bbox` หน้า 12): ป้ายซ้ายตรงกับอ้างอิงถึง "Investments?" (y≈208.5) แล้วเลื่อนลง +4.4 pt ที่ตาราง Currency/Value, +1 pt ในตาราง และ +8.4 pt ระหว่างตารางกับป้าย "You must provide" (รวม +13.4 pt) +8.4 pt ใกล้เคียงหนึ่งบรรทัดว่างของฟอนต์ 7 pt (8.05 pt) พอดี: หลังตารางมีย่อหน้าว่างสองอัน (อันที่สองถือ `sectPr` แบบ `continuous` ฟอนต์ 7 pt) แล้วจึงเป็นป้าย ขณะที่ช่องว่างในอ้างอิงมีหนึ่งบรรทัดเท่านั้น; ก่อนตารางมีย่อหน้าว่างสองอัน (10 pt) และย่อหน้าถือ `sectPr` แบบสามคอลัมน์หนึ่งอัน สมมติฐานที่ยังไม่พิสูจน์: Word ไม่ให้ความสูงแก่ย่อหน้าว่างที่ถือ `sectPr` แบบ `continuous` (หรือการ balance คอลัมน์ของ section ต่อเนื่องทำให้ต่าง) ต้องหาตัวอย่างเทียบในคลังก่อนแก้ แล้ววัดว่าไม่มีไฟล์แย่ลง
- ยังไม่มี commit สำหรับการจัดหน้าส่วนนี้ (ไม่ได้เดาแก้โดยไม่มีหลักฐาน); งานต่อไปคือยืนยันว่า wpg anchor ทำให้ย่อหน้าแถวหลังตารางขยับ/ซ้อน แล้วแก้ที่ layout ของ anchor นั้น และวัด govuk-005 ซ้ำ

### เจ้าของเขียนทับ `main` ของ fork ระหว่างรอบ (พบ 13:02 UTC)

- `main` ของ fork ถูก force-update จาก `daa522a` เป็น `4554ea9` หลัง 11:48 UTC: `4554ea9` คือ `3e00170` ที่ `chatraweechai` (committer, 11:29 UTC) นำไปไว้ใน repo ต้นทาง โดยเนื้อไฟล์เหมือน `3e00170` ทุกไฟล์ และต้นทางมี tag `v1.1.2` → `4554ea9` (`main` ของต้นทางก็อยู่ที่ `4554ea9`) ส่วน tag `v1.1.2` ของ fork ยังชี้ `3e00170`
- commit ที่หายจาก `main` ของ fork (`473e0da`, `be45dad`, `daa522a`) และงานใหม่ ถูก rebase ต่อบน `4554ea9` โดยไม่มีไฟล์ชน แล้ว push แบบ fast-forward (ไม่ได้ force) จึงได้ hash ใหม่: `473e0da` → `8d21a41`, `be45dad` → `effd83f`, `daa522a` → `6d9bf34`; ถ้าเจ้าของตั้งใจให้ `main` ของ fork ตามต้นทางเท่านั้น ต้องตัดสินว่าจะเอา commit เหล่านี้เข้าต้นทางหรือไม่

### ผลการออกรุ่น (ตรวจด้วย GitHub API และหน้า Releases)

| รุ่น | commit ที่ build (tag) | run | ไฟล์แนบ | ป้าย |
|---|---|---|---|---|
| v1.0.0 | `7946cbb` (annotated tag ของเจ้าของ) | 41430786475 | `doc4me-1.0.0-windows-x64.msi` 30,146,560 B, `-portable.zip` 40,242,969 B | — |
| v1.1.0 | `51a9658` (annotated tag ของเจ้าของ) | 41430819045 | msi 33,005,568 B, zip 43,571,345 B | — |
| v1.1.1 | `be5893e` (annotated tag ของเจ้าของ) | 41430823465 | msi 32,997,376 B, zip 43,584,023 B | — |
| v1.1.2 | `3e00170` (HEAD ของ `main` ตอนสั่ง; job `release` สร้าง tag เอง; โค้ดแอปเท่ากับ `861e994`) | 41433220035 | msi 36.9 MB, zip 50 MB | Latest |

- ทุกไฟล์ยังไม่ลงนาม (ไม่มี secret ลงนาม) บันทึกของ Release มาจากหัวข้อของรุ่นนั้นใน `CHANGELOG.md` ของ `main` (v1.1.2 ย่อเหลือบรรทัดแรกของแต่ละรายการพร้อมลิงก์ไปหัวข้อเต็มใน fork เพราะยาวเกิน 125,000 ตัวอักษร)
- วิธี: รุ่นเก่าใช้ทางสำรองของ workflow (`run_workflow` บน `main` inputs `release: true`, `commit: <sha>`) สามรุ่นพร้อมกัน job `build` ราว 16 นาทีต่อรุ่น (fork ยังไม่มี cache) job `release` เห็น tag ที่ชี้ commit เดียวกันจึงข้ามการสร้าง tag แล้ว `gh release create --verify-tag --latest=false` สำเร็จทั้งสาม (ข้อจำกัด workflow scope ไม่ติดเพราะ commit มี tag อยู่แล้ว); จากนั้นจึงสั่ง v1.1.2 (`release: true`) ซึ่ง build แบบปกติ (`-BuildOnly` แล้ว `-SkipBuild`) และสร้าง Release พร้อม tag ที่ HEAD
- ก่อนมี v1.1.2 `get_latest_release` ตอบ v1.1.1 แม้สร้างด้วย `--latest=false` (API เลือกตาม `created_at` เมื่อไม่มีรุ่นใดถูกตั้ง Latest); หลังออก v1.1.2 หน้า Releases แสดงป้าย Latest ที่ v1.1.2 รุ่นเดียว
- `chatraweechai/doc4me` ยังไม่มี Release (มี tag `v1.1.2` → `4554ea9` แล้ว ดูหัวข้อด้านบน) ส่วน `main` ของ fork นำหน้าต้นทางด้วยงานรอบนี้ (ต้นทางอ่านได้แต่ push ไม่ได้จาก session นี้)

### สิ่งที่พบตอนเริ่มรอบ (ตรวจ 2026-11-02 ราว 10:00–10:30 UTC)

- session นี้เชื่อมกับ `veevie55/doc4me` ซึ่งเป็น fork (public) ของ `chatraweechai/doc4me`; บัญชี GitHub ของ session คือ `veevie55` (สร้าง 2026-11-02 09:44 UTC) จึงไม่มีสิทธิ์ API กับ repo ต้นทาง อ่านได้เฉพาะ git (`git ls-remote`) และหน้าเว็บสาธารณะ
- เจ้าของ repo push tag ของรุ่นเก่าแล้ว (annotated, tagger `envburiram`, 09:27–09:28 UTC) ชี้ commit ถูกต้องทั้งสาม: `v1.0.0` (`f6055dd`) → `7946cbb57a0f4618711e765a844f46194150ec1f`, `v1.1.0` (`4cfa336`) → `51a9658096d7f5f8b2ea6ed00acd49b5db463a47`, `v1.1.1` (`091a436`) → `be5893e7ab2fe40285205e976ad366bd1d27d50d` (ตรวจด้วย `git ls-remote` ทั้งต้นทางและ fork และ `get_tag`; fork ได้ tag ชุดเดียวกันมาตอน fork)
- **สาเหตุที่ Actions ไม่จ่าย runner (ปัญหาของรอบที่ 19)**: run ของ tag ใน repo ต้นทาง #154 (41420985373, `v1.0.0`), #155 (41421019066, `v1.1.0`), #156 (41421048403, `v1.1.1`) ล้มภายใน 4–11 วินาที และ annotation ของ job `build` ทั้งสาม run (อ่านจากหน้า run สาธารณะด้วย WebFetch) คือ "The job was not started because your account is locked due to a billing issue." คือบัญชี `chatraweechai` ถูกล็อกเพราะปัญหาการชำระเงิน ไม่ใช่ไฟล์ workflow หรือสิทธิ์ของ action; push build #153 (41413504791) ของ `main` มีข้อความเดียวกัน (#151–#152 ล้มภายในไม่กี่วินาทีเหมือนกัน แต่ไม่ได้เปิดอ่าน)
- fork ตอนเริ่ม: ไม่มี Release (Release ไม่ติดมากับการ fork), `list_workflows` ได้ 0 และ `run_workflow` ได้ HTTP 404 เพราะ GitHub ปิด workflow ของ fork ไว้จนกว่าเจ้าของจะกดเปิดในแท็บ Actions (เจ้าของเปิดแล้วราว 10:40 UTC)

### งานรอบนี้ (push ไป `main` แล้ว รายละเอียดและผลตรวจอยู่ใน `CHANGELOG.md` `[Unreleased]`)

- `3e00170` (บน `main` ตอนนี้คือ `4554ea9`) handoff: สิ่งที่พบตอนเริ่มรอบ (ก่อนออกรุ่น)
- `8d21a41` (เดิม `473e0da`) แก้หน้าเปล่าหลังตัวแบ่งหน้า (`crates/layout/src/lib.rs`, เทสต์ `a_page_break_before_an_empty_section_break_paragraph_leaves_no_blank_page`): ย่อหน้าที่จบด้วยตัวแบ่งหน้าแล้วตามด้วยย่อหน้าว่างที่ปิด section ซึ่ง section ถัดไปขึ้นหน้าใหม่ ไม่สร้างหน้าเปล่าแล้ว (ข้อ "zenodo-160" ในงานต่อไปของรอบที่ 18) ทำและ push หลัง v1.1.2 ออกแล้ว Release v1.1.2 จึงไม่มีโค้ดนี้
- วัดบน Linux ด้วย `cargo xtask readiness --corpus --jobs 4` เทียบ CLI ก่อน/หลังแก้ (ให้รายงานแสดงไฟล์ไม่ผ่านครบชั่วคราว): จำนวนหน้าตรง 166 → 168, ต้น/ท้ายหน้าตรง 89 → 90, ไม่มีไฟล์ที่ผลแย่ลง (zenodo-169 ตรงทั้งสองแบบ, zenodo-160 จำนวนหน้าตรง, zenodo-053 64 → 61 จาก Word 58, zenodo-136 27 → 26 จาก Word 25); govuk-037 มีย่อหน้าแบบเดียวกันแต่ section ถัดไปเป็นแบบต่อเนื่องจึงไม่เปลี่ยน; คลังทั้ง 222 ไฟล์ดาวน์โหลดได้ครบ (zenodo-078.pdf และ zenodo-161.docx ล้มครั้งแรก ดาวน์โหลดซ้ำแล้ว SHA-256 ตรง); ตรวจก่อน push: `cargo test -p doc4me-layout` 240, `-p doc4me-engine` 451 + 13 + 8 ผ่าน, `cargo fmt --all --check` และ `cargo clippy --workspace --all-targets -- -D warnings` ผ่าน (บน Linux ต้อง `apt-get install libasound2-dev` ก่อน ไม่งั้น build script ของ `alsa-sys` ล้ม)
- push ของ `473e0da` เป็น push build แรกของ `main` ใน fork (ไม่ออก Release): run 41437764535 ผ่าน (build และทำ MSI/zip บน Windows ราว 12 นาที ใช้ cache ที่ run ของ v1.1.2 บันทึกไว้)
- `effd83f` (เดิม `be45dad`) และ `6d9bf34` (เดิม `daa522a`): handoff, `ROADMAP.md`, `docs/roadmap.md` หลังออกรุ่น และการตัดสินใจเรื่องลิงก์ README

### งานต่อด้าน pagination (สั่ง "ทำงานต่อจากงานต่อไปใน handoff"; วัดบน Linux เครื่องเดียวกัน แสดงไฟล์ไม่ผ่านครบชั่วคราว)

- `zenodo-023` (งานต่อไปของรอบที่ 18): ไม่ใช่บั๊ก — DOCX สร้างเมื่อ 2026-07-22 ด้วยตัวแปลงไฟล์ (core.xml วันสร้างเท่าวันแก้ไข, AppVersion 12.0000, ไม่มีจำนวนหน้า) ส่วน PDF ที่ Word สร้างลงวันที่ 2022-06-14 จึงเป็นคนละรุ่น; บนหน้า 52 ของ PDF ข้อความ header1 และเลข "1" ถูกรูปสีเขียวทั้งหน้าทับไว้ (pdftotext ยังอ่านได้) แต่ section สุดท้ายของ DOCX อ้าง header2/footer2 ที่ว่าง; ไม่ได้แก้ manifest หรือ comparator
- `bddb167` ตาราง: `w:trPr/w:hidden` และ `w:hideMark` ไม่หายตอนบันทึกแล้ว (เดิมทิ้งไป ไฟล์ที่บันทึกด้วย Doc4Me จึงแสดงแถวที่ซ่อนใน Word) แถวที่เครื่องหมายท้ายแถวและเนื้อหาทั้งหมดซ่อนไม่กินที่ และย่อหน้าว่างท้ายเซลล์ `hideMark` ไม่นับในความสูงแถว: govuk-066 9 → 6 หน้าตรงทั้งสองแบบ, govuk-068 17 → 16 (Word 15); จำนวนหน้าตรง 168 → 169, ต้น/ท้ายหน้าตรง 90 → 91 (`hideMark` ใช้ใน 36 ไฟล์ แถวซ่อนมีแค่ govuk-066)
- `3593d05` ตัวแบ่งหน้าท้ายย่อหน้าที่ปิด section เองไม่สร้างหน้าเปล่า และบรรทัดที่มีแค่ตัวแบ่งหน้ากับรูปลอยหรือ marker คงอยู่ท้ายหน้า: zenodo-029 18 → 17 และ zenodo-140 16 → 15 (เท่า Word แต่ยังต่างที่หน้า 2), zenodo-149 42 → 41, zenodo-053 61 → 60; จำนวนหน้าตรง 169 → 171, ต้น/ท้ายหน้าคงที่ 91
- รวมรอบนี้บน Linux: จำนวนหน้าตรง 166 → 171, ต้น/ท้ายหน้าตรง 89 → 91, ไม่มีไฟล์ที่ผลแย่ลง; ตรวจก่อน push ทุกครั้ง: เทสต์ layout (ล่าสุด 245), docx 75 + 49, docbin 65, doc 118, engine 451 + 13 + 8, ui-egui `--lib` 396, `cargo fmt --all --check`, `cargo clippy` (workspace สำหรับ `bddb167`, crate layout สำหรับ `3593d05`)
- `a351c88` ผลลัพธ์ของฟิลด์ (FORMTEXT, FILLIN) ตัดบรรทัดตามช่องว่าง: govuk-024 3 → 2 หน้า (Word 2) แก้แล้ว (ต้น/ท้ายหน้า 2 ยังต่าง), จำนวนหน้าตรง 171 → 172 บน Linux; zenodo-115 ได้ 10 หน้า (Word 9) ทั้งก่อนและหลังแก้ (ไม่เกี่ยวกับงานนี้ ยังไม่ได้สอบสวน); `cargo xtask readiness --corpus` เขียนทับ `docs/readiness.md` ต้อง `git checkout docs/readiness.md` ก่อน commit (ไฟล์ที่ commit คือผลวัดบน Windows)
- (แก้แล้วใน `a351c88`; ข้อความต่อไปนี้เป็นบันทึกก่อนแก้) พบแต่ยังไม่แก้: **govuk-024** (3 หน้า Word 2 ทั้งบน Windows และ Linux) ฟิลด์ฟอร์ม FORMTEXT ที่ผลลัพธ์ยาวถูกจัดเป็น cluster เดียว (`shape_atomic` ใน `crates/layout/src/para.rs`) ตัดบรรทัดไม่ได้ ความกว้างขั้นต่ำของเซลล์จึงเป็นทั้งประโยค (499 และ 1,306 pt) autofit ขยายคอลัมน์ทั้งหมดจนรวม 1,385 pt แล้วย่อทั้งตารางเหลือ 538 pt คอลัมน์หมายเหตุซ้ายจึงแคบจนตัดทีละพยางค์ ตารางสูงจนล้นหน้า; ใน Word ผลลัพธ์ของ FORMTEXT ตัดบรรทัดได้แบบข้อความปกติ ทางแก้คือแตก cluster ของฟิลด์ฟอร์มเป็นคำที่ตัดบรรทัดได้ (ต้องดูการวาง caret การเลือก และข้อความที่ส่งออก PDF) ในคลังมีไฟล์นี้ไฟล์เดียว; **zenodo-149** ยังเกิน 13 หน้า หน้าปกล้นเพราะข้อความในกล่องสัญญาอนุญาตตัดบรรทัดมากกว่า Word (ความกว้างของฟอนต์บน Linux ต้องดูบน Windows); **zenodo-156** หน้า 1 ของ Word จุคำมากกว่าเกือบสองเท่า (น่าจะเป็นฟอนต์)

### ข้อค้นพบที่ต้องให้เจ้าของ repo ตัดสิน

- **บ้านของโปรเจกต์ (ตัดสินแล้วสำหรับ README)**: ผู้ใช้สั่ง (2026-11-02) ไม่เปลี่ยนลิงก์ Releases ใน README จึงคงชี้ `chatraweechai/doc4me` แม้ Release อยู่ที่ https://github.com/veevie55/doc4me/releases (บันทึกไว้ในการตัดสินใจถาวรด้านล่าง); ลิงก์อื่นที่ชี้ `chatraweechai/doc4me` (หน้า About, `repository` ใน `Cargo.toml`, URL แค็ตตาล็อก Add-in ใน `crates/engine/src/cmd/addins.rs`, `websiteUrl` ของ MCP และลิงก์ท้าย `CHANGELOG.md`) ก็ยังไม่ได้เปลี่ยน (app id `io.github.chatraweechai.doc4me` ไม่ควรเปลี่ยน เพราะเป็นชื่อไฟล์ .desktop ไอคอน และ Flatpak ที่ติดตั้งไว้แล้ว; แค็ตตาล็อก Add-in ใน `raw.githubusercontent.com/chatraweechai/...` ยังเปิดได้เพราะ repo ต้นทางยัง public)
- ถ้าจะให้ต้นทางมี Release ด้วย: แก้การชำระเงินของ `chatraweechai` แล้ว re-run #154–#156 (ใช้ commit และ workflow เดิมของ tag) แล้วออก v1.1.2 ที่นั่น (ต้นทางมี tag `v1.1.2` → `4554ea9` แล้ว ซึ่งเนื้อไฟล์เหมือน `3e00170` ที่ Release v1.1.2 ของ fork build; การ push tag นั้นสร้าง run #158 และ push `main` สร้าง run #157 ซึ่งล้มแบบ startup failure ทั้งคู่เพราะบัญชียังถูกล็อก เมื่อแก้การชำระเงินแล้ว re-run #158 ได้เลย)
- `zenodo-023`: PDF อ้างอิง (Word, 2022-06-14) กับ DOCX (ตัวแปลงไฟล์, 2026-07-22) เป็นคนละรุ่น หน้าสุดท้ายจึงต่างเฉพาะข้อความหัว/ท้ายกระดาษที่ถูกรูปทับ (ดูงานต่อด้าน pagination)
- `zenodo-160` หน้า 8: หน้า PDF อ้างอิงมีข้อความ comment "Commented [SP1]: I updated this table …" ในแถบ markup แต่ไฟล์ .docx ไม่มี `word/comments.xml` แล้ว (คนละรุ่นในส่วนของ comment) ตัวตรวจต้น/ท้ายหน้าจึงไม่ผ่านทั้งที่เนื้อหาในหน้าตรงกัน (ตารางจบที่ "66 60 60 66 62 62" ทั้งสองฝั่ง); ไม่ได้แก้ manifest หรือ comparator ตามกติกา
- ข้อค้นพบของรอบที่ 18 (`govuk-026`, `govuk-018`, `zenodo-104`) ยังรอการตัดสินใจ

- **Dialog ที่ยังไม่ผ่าน 12 แถว (ไม่นับ 12 deferred) — ขอให้เจ้าของตัดสินว่าจะ defer หรือไม่** (2026-11-03): เป็นฟีเจอร์ที่ Word รุ่นปัจจุบัน (Microsoft 365) ตัดออกไปแล้ว — `dlg-081`/`dlg-082` WordPerfect Help, `dlg-151` Schema Library / `dlg-234` XML Element Attributes / `dlg-235` XML Options (custom XML markup ถูกตัดตั้งแต่ Word 2010; Doc4Me อ่านเนื้อหาแต่ทิ้ง markup แบบเดียวกัน), `dlg-215` Smart Tag Options (smart tags ถูกตัด; Doc4Me ทิ้ง markup เช่นกัน), `dlg-079` Frames Page properties และ `dlg-010` CSS Links (หน้าเว็บแบบ frames/HTML รุ่นเก่า), `dlg-052` Format Callout (callout ของ Word 97); ที่ยังมีใน Word แต่ต้องงานใหญ่หรือเครื่องของเจ้าของ — `dlg-083` Horizontal in Vertical (ต้องจัดวางที่รู้ทิศทางข้อความ), `dlg-108` Insert Subdocument (master document), `dlg-041` Print Setup (เลือกเครื่องพิมพ์ของระบบ; Doc4Me ส่ง PDF ให้โปรแกรมดู PDF พิมพ์ จึงต้องรายการเครื่องพิมพ์ของแต่ละระบบ)
- **Shortcut ที่ยังไม่ผ่าน**: `ks-203` (Alt+Shift+F12, Space: ตัวเลือก citation) และ `ks-329` (Alt+Shift+F12: ปุ่ม Table of Contents) ต้องมีปุ่มแท็บของ content control ของ TOC/citation ซึ่ง Doc4Me ยังไม่มี — ไม่ผูกปุ่มกับคำสั่งอื่นแทนเพื่อไม่ให้ผ่านแบบไม่จริง; `ks-219` (Ctrl+Shift+F7 อัปเดตไฟล์ต้นทางของลิงก์) ต้องประเมิน INCLUDETEXT ก่อน; `ks-177` (F9 รีเฟรชหน้าเว็บ) ไม่มีมุมมองเว็บสด; `ks-165` รอเจ้าของตรวจบน Word for Windows

### งานต่อไป (ยังไม่ถึงเป้าหมาย 100%)

- ทำต่อจาก "งานต่อไป" ของรอบที่ 18 ด้านล่าง (ข้อ zenodo-160 เสร็จแล้ว, zenodo-023 เป็นคนละรุ่น): วัดซ้ำบน Windows แล้วสร้าง `docs/readiness.md` ใหม่, pagination (`govuk-005`, govuk-024 ฟิลด์ฟอร์มที่ตัดบรรทัดได้, zenodo-029/zenodo-140 หน้า 2), dialogs, localization
- ห้ามออกเวอร์ชันถัดจาก 1.1.2 จนกว่าผู้ใช้สั่ง; release workflow ใน fork: push ไป `main` ไม่ build installer แล้ว (ผู้ใช้สั่ง 2026-11-03) สั่งจากแท็บ Actions หรือ push tag `v*` เท่านั้น

## สถานะรอบที่ 19 (2026-11-02: ออก v1.0.0, v1.1.0, v1.1.1 และ v1.1.2; ประวัติ เจ้าของ push tag แล้วและพบสาเหตุในรอบที่ 20)

ผู้ใช้สั่ง (2026-11-02): "ออกเวอร์ชันเก่าทั้ง v1.0.0 v1.1.0 v1.1.1 และเวอร์ชันใหม่จากโค้ดล่าสุด v1.1.2" ระหว่างทำสั่งเพิ่มให้ตรวจ tag/draft release อีกครั้ง แล้วให้ตรวจสคริปต์ build ของแต่ละรุ่นเพื่อแก้ workflow ให้ออกรุ่นเก่าจาก commit เดิมได้

### สถานะ: รอเจ้าของ repo แก้ปัญหา Actions ไม่จ่าย runner และ push tag รุ่นเก่า แล้วจึงออก v1.1.2 (ผู้ใช้เลือก 2026-11-02 ให้ออก v1.1.2 หลังรุ่นเก่า เพื่อให้ v1.1.2 เป็น Latest)

- **ปัญหาใหม่ (พบ 2026-11-02 08:25 UTC): GitHub Actions ไม่จ่าย runner ให้ job ใดเลย** เริ่มระหว่าง 03:47 ถึง 07:51 UTC (job ล่าสุดที่ได้ runner คือ attempt 9 ของ run 41379963417 เวลา 03:37–03:47 UTC ซึ่งถูกยกเลิก)
  - job `build` (windows) ของ push build `8f7d2e2` (run 41409013327, 07:51 UTC) และ `ee4e1ee` (run 41409192856, 07:53 UTC และ attempt 2 ที่ session re-run 08:25 UTC) ล้มภายใน 2 วินาที: `runner_id` 0, ไม่มี step, log ได้ HTTP 404
  - job `release` (ubuntu) ของ run 41379967784 ที่ session re-run (attempt 3, 08:28 UTC) ล้มแบบเดียวกัน จึงไม่ได้เป็นเฉพาะ Windows
  - ไม่ได้เกิดจากไฟล์ workflow: ส่วนระดับ job (`runs-on`, `timeout-minutes`, `outputs`) ไม่เปลี่ยนตั้งแต่ run 41379931795 (`861e994`) ที่ผ่าน และ job ubuntu ข้างบนใช้ workflow ของ `861e994` นั้นเอง; repo เป็น public (นาทีของ runner มาตรฐานฟรี); githubstatus.com ทุกระบบ Operational ไม่มี incident
  - ข้อความสาเหตุอยู่ใน annotation ของ run ซึ่งเครื่องมือของ session อ่านไม่ได้ (`get_check_run` ได้ output ว่าง)
  - **[เจ้าของ repo]** เปิด https://github.com/chatraweechai/doc4me/actions/runs/41409192856 อ่านข้อความใต้ "Annotations" แล้วแก้ตามนั้น สาเหตุที่ให้อาการนี้: Settings › Actions › General › Actions permissions ไม่อนุญาต action ที่ใช้ (`actions/checkout`, `actions/upload-artifact`, `actions/download-artifact`, `dtolnay/rust-toolchain`, `Swatinem/rust-cache`) หรือบัญชีถูกล็อกเพราะการชำระเงิน (Settings › Billing and licensing)
  - push tag ได้ก่อนแก้ แต่ run ของ tag จะล้มแบบเดียวกัน หลังแก้แล้ว re-run run นั้นได้เลย ไม่ต้อง push tag ซ้ำ (re-run ใช้ commit และ workflow เดิมของ tag)

- push ไป `main` แล้ว:
  - `861e994` ออกเวอร์ชัน 1.1.2 (`cargo xtask version set 1.1.2`, ย้าย `[Unreleased]` เป็น `## [1.1.2] - 2026-11-02`) และ workflow รับ input `commit` เพื่อ build/ออก Release ของ commit เก่าบน `main` (ทางสำรอง)
  - `8f7d2e2` แก้ workflow หลังรันจริง: tag ของ commit ที่ไฟล์ workflow ต่างจาก `main` ต้อง push จาก clone ก่อน job `build` จึงหยุดตั้งแต่ก่อน build พร้อมบอกคำสั่ง
- รันจริงบน `861e994`: run 41379963417 (1.0.0), 41379965614 (1.1.0), 41379967784 (1.1.1) ตรวจ input/ancestry และ build MSI/zip ผ่านทั้งหมด แต่ "Tag the earlier commit" ได้ `Resource not accessible by integration (HTTP 403)` ทั้งสามรุ่น จึงยังไม่มี Release; push build ของ `main` (run 41379931795) ผ่านด้วยทางปกติ
  - เจ้าของ repo re-run เองหลายครั้ง (attempt 2 ของ 41379965614 ล้มที่ "Choose the tag" เพราะตอนนั้นมี tag `v1.1.0` ที่ `861e994` ซึ่งเกิดจากการ Publish draft แล้วลบภายหลัง; 41379963417 ถึง attempt 9 แล้วถูกยกเลิก)
  - session นี้ลอง push tag `v1.1.1` เองอีกครั้ง ได้ HTTP 403 จาก git proxy เหมือนเดิม (ลบ tag ในเครื่องแล้ว ไม่ลองซ้ำ)
- **[เจ้าของ repo]** รันใน clone ของตัวเอง (ห้าม Publish draft 4 รายการ เพราะ target เป็น `main` จะได้ tag ผิด commit; ลบทิ้งได้):
  - `git fetch origin`
  - `git tag -a v1.0.0 7946cbb57a0f4618711e765a844f46194150ec1f -m "Doc4Me 1.0.0"`
  - `git tag -a v1.1.0 51a9658096d7f5f8b2ea6ed00acd49b5db463a47 -m "Doc4Me 1.1.0"`
  - `git tag -a v1.1.1 be5893e7ab2fe40285205e976ad366bd1d27d50d -m "Doc4Me 1.1.1"`
  - `git push origin v1.0.0`, `git push origin v1.1.0`, `git push origin v1.1.1` (แยกทีละตัว) แต่ละ tag รัน workflow ของ commit นั้นเองซึ่ง build และออก Release พร้อมหัวข้อ CHANGELOG ของรุ่นนั้น
- งานของ session หลังเจ้าของ repo แก้ปัญหา Actions และ push tag:
  0. re-run push build ล่าสุดของ `main` ให้ผ่านก่อน เพื่อยืนยันว่า Actions จ่าย runner แล้ว และ re-run run ของ tag ที่ล้มเพราะปัญหานั้น
  1. รอ run ของ tag ทั้งสามจบ ตรวจว่า Release มี `doc4me-<เวอร์ชัน>-windows-x64.msi` และ `-portable.zip` และ tag ชี้ commit ข้างบน
  2. ถ้า run ของ tag ใดออก Release ไม่ได้ (เช่น workflow ของ `be5893e` ส่ง `--target` ทั้งที่มี tag แล้ว) ให้ใช้ทางสำรอง: re-run failed jobs ของ run 41379967784 (หรือ run ใหม่ `release: true`, `commit: <sha>`) ซึ่งจะเจอ tag ที่ commit เดียวกันแล้วออก Release แบบไม่ย้ายป้าย Latest
  3. แล้วจึงออก v1.1.2: `actions_run_trigger` workflow `windows-x64-installer.yml` ref `main` input `release: true` (ไม่ใส่ `commit`) บันทึกของ Release จะย่อเพราะหัวข้อ `[1.1.2]` ยาวเกิน 125,000 ตัวอักษร; ห้าม push ไป `main` ระหว่างที่ job `release` รัน
  4. ตรวจว่า v1.1.2 เป็น Latest แล้วอัปเดต handoff, `ROADMAP.md` แถว Releases และ `docs/roadmap.md` M14

### สิ่งที่พบก่อนออก (ตรวจ 2026-11-02 ด้วย GitHub API)

- ไม่มี tag เลย (`list_tags` ว่าง, `git ls-remote --tags` ว่าง) และไม่มี Release ที่เผยแพร่: Release v1.1.0 (tag ที่ `51a9658`, ออก 2026-10-21 run #31) และ v1.1.1 (`be5893e`, ออก 2026-10-22 run 39788628618 แล้วออกซ้ำที่ `4f1d407` โดย run 40634619053 เมื่อ 2026-10-28) ไม่มีแล้วทั้ง Release และ tag เมื่อตรวจ (ไม่ทราบว่าใครลบเมื่อไร); v1.0.0 (`7946cbb`) ยังไม่เคยมี tag
- draft ที่เจ้าของ repo สร้างไว้ 4 รายการ (ยังไม่มี tag, body ว่าง): `v1.0.0` "Doc4Me v1.0.0", `v1.1.0` "Doc4Me v1.1.0", `v.1.1.1` "Doc4Me v1.1.1" (ชื่อ tag มีจุดเกินหลัง v) และ `v1.1.2` "Doc4Me v1.1.2"; draft ไม่ขวาง `gh release create` (gh ตรวจเฉพาะ Release ที่เผยแพร่แล้วด้วย HEAD `releases/tags/<tag>`) session จึงไม่ได้แตะ draft เหล่านี้
- commit ของรุ่นเก่า (ยืนยันด้วย `git merge-base --is-ancestor` ว่าอยู่บน `main` ทั้งหมด และ `Cargo.toml` ตรงเวอร์ชัน): 1.0.0 = `7946cbb57a0f4618711e765a844f46194150ec1f` (handoff ข้อ 5.1 เดิม), 1.1.0 = `51a9658096d7f5f8b2ea6ed00acd49b5db463a47`, 1.1.1 = `be5893e7ab2fe40285205e976ad366bd1d27d50d`
- สคริปต์ build ของทั้งสามรุ่น: workflow รัน `package.ps1 -Arch x64` ขั้นเดียวไม่มี secret (ยังไม่มี `-BuildOnly`), `DOC4ME_BUILD_SHA` มาจาก `git rev-parse HEAD` จึงตรงกับ commit ที่ checkout, `cargo build --release --locked`, ไม่มี `rust-toolchain.toml` (ใช้ Rust stable ปัจจุบัน) และไม่มี workflow รุ่นใดตั้ง `CRAFT_FONTS_DIR`
- ข้อจำกัดของ GitHub: เอกสาร REST "Create a release" ระบุว่า `GITHUB_TOKEN` ให้ Release สร้าง tag ที่ commit ซึ่งไฟล์ workflow ต่างจาก default branch ไม่ได้ (commit ของรุ่นเก่าต่างทั้งหมด) `861e994` จึงให้ workflow สร้าง tag ด้วย git refs API ก่อนแล้วค่อยออก Release ของ tag นั้น แต่ API นั้นก็ตอบ 403 (ข้างบน) tag ของรุ่นเก่าจึงต้อง push จาก clone

## สถานะรอบที่ 18 (2026-11-02)

ผู้ใช้สั่ง (2026-11-02): อ่าน handoff.md และ ROADMAP.md แล้วทำส่วนที่ค้างต่อให้เสร็จ commit/push `main` ก่อนแล้วค่อยตรวจทีหลัง ตรวจเสร็จ commit/push ที่แก้ แล้วบันทึก handoff.md และ ROADMAP.md ตามสถานะปัจจุบัน

### งานรอบนี้ (push ไป `main` แล้ว รายละเอียดและผลตรวจอยู่ใน `CHANGELOG.md` `[Unreleased]`)

- `be82b92` เก็บ `w:wBefore`/`w:wAfter` ของแถวที่เว้นคอลัมน์ต้น/ท้าย (ต่อจาก `ee9c49f` ของรอบที่ 17 ซึ่งรองรับ `w:gridBefore`/`w:gridAfter` แล้ว; session นี้เคยทำซ้ำซ้อนไว้ในเครื่องจึงทิ้งของตัวเองและเติมเฉพาะส่วนที่ขาด)
- `7c5899a` ระยะขอบภายในกล่องข้อความตามไฟล์ (`wps:bodyPr` `lIns`/`tIns`/`rIns`/`bIns`) และจุดยึดของวัตถุลอยไม่ทำให้บรรทัดสูงขึ้น (govuk-005: ท้ายกระดาษ "Page 5 of 37 ASF1_ V4.0" ครบแล้ว และไม่มีหน้า 6 เปล่า)
- `b7d4321` ย่อหน้าว่างที่มีแต่ตัวแบ่ง section ท้ายหน้า (section ถัดไปขึ้นหน้าใหม่) ไม่สร้างหน้าเปล่า (zenodo-135 เคยเกิน 4 หน้า)
- `9cd6bff` Review › Chinese Conversion: จีนตัวเต็ม ↔ ตัวย่อด้วยตาราง OpenCC (Apache-2.0, `assets/opencc`, มีแถวใน `ATTRIBUTION.md`) ผูก `dlg-174`; Convert to Thai/Arabic Digits ใช้ `map_chars` ตัวเดียวกัน
- ตรวจหลัง push (กฎผู้ใช้: push ก่อนแล้วตรวจ): `cargo fmt --all --check`, `cargo test -p` doc 118, docbin 65, docx 224, layout 239, engine 451+13+8, ui-egui `--lib` 396, i18n `--include-ignored` 62 ผ่าน, `cargo clippy --workspace --all-targets -- -D warnings` ผ่าน, `cargo xtask assets` ผ่าน (79 ไฟล์); อ่าน diff ซ้ำแล้วไม่พบบั๊กที่ต้องแก้ จึงไม่มีคอมมิตแก้หลังตรวจ

### ผลวัดรอบนี้ (ยังไม่ได้สร้าง `docs/readiness.md` ใหม่)

- Dialogs: 188/228 (เพิ่ม `dlg-174`); ชุดนี้ตรวจจาก registry จึงไม่ขึ้นกับแพลตฟอร์ม
- Pagination วัดบน Linux ด้วยสคริปต์ที่ใช้เกณฑ์เดียวกับ `cargo xtask readiness --corpus` (scratchpad ของ session) เทียบก่อน/หลังงานรอบนี้บนเครื่องเดียวกัน: จำนวนหน้าตรง 160 → 166, คำต้น/ท้ายทุกหน้าตรง 82 → 89, ไม่มีไฟล์ที่ผลแย่ลง (ดีขึ้น: govuk-057, zenodo-054, zenodo-127, zenodo-165 จากระยะขอบกล่องข้อความ/จุดยึด; zenodo-072, zenodo-102, zenodo-135, zenodo-151, zenodo-154 จากกฎย่อหน้า section); ตัวเลขสัมบูรณ์บน Linux ต่างจาก Windows (รอบ 17: 162/87) เพราะ Linux ไม่มีฟอนต์ Windows หลายตัว (Tahoma, Segoe UI, MS Gothic ฯลฯ) ต้องวัดซ้ำบน Windows: `cargo xtask readiness --corpus --jobs 4 --cli target\\release\\doc4me-cli.exe`
- ภาษา 30/95 ไม่เปลี่ยน

### ข้อค้นพบที่ต้องให้เจ้าของ repo ตัดสิน (ไม่ได้แก้ manifest หรือ comparator ตามกติกา)

- `govuk-026`: DOCX บันทึกเมื่อ 2025-01-30 และ `docProps/app.xml` นับ 4 หน้า (เท่ากับ Doc4Me) แต่ PDF อ้างอิงสร้างปี 2012 ด้วย Acrobat PDFMaker 9.1 มี 7 หน้า (แต่ละตารางขึ้นหน้าใหม่) จึงน่าจะเป็นคนละรุ่น; เกณฑ์ `excluded` ใน `docs/readiness/README.md` เขียนไว้ตามชื่อไฟล์ จึงไม่ได้ตัดออกเอง
- แถวที่หน้าของ Doc4Me เท่ากับ `app.xml` แต่ไม่เท่า PDF (อาจคนละรุ่นเหมือนกัน แต่ `app.xml` อาจค้างจึงต้องตรวจก่อน): `govuk-018` (7/7 vs PDF 6), `zenodo-104` (25/25 vs PDF 29)

### งานต่อไป (ยังไม่ถึงเป้าหมาย 100%)

1. **วัดซ้ำบน Windows** แล้วสร้าง `docs/readiness.md` และตัวเลขใน `ROADMAP.md` ใหม่จากผลนั้น
2. **Pagination**: `govuk-005` บน Linux เหลือ 37 หน้า (Word 36) ส่วนที่ยังคลาด: ความสูงบรรทัดที่มี ☐ (U+2610, Word ใช้ MS Gothic/Segoe UI Symbol ซึ่ง Linux ไม่มี) ใน section แบบหลายคอลัมน์ต่อเนื่อง และการจัดคอลัมน์ก่อนตัวแบ่งต่อเนื่อง (ตารางถัดไปเลื่อนลง ~13pt) ให้ดูบน Windows ก่อน; รูปแบบที่พบแต่ยังไม่แก้: `zenodo-160` (ตัวแบ่งหน้าตามด้วยย่อหน้าว่างที่มีตัวแบ่ง section และ section ถัดไปขึ้นหน้าใหม่: Word ไม่มีหน้าเปล่า แต่ Doc4Me มี; ต้องทิ้งหน้านั้นพร้อมจัดการเลขหน้าและขนาดหน้า), `zenodo-023` (หน้าสุดท้ายของ Word แสดงหัวกระดาษของ header1 และเลข "1" ทั้งที่ section สุดท้ายอ้าง header2/footer2 ที่ว่าง)
3. **Dialogs**: เหลือ 28 แถวที่ไม่ deferred; ที่ทำได้จริงต่อไป: `dlg-047` Fit Text (`w:fitText` ต้องจัดวาง), `dlg-021` Edit Links, `dlg-218` East Asian typography (kinsoku), `dlg-229` Two Lines in One, `dlg-083` Horizontal in Vertical; `dlg-209` File Locations มีผลจริงได้เฉพาะโฟลเดอร์ Documents เพราะ AutoSave เขียนไฟล์ตรงและไม่มีโฟลเดอร์ template ของผู้ใช้
4. **Localization** 30/95: ต้องแปลจริงราว 2,900 ข้อความต่อภาษา ห้ามนับ fallback ภาษาอังกฤษ
5. คง 26 แถว deferred ไว้จนกว่าจะมีหลักฐานหรือคำสั่งจากเจ้าของ

## ผลวัดรอบที่ 17 (2026-11-01, Windows)

### Readiness ล่าสุด: 86.0% (วัดครบ 222 เอกสาร, 2026-11-01)

| มิติ | น้ำหนัก | คะแนน | ชุดตรวจ |
|---|---:|---:|---|
| ฟีเจอร์ | 0.30 | 91.0% | ribbon 499/499; dialogs 187/228 (12 deferred; dlg-040 excluded: Macintosh-only) |
| รูปแบบไฟล์ | 0.20 | 100.0% | เปิด 222/222; round-trip 222/222 |
| การจัดหน้า | 0.15 | 56.1% | จำนวนหน้าตรง 162/222; คำต้น/ท้ายตรง 87/222 |
| UI/UX | 0.10 | 95.7% | shortcuts 268/280 (7 deferred) |
| ความเสถียร | 0.10 | 100.0% | corpus 222/222; hand checks 5/5 |
| ประสิทธิภาพ | 0.05 | 100.0% | 6/6 |
| ภาษา | 0.04 | 31.6% | 30/95 |
| แพลตฟอร์ม | 0.03 | 80.0% | 4/5 (Windows arm64 deferred) |
| ระบบเสริม | 0.02 | 83.3% | 5/6 (1 deferred; Syncthing eco-003 ผ่าน) |
| ฮาร์ดแวร์ | 0.01 | 37.5% | 3/8 (5 owner-deferred) |

คะแนนเมื่อไม่นับ 26 แถว deferred = 88.5%; ยังไม่ถึง 100% เพราะยังมีความคลาดเคลื่อนด้าน pagination, dialog และภาษา รวมถึงการตรวจที่ต้องใช้ฮาร์ดแวร์/แพลตฟอร์มของเจ้าของ repo; ไม่เปลี่ยนสถานะ deferred โดยไม่มีหลักฐานหรือคำสั่งตรงตามกติกา. `dlg-040` ถูกตัดออกจาก check set เพราะเป็น Macintosh-only ตามเอกสารอ้างอิง ไม่ใช่การทำเครื่องหมายผ่าน. ไฟล์ผล generated อยู่ใน `docs/readiness.md`.

รอบนี้เชื่อม `dlg-144`, `dlg-189` และ `dlg-190` (New Toolbar, Customize Menu Bar และ Customize Menus แบบคลาสสิก) กับ `tools.customizeRibbon`; Word 365 ใช้ ribbon แทน menu bar และ Doc4Me มี custom tabs, groups และ command placement. UI regression `customize_ribbon_opens_and_the_layout_persists` ผ่าน. Dialog checks เพิ่ม 184→187/228; readiness 86.0% (88.5% ไม่มี deferred), corpus ยัง 222/222 เปิดและ round-trip, pagination 162/222 page counts และ 87/222 page boundaries. ตรวจ: `cargo test -p doc4me-ui-egui --lib customize_ribbon_opens_and_the_layout_persists` และ `cargo xtask readiness --corpus --jobs 4 --cli target\\release\\doc4me-cli.exe` ผ่าน.

### งานต่อไป (ยังไม่ถึงเป้าหมาย 100%)

1. **Pagination ก่อน**: 162/222 page counts และ 87/222 page-boundary checks ผ่าน; ใน readiness ปัจจุบันยังตก 60 page counts และ 135 page-boundary checks. เริ่มตรวจ `govuk-005` (Doc4Me 40 หน้า, Word reference 36) และแยกสาเหตุระดับเอกสาร/ตาราง/ฟอนต์ก่อนเปลี่ยนกฎ layout; อย่าปรับ comparator หรือรับผลเมื่อ DOCX กับ reference เป็นคนละรุ่น.
2. **Dialogs**: ผ่าน 187/228; 41 แถวไม่ผ่านรวม 12 deferred (จึงมี 29 แถวที่ยังต้องตรวจ/ทำจริง). เติม mapping เฉพาะเมื่อมี UI/command ที่ทำงานเทียบเท่าจริงและมี regression test.
3. **Localization**: ผ่าน 30/95; อีก 65 ภาษาไม่มี catalog ที่ผ่านเกณฑ์. ต้องเพิ่มคำแปลจริงและตรวจ coverage; ห้ามนับ English fallback หรือ mapping ที่ไม่มี catalog เป็นผ่าน.
4. รัน `cargo xtask readiness --corpus --jobs 4 --cli target\\release\\doc4me-cli.exe` หลังการแก้แต่ละช่วง, ปรับ readiness/gap/parity documents และ `CHANGELOG.md` ตามผลจริง, แล้ว commit/push `main`; คง 26 owner-deferred checks ไว้ตามสถานะจนกว่าจะมีหลักฐานหรือคำสั่งจากเจ้าของ.
5. งานรอบนี้จบตามคำขอให้หยุดหลังบันทึกสถานะ; readiness ยังไม่ใช่ 100% (86.0% รวม, 88.5% เมื่อตัด 26 deferred) และยังไม่มีการอ้างว่าทำเป้าหมายสำเร็จ.

รอบนี้เชื่อม `dlg-177` (`wdDialogToolsAdvancedSettings`) กับ `file.options` โดยมี `pane=advanced`; File › Options มี Advanced pane จริงและ UI regression `options_panes_open_and_change_settings` ตรวจการเปิด pane และเปลี่ยนค่าได้. จำนวน dialog ผ่านเพิ่ม 183→184/228; readiness 85.8% (88.3% ไม่มี deferred), corpus ยัง 222/222 เปิดและ round-trip, pagination 162/222 page counts และ 87/222 page boundaries. ตรวจ: `cargo test -p doc4me-ui-egui --lib options_panes_open_and_change_settings` และ `cargo xtask readiness --corpus --jobs 4 --cli target\\release\\doc4me-cli.exe` ผ่าน.

รอบนี้ยืนยัน `eco-003` ด้วย Syncthing 2.1.7 สอง peer บน loopback-only: Doc4Me สร้าง DOCX ในโฟลเดอร์ A, sync ไป B, เปิดและบันทึกทับไฟล์เดิมจาก B แล้วตรวจว่าไฟล์ที่เปิดซ้ำจากทั้งสอง peer มี edits จากทั้งคู่. ปิด global/local discovery, relay, NAT และ crash reporting ระหว่างทดสอบ; ล้าง whitespace ใน `encryptionPassword` ของ template เพื่อให้ cluster-config มี device info ครบ. บันทึกแถวเป็น `pass`; Nextcloud Desktop ไม่ได้ทดสอบแยก เพราะเกณฑ์ระบุ Syncthing หรือ Nextcloud อย่างใดอย่างหนึ่ง. readiness ใหม่ 85.7% (88.2% ไม่มี deferred), ecosystem 5/6.

รอบนี้แก้ `crates/layout/src/lib.rs`/`table.rs` ให้ข้ามชิ้นแรกที่เกิดจากการแยกแถวตารางเฉพาะเมื่อทุก cluster เป็น page-break marker; เดิมตรวจ `Line::vis` ซึ่งว่างสำหรับบรรทัด LTR ทำให้เสี่ยงทิ้งข้อความจริง. เพิ่ม/คง regression สำหรับ `a_leading_page_break_in_a_table_row_does_not_leave_a_blank_page` และ `table_rows_keep_with_next_and_page_breaks_in_cells`; ทดสอบ layout 235 ผ่านเมื่อข้าม 3 test ที่ขึ้นกับฟอนต์ไทยใน Windows profile, suite เต็ม 235 ผ่านแต่ 3 fail เพราะ TH Sarabun New/PSK ติดตั้งอยู่และทำให้ test fallback ล้ม. `cargo clippy` ของ 6 crates, `cargo fmt --all -- --check`, release build และ `cargo xtask readiness --corpus --jobs 4 --cli target\\release\\doc4me-cli.exe` ผ่าน; corpus ครบ 222 ไฟล์ readiness 85.4%, page counts 162/222, page breaks 87/222. `govuk-003` มีจำนวนหน้าเท่ากับ reference (5) แต่ page-boundary check ยังไม่ผ่าน จึงไม่เปลี่ยนสถานะ check. `cargo xtask ci` ยังติด workspace Clippy จาก unused import ใน `apps/doc4me/src/screen_clip.rs` และ `chunks_exact_to_as_chunks` ใน `apps/doc4me/src/dictate.rs` ซึ่งอยู่นอก diff นี้; dialogs 183/228, localization 30/95 และ ecosystem 4/6 (หนึ่งรายการยังไม่ทดสอบและหนึ่ง deferred) ยังคงเป็นช่องว่าง.

รอบนี้เพิ่ม `w:gridBefore`/`w:gridAfter` ใน row model, DOCX reader/writer, table layout, cell/column helpers และ flow conversion เพื่อไม่เลื่อน cell ที่ขาด grid columns ต้น/ท้ายแถวไปยังคอลัมน์ผิด; govuk-009 ผ่าน page-count และ page-break check หลังแก้. วัด corpus ครบ 222 เอกสาร: readiness 85.4% (87.8% เมื่อตัด deferred), pagination 161/222 และ 87/222. ผ่าน `cargo fmt --all -- --check`, model (118 tests), DOCX (75 unit + 49 round-trip), docbin, formats (70), layout regression, engine table tests (36) และ Clippy ของ 6 crates. `cargo xtask readiness --download --corpus --jobs 4 --cli target/release/doc4me-cli.exe` ผ่าน; `cargo xtask ci` หยุดที่ workspace Clippy จาก unused import ใน `apps/doc4me/src/screen_clip.rs` และ lint `chunks_exact_to_as_chunks` ใน `apps/doc4me/src/dictate.rs` ซึ่งอยู่นอก diff นี้ จึงไม่ถึง gates ที่เหลือ. ยังไม่ถึงเป้าหมาย 100%, โดย dialogs และ localization ยังเป็นช่องว่างหลัก และ Nextcloud/Syncthing ยังไม่มีหลักฐานผ่าน.

รอบนี้แก้ `crates/layout/src/lib.rs` ให้ `w:framePr` marker ของ Word ที่มีค่า auto/zero-size ตามรูปแบบที่พบใน corpus อยู่ใน page flow แทนการรวมเป็น floating frame เดียว; เพิ่ม regression test ใน `crates/layout/src/tests.rs`. `zenodo-030` กลับมาแสดงเนื้อหาที่เคยถูกตัดและจำนวนหน้าเพิ่มจาก 16 เป็น 34 (Word 32) แต่ยังไม่ผ่าน exact page-count; aggregate pagination คงเดิมที่ 160/222 และ 86/222. `cargo xtask readiness --corpus --jobs 4 --cli target/release/doc4me-cli.exe` วัดครบ 222 เอกสารและ readiness เป็น 85.3%. Layout suite แบบข้าม 3 test ผ่าน 233 tests; suite เต็มมี 3 failures เพราะ test เหล่านั้นคาดว่า TH Sarabun New/PSK จะไม่ติดตั้งเพื่อทดสอบ fallback แต่ Windows profile นี้มี TH Sarabun New ติดตั้งอยู่; fmt และ Clippy ของ layout ผ่าน. `cargo xtask ci` หยุดที่ workspace Clippy เพราะข้อผิดพลาดเดิมใน `apps/doc4me/src/screen_clip.rs` และ `apps/doc4me/src/dictate.rs` (ไม่อยู่ใน diff นี้) จึงยังไม่ถึง test/assets/layers/wasm gates.

ทดสอบ ecosystem `eco-003` ด้วย Syncthing 2.1.7 แบบสอง peer บน loopback แล้ว แต่ peer ปฏิเสธ cluster configuration และไม่ได้ยืนยันว่า Doc4Me บันทึก DOCX ทับตำแหน่งแล้ว sync/reopen ได้ จึงคงสถานะ `untested` และไม่นับผ่าน; Nextcloud Desktop ยังไม่ได้ตรวจ.

รอบนี้แก้การจัดประเภทภาพใน `crates/docx/src/read/story.rs`: เมื่อ `wps:wsp` มี `w:txbxContent` ภาพที่อยู่ข้างในเป็นเนื้อหาของ story ไม่ใช่รูปภาพแทนกล่องข้อความ; เพิ่ม regression test ที่มีภาพ PNG จริงและตรวจ round-trip. corpus ผ่าน 222/222 (รวม zenodo-153); คะแนน readiness รวมและคะแนนเมื่อตัด deferred ยังเท่าเดิมหลังปัดเศษ. ตรวจ `cargo fmt --all`, `cargo test -p doc4me-docx`, `cargo clippy -p doc4me-docx --all-targets -- -D warnings`, release build และ readiness corpus ครบ. เป้าหมาย 100% ยังไม่สำเร็จ; งานหลักที่เหลือคือ pagination, dialogs และภาษา.

รอบนี้เพิ่ม `Alt+Shift+F10` ให้ปิด KeyTips (ถ้าเปิดอยู่) และเปิดเมนูคำสั่งของเอกสารตรง caret; Escape ปิดกลับได้; UI regression test ผ่าน, `ks-319` เปลี่ยนเป็นผ่านตามหลักฐาน; shortcuts 268/280 และ readiness รวมยัง 85.2% (87.6% เมื่อตัด deferred). ตรวจ `cargo fmt --all -- --check`, UI suite แบบข้าม network integration test (394 ผ่าน), `cargo clippy -p doc4me-ui-egui --all-targets -- -D warnings` และวัด corpus readiness ครบ. UI suite เต็มมี Zotero integration test ล้มเหลวเพราะ remote host ปิดการเชื่อมต่อ; งาน 100% ยังไม่สำเร็จ.

รอบนี้แก้ `Alt+Shift+Plus` ให้สลับขยาย/ยุบหัวข้อใน Outline ตามสถานะ และเลิกดัก `Alt+Shift+Minus` เพื่อให้แทรก nonbreaking hyphen ได้ตามปกติ; regression test ตรวจ mapping และทั้งสองสถานะ; ปรับรายการ `ks-257`/`ks-258`. ตรวจ `cargo fmt --all -- --check` ผ่าน, `cargo test -p doc4me-engine --lib outline_show_level_filters_paragraphs` (1 ผ่าน), `cargo test -p doc4me-ui-egui` (395 ผ่าน) และ readiness corpus ครบ (85.2%). `cargo xtask ci` ยังไม่ผ่าน: Clippy พบ unused import ใน `apps/doc4me/src/screen_clip.rs` และ lint `chunks_exact_to_as_chunks` ใน `apps/doc4me/src/dictate.rs` ซึ่งไม่ได้อยู่ใน diff นี้; `cargo test --workspace` มี test failures ใน control server/XDG app dirs และ font-dependent tests บนเครื่องนี้. งานใหม่อยู่ใต้ `[Unreleased]`; ยังไม่ออกเวอร์ชัน.

### การตัดสินใจถาวรของผู้ใช้ (ทำตามเสมอ ไม่ต้องถามหรือแจ้งอีก)

- เป้าหมาย (สั่ง 2026-11-03): ทำงานต่อจน readiness **100% เมื่อไม่นับ 26 แถว deferred** แล้วรายงานผล; ลำดับ: **pagination ก่อน** แล้วจึง localization และ dialogs/shortcuts; ไม่เพิ่มแถว deferred (เกณฑ์คงที่ 26 แถว)
- Localization (สั่ง 2026-11-03): **ใช้ subagent แปลหลายภาษาพร้อมกันได้**; ระบุในหัวไฟล์และ `CHANGELOG.md` ว่าเป็นคำแปลจาก AI ที่ยังไม่ได้ให้เจ้าของภาษาตรวจ; คง clean room (ไม่ดูคำแปลของ Microsoft); ภาษาที่ต้องใช้ฟอนต์/การจัดรูปอักษรนอกเหนือ Inter (อารบิก ฮีบรู อินเดีย อาร์เมเนีย จอร์เจีย สิงหล เกาหลี ฯลฯ) ต้องให้ UI วาดได้จริงก่อนนับว่าผ่าน
- Dialog 8 แถวที่ Word รุ่นปัจจุบันตัดออกแล้ว (`dlg-081`/`dlg-082` WordPerfect Help, `dlg-151` Schema Library, `dlg-234` XML Element Attributes, `dlg-235` XML Options, `dlg-215` Smart Tag Options, `dlg-079` Frames Page, `dlg-010` CSS Links, `dlg-052` Format Callout) **ให้สร้างให้ใช้งานได้จริง** (มีผลจริงต่อเอกสาร ไม่ใช่ dialog เปล่า) ไม่ defer (สั่ง 2026-11-03)
- แจ้งสถานะความคืบหน้า readiness ให้ผู้ใช้เป็นระยะ (สั่ง 2026-11-03): หลังผลวัด Windows แต่ละรอบ ให้รายงานตัวเลข (รวม, ไม่นับ deferred, dialogs, การจัดหน้า) และสิ่งที่เปลี่ยน
- วัด readiness บน Windows ทุกครั้งหลังเปลี่ยนโค้ด ด้วย workflow `readiness-windows.yml` (สั่ง 2026-11-03) แล้วบันทึกผลใน handoff และ ROADMAP พร้อม commit/push `main`
- ไม่ต้องสั่งรัน workflow Windows x64 installer ทุกครั้งที่ commit push `main` เว้นแต่จะสั่งให้ออกเวอร์ชันใหม่ (สั่ง 2026-11-03; trigger ของ `main` ถูกเอาออกแล้ว)

- ตอบและแสดงข้อความเป็นภาษาไทยเท่านั้น (ชื่อคำสั่ง ชื่อไฟล์ และโค้ดคงเดิมได้)
- ห้ามออกเวอร์ชันใหม่จนกว่าผู้ใช้สั่ง (งานใหม่อยู่ใต้ `[Unreleased]`, ห้ามสั่ง workflow พร้อม `release: true`); 2026-11-02 ผู้ใช้สั่งออก v1.0.0, v1.1.0, v1.1.1 จาก commit เดิม และ v1.1.2 จากโค้ดล่าสุด (`Cargo.toml` 1.1.2) โดยออก v1.1.2 หลังรุ่นเก่า (ออกครบแล้วในรอบที่ 20 ที่ fork `veevie55/doc4me` ตามที่ผู้ใช้เลือก) รุ่นถัดจาก 1.1.2 ยังห้ามออกจนกว่าผู้ใช้สั่ง
- release workflow ไม่ตั้ง `CRAFT_FONTS_DIR` ข้ามฟอนต์ CJK ของ craft-fonts
- parity ที่ต้องใช้บริการเสียเงินให้ตัดทิ้ง
- ไม่เพิ่ม job ทดสอบบน Linux ใน CI เว้นแต่ผู้ใช้สั่ง
- ข้ามงาน build ของ Windows arm64, Linux และ macOS จนกว่าผู้ใช้จะสั่ง (สั่งเมื่อ 2026-10-26: ไม่สั่งรัน workflow "Windows arm64 and macOS builds" ไม่แก้ปัญหา build ของแพลตฟอร์มเหล่านี้ แถว plat-002 คงสถานะไว้ตามผลล่าสุด)
- ยังติดตาม WordCraft ต้นทางต่อ (เอกสารที่อ้าง PR/issue ของ WordCraft คงไว้)
- ระบบเสริมที่เป็นของ Microsoft (VBA, Office add-ins/Office Store, OneDrive/SharePoint, co-authoring ของ Microsoft) อยู่นอกเป้าหมาย ใช้ open-source แทน ถ้าหาตัวแทนไม่ได้ให้ตัดเงียบ ๆ
- ไม่มีผู้ให้บริการ AI ในโปรแกรม
- TH SarabunIT๙ ไม่มีค่าวัด: เครื่องที่ไม่มีฟอนต์นี้แสดงด้วย Sarabun และแจ้งผู้ใช้ตอนเปิดไฟล์; ไม่ต้องให้เจ้าของภาษาตรวจ UI ไทย
- `.gitignore` คง `/target` ฯลฯ ไว้ (ผู้ใช้เลือกลบแค่บรรทัด `/.claude/`)
- ไม่เปลี่ยนลิงก์ Releases ใน `README.md`/`README.en.md`: คงชี้ `https://github.com/chatraweechai/doc4me/releases` แม้ Release ของ v1.0.0–v1.1.2 อยู่ที่ fork `veevie55/doc4me` (ผู้ใช้สั่ง 2026-11-02 รอบที่ 20)
- ลำดับงาน (ผู้ใช้สั่งซ้ำ 2026-11-02): commit/push `main` ก่อน แล้วตรวจทีหลัง; ตรวจเสร็จ commit/push ที่แก้ แล้วบันทึก `handoff.md` และ `ROADMAP.md` ตามสถานะปัจจุบัน

### ประวัติถึงรอบที่ 8 (2026-10-25; รายละเอียดและผลตรวจอยู่ใน `CHANGELOG.md` `[Unreleased]`)

- `f158368` ลบบรรทัด `/.claude/` ใน `.gitignore`, ลบ `package-lock.json`, บันทึกคอมมิต `9954ae0` และ `fc7bab5` ใน CHANGELOG
- `adb9db9` TH SarabunIT๙: ใช้ Sarabun ขนาดของ Sarabun เอง และหน้าต่าง "ไม่มีฟอนต์นี้ในเครื่อง" ตอนเปิดไฟล์ (`cmd::file::unmatched_fonts`, `font_notice.rs`) — CI run 40189539051 ผ่าน
- `83e1611` ระบบเสริมของ Microsoft และ AI ในโปรแกรมออกจากเป้าหมาย (gap #22, #25) ตัวทดแทน: แมโครของ Doc4Me, แค็ตตาล็อก Add-in ของ Doc4Me, โฟลเดอร์ที่ Nextcloud Desktop/Syncthing ซิงก์
- `062d409` ฝัง Intos และ Intos Display (OFL, github.com/muglug/intos) แทน Aptos/Aptos Display — วัดเทียบไฟล์ Aptos จริงแล้วตรงทุกความกว้าง (ตัดบรรทัดตรง 387/387) ตัวอื่นที่ใกล้ที่สุดคือ Lato 35.7%; แผนความพร้อม 80% ใน `docs/roadmap.md`; handoff นี้ — CI run 40191882219 ผ่าน (build, MSI และ zip)
- ขั้น A เสร็จ (คอมมิต `cargo xtask readiness` ชุดล่าสุดบน `main`): เครื่องวัด `xtask/src/readiness.rs`, ชุดตรวจใน `docs/readiness/` (กติกาใน `README.md`), คลังเอกสารจริง 244 แถวนับ 222 (GOV.UK 72 OGL v3 + Zenodo 172 CC; 22 แถว GOV.UK เป็นคนละฉบับจึง `excluded`) ผลอยู่ที่ `docs/readiness.md` — CI run 40195346897 ของ `52d83ba` ผ่าน
- ขั้น B เสร็จ: `ROADMAP.md` และเอกสาร parity ใช้ค่าวัด 66.1% แทนค่าประมาณ ~60% (รายละเอียดใน CHANGELOG `[Unreleased]` หมวดเปลี่ยนแปลง)
- หมายเหตุ: รายละเอียดขั้น C และคะแนน 73.4% ในบรรทัดถัดไปเป็นประวัติ ณ 2026-10-25 ไม่ใช่สถานะล่าสุด; ผลปัจจุบัน 86.0% อยู่ในส่วนสถานะปัจจุบันด้านบน
- ขั้น C (เป้า 100% — ผู้ใช้ปรับจาก 95% เมื่อ 2026-10-27) กำลังทำ — ค่าวัดล่าสุด **73.4%** (`530f4cc`, ตารางด้านล่าง): `e1678b9` ระยะย่อหน้าแบบค่าที่มากกว่า + ระยะบรรทัด Aptos 1.2207 em, `eca31e8` ตัวแบ่งหน้าท้ายย่อหน้าและ Ctrl+Enter แบบ Word, `530f4cc` บรรทัดที่มีแต่รูปไม่บวก descent (จำนวนหน้าตรง 127 → 153, ต้น-ท้ายทุกหน้า 46 → 63), `1e00303` คีย์ลัดตามรายการของ Microsoft (ผ่าน 145 → 218/280), `10e228f` ข้อความหน้าจอทุก 18 ภาษาครอบคลุม 100% (ภาษาที่ผ่าน 6 → 19 จาก 95), แพ็กเกจ Linux และรุ่นเว็บผ่าน (plat-004, plat-005), `cafd5bf` `33146e8` `00e984e` กล่องโต้ตอบแบบ Word เพิ่ม 23 แถว (127 → 150/229: Break, Mark TOC Entry, Accept/Reject Changes, Highlight Changes, List Commands, Modify Style › Format ×5, Drawing Grid, Compatibility Options, Font Substitution, Insert Rows/Cells, Number, Placeholder และแถวที่มีกล่องอยู่แล้ว); เครื่องมือเทียบตำแหน่งบรรทัดกับ PDF ของ Word อยู่ใน scratchpad ของ session (ydrift.py ใช้ `pdftotext -bbox-layout`) ไม่ได้อยู่ใน repo
- ขั้น C ที่พบแต่ยังไม่ได้แก้ (การจัดหน้า): ฟอนต์ที่ไม่มีในเครื่อง Word เลือกตัวแทนจาก `w:altName` และ `w:family` (roman/swiss/modern) ใน `fontTable.xml` ซึ่ง Doc4Me ยังไม่อ่าน (เช่น govuk-010 Bembo → Times New Roman แต่เราใช้ฟอนต์ sans); เอกสารหลายไฟล์คลาดตั้งแต่ต้นหน้าเพราะความสูงของหัวกระดาษ (govuk-018 +21.7 pt, govuk-024 +15.4 pt, zenodo-025 +5.5 pt)

### ผลวัดก่อนหน้า (2026-10-25, คอมมิต `530f4cc`): 73.4%

| มิติ | น้ำหนัก | คะแนน | ชุดตรวจ |
|---|---|---|---|
| ฟีเจอร์ | 0.30 | 82.8% | ribbon 499/499, กล่องโต้ตอบ 150/229 |
| รูปแบบไฟล์ | 0.20 | 99.8% | เปิด 222/222, round-trip 221/222 |
| การจัดหน้า | 0.15 | 48.6% | จำนวนหน้า 153/222, คำต้น-ท้ายทุกหน้า 63/222 |
| UI/UX | 0.10 | 77.9% | คีย์ลัด 218/280 (ตรวจมือยังไม่ได้รัน 38) |
| ความเสถียร | 0.10 | 60.0% | ไม่มี crash/ค้าง 222/222, ตรวจมือ 1/5 |
| ประสิทธิภาพ | 0.05 | 66.7% | 4/6 |
| ภาษา | 0.04 | 20.0% | 19/95 |
| แพลตฟอร์ม | 0.03 | 60.0% | 3/5 (Windows x64, Linux x64, เว็บ) |
| ระบบเสริม | 0.02 | 66.7% | 4/6 |
| ฮาร์ดแวร์ | 0.01 | 25.0% | 2/8 |

ยังขาด 21.6 จุดจึงถึง 95%: การจัดหน้าให้ได้อีกมากที่สุด (7.7 จุด), กล่องโต้ตอบ 79 แถว (5.2 จุด; หลายแถวเป็นกล่องเก่าของ Word 97–2003), ภาษา 76 ภาษา (3.2 จุด; ต้องแปลภาษาใหม่ทั้งชุดราว 2,600 ข้อความต่อภาษา และ egui ยังไม่รองรับข้อความขวาไปซ้าย), แถวตรวจมือของความเสถียร ประสิทธิภาพ แพลตฟอร์ม ฮาร์ดแวร์ ซึ่งส่วนใหญ่ต้องใช้เครื่อง Windows/macOS ของเจ้าของ repo

### ผลวัดครั้งแรก (2026-10-25, คลัง 222 เอกสาร ตัวเลขสุดท้าย): 66.1%

| มิติ | น้ำหนัก | คะแนน | ชุดตรวจ | ได้เพิ่มสูงสุด (จุด) |
|---|---|---|---|---|
| ฟีเจอร์ | 0.30 | 77.7% | ribbon 499/499, กล่องโต้ตอบ 127/229 | 6.7 |
| รูปแบบไฟล์ | 0.20 | 99.8% | เปิด 222/222, round-trip 221/222 (zenodo-153 กล่องข้อความหาย 5 → 4) | 0.05 |
| การจัดหน้า | 0.15 | 39.0% | จำนวนหน้า 127/222, คำต้น-ท้ายทุกหน้า 46/222 | 9.2 |
| UI/UX | 0.10 | 51.8% | คีย์ลัด 145/280 (ตรวจมือยังไม่ได้รัน 42) | 4.8 |
| ความเสถียร | 0.10 | 60.0% | ไม่มี crash/ค้าง 222/222, ตรวจมือ 1/5 | 4.0 |
| ประสิทธิภาพ | 0.05 | 66.7% | 4/6 (การพิมพ์และเลื่อนตรวจมือ 2) | 1.7 |
| ภาษา | 0.04 | 6.3% | 6/95 (ภาษาอื่นแปลไว้ราว 84%) | 3.7 |
| แพลตฟอร์ม | 0.03 | 20.0% | 1/5 | 2.4 |
| ระบบเสริม | 0.02 | 66.7% | 4/6 | 0.7 |
| ฮาร์ดแวร์ | 0.01 | 25.0% | 2/8 | 0.75 |

ต้องได้เพิ่มอีก 28.9 จุดจึงถึงเป้า 95% (ผู้ใช้เปลี่ยนเป้าจาก 80% เป็น 95% เมื่อ 2026-10-25) วิธีรัน: ติดตั้ง poppler-utils, `cargo build --release -p doc4me-cli` แล้ว `cargo xtask readiness --cli target/release/doc4me-cli --download --corpus` (ราว 3 นาที ไฟล์ลง `corpus/readiness/` ที่ไม่เข้า git)

### งานที่ต้องทำต่อ: ความพร้อมใช้งานจริง 100% แบบวัดได้ (ไม่ใช่ค่าประมาณ; เป้าเดิม 95%)

ตัวเลขใน `ROADMAP.md` เดิมเป็นค่าประมาณ ~60% (ผลรวมถ่วงน้ำหนักของค่าที่ประมาณรายมิติ) ตอนนี้เป็นค่าวัด 66.1% แล้ว (ขั้น A และ B เสร็จ) งานต่อไปคือขั้น C แก้ตามผลจนได้ 95% ฉบับภาษาอังกฤษอยู่ที่ `docs/roadmap.md` หัวข้อ "Path to 95% measured readiness" ทุกข้อต้องบันทึก `[Unreleased]` และ commit/push ไป `main`

**ขั้น A สร้างเครื่องวัด (เสร็จแล้ว 2026-10-25 ผลอยู่ด้านบน รายการเดิมคงไว้เป็นข้อกำหนด)**

1. `cargo xtask readiness` (คำสั่งใหม่ใน `xtask/src/main.rs`): อ่านผลของชุดตรวจทุกมิติ คำนวณ Σ น้ำหนัก × อัตราผ่าน ด้วยน้ำหนักเดิม (ฟีเจอร์ 0.30, รูปแบบไฟล์ 0.20, การจัดหน้า 0.15, UI/UX 0.10, ความเสถียร 0.10, ประสิทธิภาพ 0.05, ภาษา 0.04, แพลตฟอร์ม 0.03, ระบบเสริม 0.02, ฮาร์ดแวร์ 0.01) เขียนผลลง `docs/readiness.md` แล้วให้ `ROADMAP.md` ใช้ตัวเลขนี้แทนค่าประมาณ รายการตรวจเพิ่มได้แต่ห้ามลบ
2. คลัง DOCX จริงอย่างน้อย 200 ไฟล์ที่ใช้ซ้ำได้ (เอกสารราชการที่เป็นสาธารณสมบัติ หรือสัญญาอนุญาตเปิด) แต่ละไฟล์มี PDF อ้างอิงที่ Word สร้าง (ดู Producer ของ PDF ว่าเป็น "Microsoft Word") จึงไม่ต้องมี Word ในเครื่อง เก็บไฟล์ใน `/corpus` (gitignored) และ manifest (URL, สัญญาอนุญาต, SHA-256) ใน repo ต่อยอดจาก `cargo xtask corpus`
3. ตรวจรูปแบบไฟล์ต่อไฟล์: เปิดได้ไม่ error, บันทึกแล้วเปิดใหม่ ข้อความ สไตล์ ตาราง รูป และ part ที่ไม่รู้จักยังอยู่
4. ตรวจการแบ่งหน้าเทียบ PDF อ้างอิง: จำนวนหน้า และคำแรก/คำสุดท้ายของทุกหน้า (ใช้ตัวอ่าน PDF ของ repo หรือ `pdftotext`)
5. ตรวจความเสถียร: เปิด จัดหน้า บันทึก ทุกไฟล์ในคลังภายใต้ timeout ต้องไม่มี panic หรือค้าง, soak test ของ autosave, fuzz `hostile_params_never_panic` ที่มีอยู่
6. รายการตรวจฟีเจอร์และ UI: ตารางคงที่ของ Word for Windows (ฟีเจอร์ กล่องโต้ตอบ ตัวเลือกในกล่องหลัก และรายการคีย์ลัดเริ่มต้นที่ Microsoft เผยแพร่) แต่ละแถวผูกกับเทสต์ที่รันได้ หรือการตรวจด้วยมือบน Windows (เจ้าของ repo)
7. เกณฑ์คงที่ของประสิทธิภาพ (เปิดเอกสาร 500 หน้า, latency การพิมพ์, การเลื่อน), ภาษา (ความครบของ catalog ต่อภาษาเทียบ 30 ภาษาของ Word), แพลตฟอร์ม (build และติดตั้งได้ต่อแพลตฟอร์ม), ฮาร์ดแวร์และระบบเสริมแบบผ่าน/ไม่ผ่าน

**ขั้น B วัดครั้งแรก (เสร็จแล้ว 2026-10-25)** 66.1% แทนค่าประมาณ ~60% ใน `ROADMAP.md` (บรรทัด Stage, ตารางตัวเลขหลัก, ตารางกลุ่มผู้ใช้, คอลัมน์ Measured ในตารางรายมิติ, Why not beta, Upcoming ข้อ 0, 2, 5), `docs/target-app-parity.md`, `docs/gaps.md`, `docs/file-format-parity.md` และ `docs/roadmap.md`; ตัวเลข Mainstream ~55% และ Essentials ~63% ยังเป็นค่าประมาณเพราะเครื่องวัดไม่ได้วัดสองกลุ่มนี้

**ขั้น C แก้ตามผลวัด** เรียงจากน้ำหนักที่ได้คืนต่อชั่วโมงมากที่สุด จนกว่า `cargo xtask readiness` บนคอมมิตล่าสุดได้ ≥ 95.0% (คอลัมน์ "ได้เพิ่มสูงสุด" ในตารางด้านบนบอกว่าแต่ละมิติให้ได้อีกเท่าไร: การจัดหน้า กล่องโต้ตอบ คีย์ลัด และแถวตรวจมือของความเสถียรให้มากที่สุด) — แถวที่ผ่านได้บนเครื่องของเจ้าของ repo เท่านั้น (รายงานจากผู้ใช้ Windows ของความเสถียร stab-002/003/004, การพิมพ์และเลื่อนบน Windows perf-005/006, เครื่องพิมพ์ ปากกา ไมโครโฟน สแกนเนอร์ จอสัมผัส หลายจอ, Mendeley) ทำให้ session ขึ้นได้สูงสุดราว 94% จึงต้องให้เจ้าของ repo ตรวจด้วยมือด้วยจึงถึง 95%

1. gap #1 ไฟล์ DOCX จริงที่เปิดไม่ได้หรือ round-trip เสีย
2. gap #4 การแบ่งหน้า: compatibility options, ตารางและรูปลอย, เทียบกับ PDF อ้างอิง (Aptos ใช้ Intos ที่ความกว้างตรงแล้ว)
3. gap #3 crash/ค้าง/บันทึกไม่ได้บน Windows (#170, #77, #59, #31, #218)
4. gap #2 Word ink (`w14:contentPart`) round-trip
5. gap #6 การพิมพ์แบบ native บน Windows (เลือกเครื่องพิมพ์ พิมพ์สองหน้า)
6. gap #7 กล่องโต้ตอบและตัวเลือกที่ขาด และ gap #24 คีย์ลัดที่ขาด ตามรายการข้อ A6 ที่ไม่ผ่าน
7. gap #13 แก้ไขแผนภูมิและ SmartArt ที่มาจาก Word
8. รัน alpha gate 6 workflow ใหม่บน Windows (เจ้าของ repo)

### งานของเจ้าของ repo (session ทำไม่ได้)

- ใบรับรองลงนามโค้ดหรือ Azure Trusted Signing แล้วใส่ secret (workflow พร้อมแล้ว) (ข้อ 5.2)

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
