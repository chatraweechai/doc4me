# บันทึกการเปลี่ยนแปลง (Changelog)

บันทึกการเปลี่ยนแปลงสำคัญทั้งหมดของ Doc4Me รูปแบบอิง [Keep a Changelog 1.1.0](https://keepachangelog.com/en/1.1.0/)
และเลขเวอร์ชันใช้ [Semantic Versioning](https://semver.org/) (แหล่งเดียวคือ `[workspace.package] version` ใน `Cargo.toml`)

กติกา (`CLAUDE.md` ข้อ 3):

- ทุกการเปลี่ยนแปลงโค้ด build packaging asset หรือเวอร์ชัน ต้องบันทึกใต้ `## [Unreleased]` ใน commit เดียวกัน
- ก่อน push tag `vX.Y.Z` ให้ย้ายรายการไปเป็น `## [X.Y.Z] - YYYY-MM-DD`
- workflow `windows-x64-installer.yml` นำเนื้อหาใต้หัวข้อนั้นไปเป็นบันทึกของ GitHub Release

## [Unreleased]

### เพิ่ม

- **เครื่องวัดความพร้อมใช้งานจริง `cargo xtask readiness` (ขั้น A ของแผน 80%)** (`xtask/src/readiness.rs` ไฟล์ใหม่, `xtask/src/main.rs`, `docs/readiness/` ไฟล์ใหม่: `README.md`, `shortcuts.tsv`, `dialogs.tsv`, `languages.tsv`, `performance.tsv`, `manual.tsv`, `corpus.tsv`; ผลที่ `docs/readiness.md`)
  - เหตุผล: ผู้ใช้สั่ง (2026-10-25) ให้เริ่มขั้น A: ตัวเลขความพร้อม ~60% เดิมเป็นค่าประมาณ ต้องมีเครื่องวัดที่ทุกมิติมาจากชุดตรวจผ่าน/ไม่ผ่านที่ตรึงไว้
  - วิธีวัด: Σ น้ำหนักเดิม × ค่าเฉลี่ยอัตราผ่านของชุดตรวจในมิตินั้น; แถวที่ไม่ได้รันหรือยังไม่ทดสอบนับว่าไม่ผ่าน; แถวเพิ่มได้แต่ห้ามลบ (แถวนอกขอบเขตคงไว้เป็น `excluded` พร้อมเหตุผล); ตัวเลขนับว่าสุดท้ายเมื่อคลังเอกสารจริงครบ 200 ไฟล์
  - ชุดตรวจ: ribbon (catalog 499 รายการ), กล่องโต้ตอบ 235 รายการจาก `WdWordDialog` ของ Microsoft (จับคู่ทีละรายการ ตัดกล่องเฉพาะ Mac และบริการ IRM/Outlook), คีย์ลัดเริ่มต้นของ Word for Windows 329 แถวจากหน้า Keyboard shortcuts in Word ของ Microsoft (ตรวจอัตโนมัติจาก registry 164 แถว ด้วยมือ 116 แถว ซ้ำ 48 แถว), 95 ภาษาของ Microsoft 365 Apps (ผ่านเมื่อ catalog ครอบคลุม ≥ 95%), ประสิทธิภาพ (เอกสารสร้างขึ้นราว 20/100/500 หน้า + การพิมพ์และเลื่อนแบบตรวจมือ), ความเสถียร แพลตฟอร์ม ระบบเสริม ฮาร์ดแวร์ (ตรวจมือพร้อมหลักฐาน)
  - คลังเอกสารจริง: เอกสาร GOV.UK (Open Government Licence v3.0) ที่เผยแพร่ทั้ง `.docx` และ PDF ที่ Microsoft Word สร้าง (Producer/Creator) คำตรงกัน ≥ 90% และ 1–80 หน้า; `corpus.tsv` เก็บ URL และ SHA-256 ไฟล์ไม่เข้า git (`/corpus`); `--download` ดาวน์โหลดและตรวจ SHA-256, `--corpus` รัน `doc4me-cli` ทีละขั้นต่อไฟล์ใน process แยกพร้อม timeout 3 นาที (เปิด, บันทึกแล้วเปิดใหม่ เทียบคำและจำนวนย่อหน้า, ส่งออก PDF เทียบจำนวนหน้าด้วย `pdfinfo` และคำแรก/คำสุดท้ายของทุกหน้าด้วย `pdftotext`, crash/ค้าง); ถ้าเครื่องไม่มี `pdfinfo`/`pdftotext` (poppler) `--corpus` จะหยุดพร้อมบอกให้ติดตั้ง แทนที่จะนับทุกไฟล์ว่าหน้าไม่ตรง; PDF อ้างอิงที่สร้างด้วย Acrobat PDFMaker for Word นับว่าเป็นของ Word ด้วย (จัดหน้าผ่าน Word เหมือนกัน)
  - การตัดสินที่ควรรู้: KeyTips ของแท็บ Design/References/Review/View ใช้ตัวอักษรต่างจาก Word (L/R/S/V แทน G/S/R/W) และ File/Developer ไม่มีตัวอักษร จึงไม่ผ่าน; Ctrl+Shift+V ของ Word 365 คือวางข้อความอย่างเดียวแต่ Doc4Me เป็นวางรูปแบบ; Ctrl+G ของ Word คือ Go To แต่ Doc4Me เป็น Find Next; แถวที่หน้าของ Microsoft ขัดกับคีย์เดิมของ Word (ตัวห้อย ขีดกลาง Collapse) ยึดตามหน้าที่เผยแพร่และบันทึกใน note ให้ยืนยันบน Word for Windows
  - ผู้ใช้: ไม่มีผลต่อโปรแกรม ไฟล์ .docx หรือ installer (เครื่องมือของนักพัฒนา); ROADMAP ยังใช้ค่าประมาณจนกว่าจะทำขั้น B
  - ตรวจสอบ: `cargo test -p xtask` ผ่าน 26 ข้อ (ใหม่ 5: การเทียบคีย์, การนับแถวทุกชนิด, สูตรถ่วงน้ำหนักรวม 1.00, catalog/วันที่/คำ, แถวประสิทธิภาพแบบตรวจมือ); `cargo clippy -p xtask --all-targets -- -D warnings` และ `cargo fmt --check` ผ่าน; รันจริง `cargo xtask readiness --download --corpus` กับคลังชุดแรก: ดาวน์โหลด 28 ไฟล์ SHA-256 ตรงทั้งหมด ตรวจ 14 เอกสารเสร็จใน 17 วินาที (เปิดและ round-trip ผ่าน 14/14, จำนวนหน้าตรง 11/14, ต้น-ท้ายทุกหน้าตรง 5/14, ไม่มี crash); ตรวจกรณีไม่ผ่านด้วยมือแล้วเป็นความต่างจริง (เช่น govuk-003 กล่องข้อความ "FOR OFFICIAL USE ONLY" ที่ Word แสดงบนสุดของหน้าแรก Doc4Me ไม่แสดง)
- **ฟอนต์ Intos และ Intos Display (open-source) แทน Aptos และ Aptos Display ได้ตรงทุกความกว้าง เอกสาร Word รุ่นใหม่จึงขึ้นบรรทัดและแบ่งหน้าตรงกับ Word แม้เครื่องไม่มี Aptos** (`assets/fonts/Intos-*.ttf`, `assets/fonts/IntosDisplay-*.ttf` 8 ไฟล์ใหม่, `assets/fonts/OFL-Intos.txt`, `crates/fonts/src/fontdb.rs`, `crates/fonts/src/word.rs`, `packaging/windows/doc4me.wxs`, `NOTICE`, `ATTRIBUTION.md`, `AGENTS.md`, `packaging/env.sh`, เทสต์ใน `crates/fonts/src/word.rs` และ `crates/layout/src/tests_complex.rs`)
  - เหตุผล: ผู้ใช้สั่ง (2026-10-25) ให้หาฟอนต์ open-source แทน Aptos ทั้งหมดแล้วเลือกตัวที่ใกล้ที่สุด Aptos เป็นฟอนต์เริ่มต้นของ Word ตั้งแต่ 2024 (เนื้อความ Aptos หัวเรื่อง Aptos Display) แต่แจกจ่ายต่อไม่ได้ เดิม Doc4Me ใช้ Carlito หรือ Arial แทน ซึ่งความกว้างต่าง บรรทัดจึงตัดต่างจาก Word (gap #4 ที่ขวาง beta)
  - วิธีเลือก: ค้นหาแล้วได้ฟอนต์ที่อ้างว่าความกว้างตรงกับ Aptos สองตัว คือ Akasia (Codeberg, ตอบ 404 แล้วและถูกระบุว่าเลิกพัฒนา) และ Intos (github.com/muglug/intos, OFL-1.1, ดัดแปลงจาก Inter และ Gelasio) แล้ววัดเทียบกับไฟล์ Aptos ของ Microsoft (ชุดดาวน์โหลด 4.40, ฟอนต์รุ่น 2.01; ใช้วัดในเครื่องเท่านั้น ไม่ได้ใส่ใน repo) ด้วยข้อความอังกฤษจากเอกสารใน repo 118 ย่อหน้า 40,296 ตัวอักษร shape ด้วย HarfBuzz ตัดบรรทัดที่ 39 em (6.5 นิ้วที่ 12 pt):
    - Intos: ความกว้างรวม 1.0000 เท่าของ Aptos, จุดตัดบรรทัดตรง 387/387 (100%), ความกว้างรายตัว ASCII ต่างเฉลี่ย 0 em ทั้ง Regular, Bold, Italic, Bold Italic; Intos Display, Narrow และ Serif ก็ได้ 100% กับ Aptos Display, Narrow, Serif; ค่าแนวตั้ง (hhea 1923/−577/0, usWin 2068/563) เท่ากันทุกค่า; มีตัวอักษร 987 จาก 1,086 ตัวของ Aptos (90.9%) และทุกตัวที่มีกว้างเท่า Aptos; ตัวที่ขาด 99 ตัวเป็นอักษรคอปติก กรีก/ซีริลลิกโบราณ เส้นตาราง และ dingbat
    - ตัวที่ใกล้รองลงมาเป็นแค่ฟอนต์หน้าตาคล้าย: Lato ตัดบรรทัดตรง 35.7%, Source Sans 3 28.7%, Roboto 25.3%, Liberation Sans 20.4%, Carlito (ตัวแทนเดิม) 13.2%, IBM Plex Sans 13.7%, Open Sans 2.8%, Inter 1.0% (วัด 22 ฟอนต์จาก Google Fonts และฟอนต์ในระบบ)
  - เปลี่ยน: ฝัง Intos และ Intos Display ทั้ง 4 แบบ (ไม่แก้ไฟล์; SHA-256 อยู่ใน `ATTRIBUTION.md`) เพราะ Aptos Display เป็นฟอนต์หัวเรื่องเริ่มต้น; `substitutes` ให้ Aptos (และ Light/SemiBold/ExtraBold/Black) → Intos, Aptos Display → Intos Display, Aptos Narrow → Intos Narrow, Aptos Serif → Intos Serif (สองตัวหลังใช้เมื่อติดตั้งไว้ ไม่ได้ฝัง); `requested_line` ให้ทุกตระกูล Aptos ใช้ระยะบรรทัดเดี่ยวแบบ Word for Windows (2068 + 563)/2048 = 1.2847 em (เดิมตัวแทนใช้ hhea 1.2207 em); MSI ติดตั้ง `OFL-Intos.txt` ข้าง Doc4Me; `AGENTS.md` ระบุว่า Intos ฝังใน repo นี้ เพราะ release ไม่ใช้ craft-fonts
  - ผู้ใช้: เอกสารที่ใช้ Aptos บนเครื่องที่ไม่มี Aptos แสดงด้วย Intos ซึ่งหน้าตาคล้าย Inter แต่ความกว้างและระยะบรรทัดเท่า Aptos บรรทัดและหน้าจึงตัดตรงกับ Word; เครื่องที่มี Aptos (Office ติดตั้งไว้) ใช้ Aptos จริงเหมือนเดิม; ไฟล์ .docx ไม่เปลี่ยน ชื่อฟอนต์ในไฟล์ยังเป็น Aptos; รายการฟอนต์จะมี Intos และ Intos Display เพิ่ม; PDF ที่ส่งออกฝัง Intos (OFL อนุญาต, fsType 0); ขนาดโปรแกรมใหญ่ขึ้นราว 6.2 MB (ไม่บีบอัด) ทั้ง desktop และเว็บ
  - ตรวจสอบ: `cargo test -p doc4me-fonts` (53 ผ่าน รวม `aptos_is_drawn_with_the_bundled_intos`), `cargo test -p doc4me-layout` (225 ผ่าน รวม `aptos_is_drawn_with_intos_on_aptos_widths_and_line_spacing` ที่เทียบความกว้าง "Hello world" กับค่าที่วัดจาก hmtx ของ Aptos: 10111, Aptos Display 9522, Aptos Bold 10729 หน่วยจาก 2048 และระยะบรรทัด 1.2847 em); `cargo xtask assets` ผ่าน (76 ไฟล์); `cargo test --workspace` ผ่าน 1,813 ล้ม 0 (55 ชุด); `cargo clippy -p doc4me-fonts -p doc4me-layout --all-targets -- -D warnings` และ `cargo fmt --check` ผ่าน; `cargo xtask wasm` ผ่านทั้ง 16 crate
- **หน้าหลัก › เสียง › พูดเป็นข้อความ (Dictate) แปลงเสียงบนเครื่องด้วย whisper.cpp** (parity 498 → 499/500)
  - Word ส่งเสียงไปยังบริการของ Microsoft; Doc4Me แปลงบนเครื่องด้วย whisper.cpp (โอเพนซอร์ส MIT ผ่าน `whisper-rs` 0.16.0 Unlicense) ไม่ต้องใช้บริการออนไลน์: ครั้งแรกดาวน์โหลดโมเดล Whisper (MIT) จาก Hugging Face ผ่าน `curl` เก็บใน `models` ของโฟลเดอร์การตั้งค่า แล้ว **ตรวจ SHA-256** กับค่าที่ Hugging Face ให้ไว้ (base-q5_1 60 MB สำหรับภาษาส่วนใหญ่; small-q5_1 190 MB สำหรับไทย จีน ญี่ปุ่น เกาหลี เวียดนาม อาหรับ ฮินดี ซึ่งจดจำได้ดีกว่ามาก) จากนั้นเสียงไม่ออกจากเครื่อง
  - host (`apps/doc4me/src/dictate.rs`): ไมโครโฟนเริ่มต้นผ่าน `cpal` 0.18 (Apache-2.0; WASAPI/Core Audio/ALSA) แปลงเป็น 16 kHz mono ตัดช่วงพูดเมื่อเงียบ 0.7 วินาทีหรือยาว 25 วินาที (เกณฑ์พลังงานที่ตามระดับเสียงรบกวนของห้อง) ถอดความแต่ละช่วงแล้วส่งข้อความ; ตัด `[BLANK_AUDIO]` `(music)` `*cough*` ที่ Whisper เขียนแทนเสียงที่ไม่ใช่คำพูด
  - UI (`crates/ui-egui/src/dictate.rs`): ปุ่ม "พูดเป็นข้อความ" เป็นปุ่มเปิด/ปิด; หน้าต่างเล็กแสดงสถานะ (ดาวน์โหลดโมเดลพร้อมแถบความคืบหน้า / กำลังเตรียม / กำลังฟัง), เลือกภาษาได้ 21 ภาษา (ค่าเริ่มต้นจากภาษาที่ใช้แก้ไข) และปุ่มหยุด; ข้อความพิมพ์ที่จุดแทรกเหมือนพิมพ์เอง (`text.insert`) เว้นวรรคนำหน้าเมื่อต่อจากคำ ยกเว้นภาษาไทย จีน ญี่ปุ่น; คำสั่ง `tools.dictate {value?}` (pure); hook ใหม่ `Services::dictate`; เว็บแสดงว่าใช้ไม่ได้
  - build: `.cargo/config.toml` ตั้ง `GGML_NATIVE=OFF` และปิด AVX/AVX2/FMA/F16C/BMI2 เพื่อให้ binary ที่ build บน CI ทำงานบน CPU x86-64 ทุกรุ่นที่มี SSE 4.2 (ค่าเริ่มต้นของ ggml คือ `-march=native` ซึ่งจะทำให้โปรแกรมหยุดด้วย "illegal instruction" บนเครื่องรุ่นเก่า) ; bindings ของ whisper.cpp สร้างด้วย bindgen บนแต่ละแพลตฟอร์ม (ต้องมี libclang; runner Windows มี LLVM) — เดิมตั้ง `WHISPER_DONT_GENERATE_BINDINGS=1` แต่ bindings ที่มากับ crate สร้างจาก Linux จึง build บน Windows ไม่ผ่าน (CI run 40178908529 ล้มที่ขนาด `_IO_FILE`) จึงเอาออก; ต้องมี CMake และคอมไพเลอร์ C++ ตอน build และบน Linux ต้องมี `libasound2-dev` (README ทั้งสองภาษาเพิ่มแล้ว); เพิ่ม dependency `sha2` ให้ app
  - แพ็กเกจ: macOS เพิ่ม `NSMicrophoneUsageDescription` ใน `Info.plist.in` และ entitlement `com.apple.security.device.audio-input` (hardened runtime บล็อกไมโครโฟนถ้าไม่มี); Flatpak เพิ่ม `--socket=pulseaudio` ทั้งสอง manifest; `ATTRIBUTION.md` เพิ่ม whisper.cpp, โมเดล Whisper และ Openverse
  - ข้อความใหม่ 7 ข้อความแปลครบ 18 ภาษา
  - ตรวจแล้ว: เทสต์ใหม่ ui-egui `dictate::tests` (2: พิมพ์ข้อความที่จุดแทรก เว้นวรรค หยุดด้วยปุ่ม; ไม่มีบริการ/ไมโครโฟนเสียแจ้งข้อความ), app `dictate::tests` (resample, ตัดช่วงพูด, ตัด marker ที่ไม่ใช่คำพูด, เลือกโมเดลตามภาษา); **ทดสอบจริงด้วย `DOC4ME_WHISPER_TEST=<wav>`**: ดาวน์โหลด base-q5_1 จาก Hugging Face ผ่าน curl ได้ SHA-256 ตรง และถอดความเสียงสังเคราะห์จาก espeak-ng "Hello world. This is a dictation test." ได้คำว่า hello และ test ภายใน 18 วินาที (รวมดาวน์โหลด); `cargo test -p doc4me` 53, ui-egui 344, engine 399 + 13 + 8 ผ่าน; clippy ผ่าน; ส่วนไมโครโฟนทดสอบใน container ไม่ได้ (ไม่มีอุปกรณ์เสียง) และ build บน Windows ตรวจด้วย CI

- **สมการแบบหมึก (Ink Equation) และ แปลงหมึกเป็นคณิตศาสตร์ (Draw › Ink to Math) ด้วยตัวจดจำลายมือของเราเอง** (parity 495 → 498/500; Ink Equation นับ 2 ที่ Insert และ Equation › Tools)
  - Word ส่งลายมือไปยังบริการจดจำของ Microsoft; Doc4Me จดจำบนเครื่องเอง ไม่ใช้เครือข่ายหรือโมเดลภายนอก (`crates/engine/src/ink_math.rs`): จัดเส้นที่ซ้อนกันในแนวนอนเป็นสัญลักษณ์ จับคู่แต่ละสัญลักษณ์กับแม่แบบในตัวด้วยอัลกอริทึม $P point-cloud (Vatavu, Anthony, Wobbrock, ICMI 2012 — เขียนเองจากบทความ ลำดับและทิศทางการลากเส้นไม่สำคัญ) แล้วอ่านโครงสร้างจากตำแหน่ง: ตัวยก/ตัวห้อย (เล็กกว่าและสูง/ต่ำกว่ากึ่งกลางของสัญลักษณ์ก่อนหน้า), เส้นเศษส่วน (เส้นแบนที่มีหมึกทั้งบนและล่าง), รากที่สอง, จุดทศนิยม; ผลเป็นข้อความ UnicodeMath (เช่น `x^2+y^2=z^2`, `a/(b+1)-3`, `√(x+1)`)
  - สัญลักษณ์ที่รู้จัก: 0–9, `a b c n x y z`, `+ − = < > / ( )`, `π`, `∫`, `√` และจุดทศนิยม
  - กล่อง "ตัวควบคุมการป้อนข้อมูลทางคณิตศาสตร์" (เมนู สมการ › สมการแบบหมึก ทั้งแท็บแทรกและแท็บสมการ): แผ่นเขียนด้วยเมาส์/ปากกา จดจำใหม่ทุกเส้น แสดงข้อความที่จดจำได้ให้แก้ไขก่อนแทรก, เลิกทำเส้น, ล้าง (`crates/ui-egui/src/dialogs_insert.rs`)
  - คำสั่ง `insert.inkEquation {strokes?, linear?}` (ไม่มีทั้งสองอย่าง = เปิดแผ่นเขียน) และ `draw.inkToMath` (ปุ่มที่มีอยู่แล้วบนแท็บวาด เดิมไม่มีคำสั่ง): เส้นหมึกที่เลือกด้วย Lasso ถูกจดจำ ลบออก และแทรกสมการแทนที่ตำแหน่งยึดของเส้นแรก เป็น undo step เดียว (`crates/engine/src/cmd/equation.rs`)
  - ความปลอดภัย: จำกัด 300 เส้น 2,000 จุดต่อเส้นในการจดจำ (คำสั่งรับ ≤ 20,000 จุดต่อเส้น) ข้ามจุดที่ไม่ใช่ตัวเลขจำกัด, ข้อความ linear ≤ 4,000 ตัวอักษร, การวนแยกเศษส่วน/รากลึกไม่เกิน 12 ชั้น
  - ข้อความใหม่ 6 ข้อความแปลครบ 18 ภาษา
  - ตรวจแล้ว: เทสต์ใหม่ `ink_math::tests` (3: ทุกสัญลักษณ์ที่เขียนต่างจากแม่แบบเล็กน้อยใน 2 ขนาดจดจำถูก, โครงสร้างตัวยก/ตัวห้อย/เศษส่วน/ราก/ทศนิยม, ข้อมูลอันตรายไม่ panic), `handwritten_math_becomes_an_equation` (engine: จากเส้น, จากข้อความที่แก้, Lasso + Ink to Math ลบหมึกเหลือสมการ 1 undo step), `ink_equation_pad_recognises_and_inserts` (ui-egui); `cargo test -p doc4me-ui-egui` 342 (รวม audit การแปล) และ engine 399 + 13 + 8 ผ่าน; clippy ผ่าน

- **แทรก › Add-in › รับ Add-in (Insert › Get Add-ins) จากแค็ตตาล็อกฟรีใน repo นี้** (parity 494 → 495/500)
  - Word ดึง Add-in จาก Office Store ซึ่งทำงานบน Office.js; Add-in ของ Doc4Me เป็น **ชุดแมโคร** (รายการคำสั่งของ Doc4Me แบบที่ มุมมอง › แมโคร บันทึก) ที่ลงทะเบียนในแค็ตตาล็อก `addins/catalog.json` ของ repo นี้ (เสิร์ฟฟรีผ่าน raw.githubusercontent.com) พร้อม SHA-256 ของแต่ละไฟล์
  - Add-in เริ่มต้น 3 ชุด (เนื้อหาของเราเอง สัญญาอนุญาต MIT OR Apache-2.0 ตาม repo): `thai-document` (เลขไทย/อารบิกทั้งเอกสาร, Sarabun 16 pt, A4 ขอบ 2.5 ซม.), `tidy-text` (เว้นวรรคเดียว, ลบย่อหน้าว่าง, ล้างรูปแบบ), `academic-paper` (Letter ขอบ 1 นิ้ว, 12 pt, ระยะบรรทัดคู่, เลขหน้าขวาบน)
  - คำสั่ง `addins.get {install?: ข้อความ JSON ของชุด, sha256?, remove?: id}` (`crates/engine/src/cmd/addins.rs`, pure): ไม่มีพารามิเตอร์ = เปิดหน้าต่าง; ติดตั้งแมโครเป็นชื่อ `{id}.{ชื่อแมโคร}` (เห็นใน มุมมอง › แมโคร ด้วย); เพิ่ม dependency `sha2` ให้ engine (มีใน build ผ่าน docx อยู่แล้ว)
  - ความปลอดภัย: ตรวจ SHA-256 ตามแค็ตตาล็อกก่อนติดตั้ง; ชุดถูกปฏิเสธถ้าขั้นใดใช้คำสั่งนอกตัวเอกสาร (`file.`, `tools.` รวมแมโครซ้อน, `ui.`, `addins.`, `mailings.`, `zotero.`, `font.`, `readAloud.`, การวาง/แปล/รูปออนไลน์/วิดีโอ/3D/ภาพหน้าจอ/ข้อความจากไฟล์) หรือส่งพารามิเตอร์ `path`/`data`/`url`/`password`/`file`/`template` ที่ระดับใดก็ตาม หรือใช้คำสั่งที่รุ่นนี้ไม่มี; จำกัดขนาด 256 KB, 50 แมโคร, 500 ขั้น, ชื่อสั้น; ชื่อไฟล์ในแค็ตตาล็อกต้องเป็น `[a-z0-9-]+.json` (ไม่มี `../`); `.gitattributes` ตั้ง `addins/*.json -text` ให้ไบต์ตรงกับ SHA-256 เสมอ
  - หน้าต่าง "Add-in" (`crates/ui-egui/src/get_addins.rs`): แท็บ ร้านค้า (รายการจากแค็ตตาล็อก ปุ่ม เพิ่ม) และ Add-in ของฉัน (เรียกใช้แต่ละแมโคร, เอาออก); Add-in ที่ติดตั้งเก็บใน preferences (`UiState.addins`) และติดตั้งใหม่ตอนเปิดโปรแกรม; กลุ่ม "Add-in" ใหม่บนแท็บแทรก (ระหว่างภาพประกอบกับสื่อ แบบ Word for Windows) และ KeyTip
  - ข้อความใหม่ 14 ข้อความแปลครบ 18 ภาษา (รวมข้อผิดพลาดของบริการเรียกเว็บ "ใช้ได้เฉพาะที่อยู่ https", "ใช้งานเว็บที่นี่ไม่ได้")
  - ตรวจแล้ว: เทสต์ใหม่ engine `addins::tests` (4: ติดตั้ง-เรียกใช้-เอาออก, SHA-256 ไม่ตรงถูกปฏิเสธ, ชุดที่เข้าถึงไฟล์/ข้อมูล/แมโครซ้อน/คำสั่งที่ไม่มีถูกปฏิเสธ, แค็ตตาล็อกที่มี `../` หรือ hash ผิดถูกข้าม, **ทุกชุดใน `addins/` ตรง SHA-256 ผ่านการตรวจ และรันได้บนเอกสารตัวอย่าง**), ui-egui `get_addins::tests` (2: เพิ่มจากแค็ตตาล็อก รัน เก็บใน preferences ติดตั้งใหม่ในหน้าต่างใหม่ เอาออก; ชุดที่ถูกแก้ถูกปฏิเสธ); `cargo test -p doc4me-ui-egui` 341 และ engine 395 + 13 + 8 ผ่าน; clippy ผ่าน

- **รีวิว › ภาษา › แปล (Review › Translate) ใช้บริการฟรี** (parity 493 → 494/500)
  - Word ใช้ Microsoft Translator; Doc4Me ใช้ MyMemory เป็นค่าเริ่มต้น (ฟรี ไม่ต้องมีบัญชี วันละ 5,000 ตัวอักษร ใส่อีเมลได้ 50,000 — ตรวจจากหน้า usage limits ของ MyMemory) หรือเซิร์ฟเวอร์ LibreTranslate ที่ผู้ใช้ระบุ (โอเพนซอร์ส ติดตั้งเองได้ฟรี รองรับ API key)
  - บานหน้าต่าง "ตัวแปลภาษา" (`crates/ui-egui/src/translator.rs`): แท็บ "การเลือก" แปลข้อความที่เลือกแล้วกด "แทรก" แทนที่ด้วยคำแปล (`edit.pasteText` รูปแบบตัวอักษรตรงนั้นคงเดิม); แท็บ "เอกสาร" แปลทุกย่อหน้าเป็นเอกสารใหม่ที่แทนที่ฉบับนี้ (แบบ Finish ของจดหมายเวียน เพราะ Doc4Me เปิดหนึ่งเอกสารต่อหน้าต่าง; มีการถามให้บันทึกก่อนถ้ายังไม่บันทึก); ภาษาต้นทางเดาจากภาษาพิสูจน์อักษรของย่อหน้า แล้วจากอักษร (ไทย ญี่ปุ่น จีน เกาหลี ซีริลลิก); มี 26 ภาษา; แสดงว่าข้อความจะถูกส่งไปที่บริการใด; โควตาหมดแสดงข้อความที่เข้าใจได้
  - engine (`crates/engine/src/cmd/translate.rs`): `review.translate` (pure) เปิดบานหน้าต่าง/คืนข้อความที่เลือก หรือทุกย่อหน้าที่มีตัวอักษร (`scope: "document"`) พร้อมภาษา; `review.translateDocument {paragraphs, language?}` สร้างสำเนาที่แทนข้อความแต่ละย่อหน้า คงสไตล์ รูปแบบย่อหน้า รูปแบบตัวอักษรตัวแรก และรูปภาพ ตั้งภาษาของข้อความใหม่; ตัดข้อความเป็นชิ้น ≤ 480 ไบต์ที่ท้ายประโยค/ช่องว่าง (ภาษาไทยตัดที่ขอบอักขระ) เพราะ MyMemory รับได้ 500 ไบต์ต่อครั้ง ส่งทีละคำขอ
  - การตั้งค่าบริการ (บริการ, อีเมล, ที่อยู่เซิร์ฟเวอร์, API key, ภาษาปลายทาง) เก็บใน preferences (`UiState.translator`); เซิร์ฟเวอร์ LibreTranslate ต้องเป็น `https`
  - ข้อความใหม่ 28 ข้อความ (รวมชื่อภาษา 9 ภาษา) แปลครบ 18 ภาษา
  - ตรวจแล้ว: เทสต์ใหม่ engine `translate::tests` (ตัดชิ้นข้อความรวมภาษาไทย; แปลเอกสารคงรูปภาพ ตั้งภาษา ข้อมูลขยะไม่ panic), ui-egui `translator::tests` (3: สร้างคำขอทั้งสองบริการ, อ่านคำตอบและการปฏิเสธ/โควตา, แปลการเลือกและทั้งเอกสารกับบริการจำลอง); `cargo test -p doc4me-ui-egui` 339 และ engine 391 + 13 + 8 ผ่าน; clippy ผ่าน; เรียก MyMemory จริงจาก container ได้คำตอบ 429 โควตาหมด (IP ของ proxy ใช้ร่วมกัน) จึงยืนยันได้เฉพาะรูปแบบคำตอบข้อผิดพลาด

- **แทรก › รูปภาพ › รูปภาพออนไลน์ (Insert › Pictures › Online Pictures)** (parity 492 → 493/500; ผู้ใช้สั่งให้ใช้บริการฟรี/โอเพนซอร์สก่อน)
  - Word ค้นหาด้วย Bing; Doc4Me ใช้ [Openverse](https://openverse.org) (บริการฟรีของ WordPress ค้นรูป Creative Commons และสาธารณสมบัติจาก Flickr, Wikimedia Commons, พิพิธภัณฑ์ ฯลฯ ไม่ต้องมีบัญชีหรือ key; ข้อจำกัดแบบไม่ระบุตัวตน 20 ค้นหา/นาที 200/วัน, รูปย่อ 1,000/วัน ตรวจจาก header `x-ratelimit-*` จริง)
  - ปุ่ม "รูปภาพ" บนริบบอนเป็นเมนูแบบ Word for Windows: "อุปกรณ์นี้…" (เดิม) และ "รูปภาพออนไลน์…"; หน้าต่างค้นหาแสดงรูปย่อ 20 รูป ติ๊ก "เฉพาะรูปที่ใช้เชิงพาณิชย์ได้" ได้ (`license_type=commercial`) ดับเบิลคลิกหรือ "แทรก" เพื่อดาวน์โหลดรูปต้นฉบับ ถ้าล้มเหลวใช้สำเนาของ Openverse แทน (`crates/ui-egui/src/online_pictures.rs`)
  - คำสั่ง `insert.onlinePicture {data, title, creator, license, licenseVersion, licenseUrl, source, attribution?, width?}` (`crates/engine/src/cmd/insert.rs`): แทรกรูปพร้อมบรรทัดให้เครดิตใต้รูปแบบที่ Word ทำ ("“ชื่อ” by ผู้สร้าง is licensed under CC BY-SA 4.0." ชื่อลิงก์ไปหน้าต้นทาง สัญญาอนุญาตลิงก์ไปหน้าสัญญา ขนาด 8 pt) เป็น undo step เดียว; รูป WebP แปลงเป็น JPEG (หรือ PNG ถ้ามีความโปร่งใส) ให้ Word ทุกรุ่นอ่านได้; ไม่มีข้อมูลรูป = เปิดหน้าต่างค้นหา
  - บริการเรียกเว็บใหม่ `Services::fetch` (`crates/ui-egui/src/net.rs`) ใช้ร่วมกับฟีเจอร์ออนไลน์อื่น: desktop เรียก `curl` ของระบบด้วย path เต็ม (Windows 10 1803+ มี `System32\curl.exe`, macOS `/usr/bin/curl`, Linux จาก `PATH`) จึงไม่เพิ่ม TLS stack ให้ Doc4Me และใช้ใบรับรอง/`HTTPS_PROXY` ของระบบ (`apps/doc4me/src/fetch.rs`); เว็บใช้ `fetch` ของเบราว์เซอร์ (`apps/doc4me-web/src/web/fetch.rs`, Openverse อนุญาต CORS ตรวจแล้ว)
  - ความปลอดภัย: ขอได้เฉพาะ `https` (redirect ก็ `https` เท่านั้น `--proto-redir =https`), ที่อยู่อยู่หลัง `--` จึงไม่ถูกอ่านเป็น option ของ curl, header ตัด CR/LF, timeout, จำกัดขนาดคำตอบระหว่างอ่าน (JSON 2 MB, รูป 25 MB, รูปย่อ 4 MB), Windows ไม่มีหน้าต่าง console โผล่ (`CREATE_NO_WINDOW`); ผลค้นหาอ่านทีละฟิลด์ ตัดข้อความยาว ลิงก์ที่ไม่ใช่ `https`/`javascript:` ถูกทิ้ง รูปย่อถอดรหัสเมื่อด้านยาว ≤ 8,000 px
  - ข้อความใหม่ 13 ข้อความแปลครบ 18 ภาษา; KeyTip ของรูปภาพเป็นเมนู
  - ตรวจแล้ว: เทสต์ใหม่ `net::tests` (2), `online_pictures::tests` (2: ค้นหา→รูปย่อ→ต้นฉบับ 404→ใช้สำเนา→แทรกพร้อมเครดิต เป็น undo step เดียว; ลิงก์อันตรายถูกทิ้ง), `an_online_picture_comes_with_its_credit_line` (engine: WebP→JPEG, ลิงก์ `javascript:` ไม่ถูกใส่), `curl_is_asked_for_https_only_with_the_address_last` (app); ทดสอบกับบริการจริงด้วย `DOC4ME_NET_TESTS=1 cargo test -p doc4me fetch` (ค้นหา Openverse, จำกัดขนาด, POST) ผ่าน; `cargo test -p doc4me-ui-egui` 336 และ engine 389 + 13 + 8 ผ่าน; clippy `-D warnings` (engine, ui-egui, doc4me `--all-targets` และ doc4me-web บน wasm32) ผ่าน

- **แทรก › ภาพประกอบ › แบบจำลอง 3 มิติ (Insert › 3D Models)** (parity 491 → 492/500)
  - ปุ่ม "แบบจำลอง 3 มิติ" ในกลุ่มภาพประกอบ (ใต้ Icons แบบ Word for Windows; Chart ย้ายไปคอลัมน์ถัดไปกับภาพหน้าจอ) เลือกไฟล์ `.glb` (glTF 2.0 binary), `.stl` (binary/ข้อความ) หรือ `.obj`; คำสั่ง `insert.model3d {path|data, name?, width?, yaw?, pitch?}` (`crates/engine/src/cmd/insert.rs`) แทรกภาพของโมเดลเป็นรูปภาพกว้าง 3 นิ้ว (undo step เดียว ข้อความแสดงแทน = ชื่อไฟล์) การเลือกไฟล์ไม่สร้าง undo step; เว็บรับไฟล์ผ่านตัวเลือกไฟล์และการลากวาง
  - ตัวอ่าน+ตัววาด (`crates/engine/src/model3d.rs`): glTF อ่าน mesh ของ scene หลักพร้อม matrix/TRS ของ node และสี `baseColorFactor` (triangles/strip/fan, index u8/u16/u32); STL ถือว่า z ชี้ขึ้น; OBJ รองรับ `v/vt/vn` ดัชนีติดลบ และรูปหลายเหลี่ยม; วาดด้วย z-buffer แบบซอฟต์แวร์ (ไม่ใช้ GPU) แรเงาแบบ flat แสงจากซ้ายบน มุมมอง yaw −30° pitch 20° พื้นหลังโปร่งใส 800×800 px (วาด 2 เท่าแล้วย่อเพื่อขอบเรียบ)
  - ความปลอดภัย: ไฟล์มาจากภายนอก ตรวจทุก offset/count กับขนาด buffer view และ chunk (ไม่อ่านเกินขอบ) จำกัด 200 MB และ 4 ล้านสามเหลี่ยม node ที่วนอ้างตัวเองตัดที่ความลึก 64 / 100,000 ครั้ง; glTF แบบ Draco, sparse accessor, buffer ภายนอก หรือ glTF 1.0 แจ้งว่าอ่านไม่ได้
  - ข้อความใหม่ 2 ข้อความ (ไทย: แบบจำลอง 3 มิติ) ครบ 18 ภาษา, KeyTip และไอคอนลูกบาศก์
  - ผลต่อผู้ใช้/ความเข้ากันได้: แบบจำลอง 3 มิติในไฟล์ของ Word (`am3d` ใน `mc:AlternateContent`) ยังเปิดเป็นรูป fallback เหมือนเดิม; ไม่เปลี่ยนรูปแบบไฟล์
  - ตรวจแล้ว: เทสต์ใหม่ `model3d::tests` (3: อ่าน STL/OBJ/GLB รวม node วน, ปฏิเสธไฟล์เสีย/glTF 1/chunk เกินไฟล์/accessor เกิน buffer view/ไฟล์เกินขนาด, ภาพมีพื้นโปร่งใสและมีโมเดลกลางภาพ) และ `a_3d_model_is_inserted_as_its_picture` (engine); `cargo test -p doc4me-engine` 388 + 13 + 8 ผ่าน, `cargo test -p doc4me-ui-egui -- --include-ignored` 331 ผ่าน (รวม audit การแปล), clippy `-D warnings --all-targets` (engine, ui-egui) ผ่าน

- **แทรก › สื่อ › วิดีโอออนไลน์ (Insert › Media › Online Video)** (parity 490 → 491/500)
  - กลุ่ม "สื่อ" ใหม่บนแท็บแทรก (ระหว่างภาพประกอบกับลิงก์แบบ Word for Windows) ปุ่ม "วิดีโอออนไลน์" เปิดกล่อง "แทรกวิดีโอ" ให้วางที่อยู่เว็บหรือโค้ดฝัง `<iframe>`; คำสั่ง `insert.video {url, poster?, title?}` (`crates/engine/src/cmd/insert.rs`) แทรกรูปโปสเตอร์ที่เก็บวิดีโอไว้ เป็น undo step เดียว การเปิดกล่องไม่สร้าง undo step (`input_dialog` ใน `crates/ui-egui/src/lib.rs`)
  - โมเดล: `InlineObject::Image` มีฟิลด์ใหม่ `video: Option<Arc<WebVideo>>` (`crates/doc/src/video.rs`) ที่อยู่ YouTube (`watch?v=`, `youtu.be`, `shorts`, `live`) และ Vimeo แปลงเป็น player (`/embed/ID`, `player.vimeo.com/video/ID`) ที่อยู่อื่นต้องเป็น https; จากโค้ดฝังเอาเฉพาะ `src` กับขนาด (16–4096 px ไม่งั้นใช้ 640×360) แล้วสร้าง `<iframe>` ใหม่เอง ไม่เก็บ HTML/script ที่ผู้ใช้วาง (กัน script แฝงในไฟล์ที่ Word จะแสดง)
  - .docx: อ่าน-เขียน `wp15:webVideoPr` (`embeddedHtml`, `w`, `h`; [MS-ODRAWXML] 2.19) ใน `a:blip/a:extLst/a:ext uri="{C809E66F-F1BF-436E-B5F7-EEA9579F0CBA}"` (URI ตามที่ Word เขียน ตรวจจากตัวอย่าง OOXML จริง) namespace ประกาศที่ตัว element จึงไม่ต้องแก้ `mc:Ignorable`; HTML ที่ยาวเกิน 64 KB ถูกทิ้ง (รูปยังอยู่) — วิดีโอออนไลน์จากไฟล์ของ Word จึงไม่หายเมื่อบันทึกด้วย Doc4Me
  - เล่น: ดับเบิลคลิกรูปวิดีโอ หรือคำสั่ง `picture.playVideo` (pure) เปิดหน้าวิดีโอในเบราว์เซอร์ (YouTube/Vimeo player แปลงกลับเป็นหน้าดูปกติ) เปิดได้เฉพาะ http/https; ฝั่ง UI รับคำขอ `{"openUrl"}` เฉพาะลิงก์เว็บหรืออีเมล
  - โปสเตอร์: `doc4me_render::video_poster` วาดการ์ดสีเข้ม 16:9 พร้อมปุ่มเล่น (Doc4Me ไม่ดึงภาพตัวอย่างจากเว็บไซต์วิดีโอ) หรือส่ง `poster` เป็น base64 เอง; ข้อความแสดงแทน = `title` หรือที่อยู่วิดีโอ
  - ข้อความใหม่ 7 ข้อความแปลครบ 18 ภาษา (ไทย: สื่อ, วิดีโอออนไลน์, แทรกวิดีโอ, เล่นวิดีโอ ฯลฯ) และ KeyTip
  - ผลต่อผู้ใช้/ความเข้ากันได้: ไฟล์ .docx เดิมเปิดได้เหมือนเดิม รูปธรรมดาไม่เปลี่ยน; ไฟล์ JSON ภายใน (`video` ข้ามเมื่อไม่มี) อ่านไฟล์เก่าได้; ติดตั้งทับได้
  - ตรวจแล้ว: เทสต์ใหม่ `video::tests` (3, doc), `poster::tests` (render), `online_video_inserts_a_poster_that_keeps_its_video` (engine: เปิดกล่อง, ที่อยู่อันตราย `javascript:`/`http:`/`file:` ถูกปฏิเสธโดยไม่สร้าง undo, round trip .docx ได้ HTML และขนาดเดิม, เล่นได้, undo), `online_video_asks_for_an_address_and_plays_in_the_browser` (ui-egui); clippy `-D warnings --all-targets` ของ doc, docx, render, engine, ui-egui, doc4me ผ่าน

- **แทรก › ภาพประกอบ › ภาพหน้าจอ › การตัดหน้าจอ (Insert › Screenshot › Screen Clipping)** (parity 489 → 490/500)
  - ปุ่ม "ภาพหน้าจอ" ใหม่ในกลุ่มภาพประกอบ (คอลัมน์ที่สองถัดจาก Icons/SmartArt/Chart แบบ Word for Windows) มีเมนู "การตัดหน้าจอ" และ KeyTip; คำสั่ง `insert.screenshot` (`crates/engine/src/cmd/insert.rs`) เป็นคำสั่ง pure เพียงขอให้ UI เริ่มการตัดหน้าจอ จึงไม่สร้าง undo step และไม่ทำให้เอกสารเป็น "ยังไม่บันทึก" ภาพที่ได้แทรกด้วย `insert.picture` เป็น undo step เดียวเหมือนแทรกรูป
  - UI (`crates/ui-egui/src/screen_clip.rs`): hook ใหม่ `Services::screen_clip` ให้ host เริ่มเครื่องมือตัดหน้าจอของระบบแล้วตอบผ่าน channel; บน Windows และ macOS หน้าต่างย่อลงระหว่างตัด (แบบ Word) แล้วกลับมาพร้อมภาพ บน Linux/BSD ไม่ย่อเพราะ Wayland คืนหน้าต่างที่ย่อแล้วไม่ได้; ถ้าผู้ใช้เปิดหน้าต่างกลับมาเองก่อนตัด ถือว่ายกเลิก; รอนานสุด 5 นาที; ภาพตรวจขนาดเหมือนภาพที่วางจากคลิปบอร์ด (`paste_picture::insert` แยกออกมาใช้ร่วมกัน และเพิ่มชนิด `ClipboardPicture::Encoded` สำหรับไฟล์ภาพที่เครื่องมือบันทึก จำกัด 200 MB)
  - host (`apps/doc4me/src/screen_clip.rs`): Windows เปิด `ms-screenclip:` (Snipping Tool แบบ Windows+Shift+S) แล้วเฝ้าเลขลำดับของคลิปบอร์ดผ่าน `clipboard_win::raw::seq_num` (อ่านภาพเมื่อเลขเปลี่ยนเท่านั้น สูงสุด 2 นาที; เพิ่ม dependency `clipboard-win` 5.4 ซึ่งอยู่ใน build ผ่าน arboard รุ่นเดียวกันอยู่แล้ว); macOS `/usr/sbin/screencapture -i -x`; Linux/BSD ใช้ XDG Screenshot portal แบบ interactive (GNOME, KDE, wlroots, Flatpak) ถ้าไม่มี portal ใช้ `gnome-screenshot --area`, `spectacle --region`, `xfce4-screenshooter --region` หรือ `flameshot gui` ตัวแรกที่พบใน `PATH`
  - ข้อความแถบสถานะ 6 ข้อความแปลครบ 18 ภาษา (ไทย: ภาพหน้าจอ, การตัดหน้าจอ, เลือกส่วนของหน้าจอที่จะแทรก ฯลฯ)
  - ผลต่อผู้ใช้: ไม่เปลี่ยนรูปแบบไฟล์ (ภาพที่แทรกเป็นรูปภาพธรรมดาใน .docx) ติดตั้งทับได้ เว็บไม่มีการตัดหน้าจอ (แสดงข้อความให้คัดลอกภาพหน้าจอแล้ววาง)
  - ตรวจแล้ว: เทสต์ใหม่ `screen_clip::tests` ใน ui-egui (แทรกเป็น undo step เดียว, เริ่มแล้วไม่สร้าง undo/ไม่ dirty, ยกเลิก, ข้อมูลไม่ใช่ภาพ, เริ่มไม่ได้, host ปิด channel, ไม่มี hook) และใน app (แปลง `file://` เป็น path รวม `%XX` และภาษาไทย, โฟลเดอร์ชั่วคราวสิทธิ์ 0700 ถูกลบพร้อมไฟล์); `cargo test -p doc4me-ui-egui` 329 ผ่าน (+ audit แปลภาษา `--ignored` ผ่าน), `cargo test -p doc4me-engine` 383 + 13 + 8 ผ่าน, `cargo clippy -p doc4me --target x86_64-pc-windows-msvc` และ `--target x86_64-apple-darwin` `-D warnings` ผ่าน, clippy Linux `--all-targets` ผ่าน, `cargo xtask parity` 490/500

- **มุมมอง › หน้าต่าง (View › Window) ครบทั้งกลุ่ม: หน้าต่างใหม่, จัดเรียงทั้งหมด, แบ่ง, ดูเคียงข้างกัน, การเลื่อนแบบซิงโครนัส และสลับหน้าต่าง** (ผู้ใช้สั่งให้ทำ parity ที่เหลือ; `crates/ui-egui/src/windows.rs` ใหม่, `crates/engine/src/cmd/view.rs`)
  - เดิม `view.newWindow` และ `view.split` เป็น stub: ส่งคำขอ (`{"open": "newWindow"}`, `{"toggle": "split"}`) ที่ไม่มีส่วนไหนของโปรแกรมรับ กดแล้วไม่มีอะไรเกิด และปุ่ม Arrange All บนริบบอนอ้างคำสั่ง `view.arrangeAll` ที่ไม่มีอยู่
  - แบ่ง (Split, Ctrl+Alt+S): พื้นที่เอกสารแบ่งเป็นบนล่าง แต่ละบานมีไม้บรรทัดและการเลื่อนของตัวเอง ใช้การเลือกเดียวกัน (แบบ Word) ลากแถบกลางเพื่อย้ายเส้นแบ่ง ดับเบิลคลิกแถบหรือกดแบ่งอีกครั้งเพื่อเอาออก
  - หน้าต่างใหม่ (New Window): เปิดอีกหน้าต่างของเอกสารเดียวกัน ชื่อ "ชื่อเอกสาร:2", ":3"… เลื่อนได้อิสระและแก้ไขได้ (เอกสารและการเลือกใช้ร่วมกัน) ใช้ viewport ของ egui: บนเดสก์ท็อปเป็นหน้าต่างจริง ปิดได้ด้วยปุ่มปิด ส่วนที่ host เปิดหลายหน้าต่างไม่ได้ (เว็บ เทสต์) แสดงเป็นหน้าต่างในหน้าต่างหลัก; แต่ละมุมมองมี `CanvasState` ของตัวเอง สลับเข้า `WordApp::canvas` ระหว่างวาด
  - จัดเรียงทั้งหมด (`view.arrangeAll`): วางทุกหน้าต่างซ้อนบนล่างเต็มความกว้างจอ ดูเคียงข้างกัน (`view.sideBySide`): วางสองหน้าต่างแรกซ้ายขวา ถ้ามีหน้าต่างเดียวจะเปิดหน้าต่างที่สองให้เทียบ การเลื่อนแบบซิงโครนัส (`view.syncScroll`, เปิดเป็นค่าเริ่มต้น) เลื่อนหน้าต่างอื่นตามระยะเดียวกันขณะดูเคียงข้างกัน สลับหน้าต่าง (`view.switchWindows`) เป็นเมนูรายการหน้าต่างและนำหน้าต่างที่เลือกขึ้นหน้า
  - แป้นลัดของหน้าต่าง (เช่น Ctrl+B) ไม่ทำงานซ้ำเมื่อบานล่างหรือหน้าต่างอื่นรับแป้นพิมพ์อยู่ (`keys::global_shortcuts` ใช้ `any_canvas_focused`)
  - ริบบอน มุมมอง › หน้าต่าง มีปุ่ม ดูเคียงข้างกัน การเลื่อนแบบซิงโครนัส และสลับหน้าต่าง พร้อม keytip และไอคอนใหม่ (`ribbon.rs`, `keytips.rs`, `icons.rs`); คำแปลชื่อคำสั่ง 4 คำสั่งครบ 18 ภาษา; audit คำแปลยกเว้นชื่อหน้าต่างซึ่งมาจากชื่อเอกสาร (`i18n/audit.rs`)
  - parity: `cargo xtask parity` ได้ 489 จาก 500 (97.8%) แท็บมุมมองครบ 29/29 (`docs/parity-checklist.md`, `ROADMAP.md`); `docs/gaps.md` #27 ปิดแล้ว เหลือข้อต่างที่บอกในหัวข้อถัดไป
  - ยังไม่มี (ต่างจาก Word): หน้าต่างที่เปิดเพิ่มแสดงเฉพาะเอกสาร ไม่มีริบบอนของตัวเอง (ใช้ริบบอนของหน้าต่างหลัก); การเลือกใช้ร่วมกันทุกหน้าต่าง; จัดเรียงใช้จอที่เริ่มที่มุมซ้ายบนของจอหลัก
  - ผู้ใช้และความเข้ากันได้: ไม่มีผลต่อไฟล์ .docx หรือการติดตั้ง
  - ตรวจสอบ: เทสต์ใหม่ `window_commands_open_split_and_switch`, `side_by_side_opens_a_second_window_and_scrolls_together`, `arrange_all_stacks_and_side_by_side_halves` และ `split_and_other_windows_draw_their_own_views` (วาดจริงด้วย egui_kittest) ผ่าน; ui-egui 328 รายการผ่าน (`--include-ignored`); ภาพหน้าจอจาก `ui_shot` แสดงสองบานที่มีไม้บรรทัดของตัวเองและหน้าต่าง "The Open Studio Handbook:2"; clippy `-D warnings` ผ่าน

- **ปุ่มแปลงเลขอารบิกเป็นเลขไทย และแปลงเลขไทยเป็นเลขอารบิก บนริบบอน หน้าหลัก › ฟอนต์ (Home › Font)** (ถัดจากปุ่มเปลี่ยนตัวพิมพ์เล็ก-ใหญ่)
  - เหตุผล: ผู้ใช้สั่งเพิ่ม เอกสารราชการไทยมักใช้เลขไทย แต่ Word ไม่มีคำสั่งนี้ (ต้องใช้ค้นหาและแทนที่ทีละตัวหรือ macro)
  - คำสั่งใหม่ `format.thaiDigits` "Convert to Thai Digits" (0–9 → ๐–๙) และ `format.arabicDigits` "Convert to Arabic Digits" (๐–๙ → 0–9) ใน `crates/engine/src/cmd/format.rs` (`convert_digits`, `swap_digit`)
  - ปุ่มเล็กสองปุ่มใน `crates/ui-egui/src/ribbon.rs` ไอคอน `thaidigits` (1 → ๑) และ `arabicdigits` (๑ → 1) วาดใน `crates/ui-egui/src/icons.rs`
  - การทำงาน
    - แปลงเฉพาะส่วนที่เลือก ถ้าไม่ได้เลือกจะแปลงคำหรือตัวเลขที่เคอร์เซอร์อยู่ (เหมือนเปลี่ยนตัวพิมพ์เล็ก-ใหญ่) อักขระอื่นไม่เปลี่ยน
    - แต่ละ run คงรูปแบบเดิม การเลือกยังครอบข้อความที่แปลงแล้ว (เลขไทยยาว 3 ไบต์ใน UTF-8) และเลิกทำได้ในขั้นเดียว
    - การเลือกแบบคอลัมน์แปลงทีละแถว (`cmd/column.rs`)
    - เมื่อเปิดติดตามการเปลี่ยนแปลง ตัวเลขเดิมถูกทำเครื่องหมายลบและเลขใหม่เป็นข้อความแทรก ยอมรับหรือปฏิเสธได้ ข้อความที่ถูกลบไว้แล้วไม่ถูกแตะ ส่วนตัวเลขที่ผู้เขียนคนเดียวกันแทรกไว้ถูกแทนที่ตรงๆ
    - ไม่แปลงตัวเลขใน content control ที่ล็อกการแก้เนื้อหา หรือที่ยังแสดงข้อความแนะนำ
    - การจำกัดการจัดรูปแบบของ Restrict Editing ไม่ปิดสองคำสั่งนี้ เพราะเปลี่ยนข้อความไม่ใช่รูปแบบ (`cmd/protect.rs` เหมือนเปลี่ยนตัวพิมพ์เล็ก-ใหญ่) ส่วนเอกสารที่ป้องกันแบบอ่านอย่างเดียวยังกันการแก้ไขตามเดิม
  - คำแปลชื่อคำสั่งครบ 18 ภาษาใน `crates/ui-egui/src/i18n/*.tsv` (ไทย: "แปลงเป็นเลขไทย" และ "แปลงเป็นเลขอารบิก")
  - ภาพ `docs/images/hero.png`, `thai.png`, `dark.png`, `dark-th.png`, `font.png` และ `font-th.png` (เห็นริบบอนหน้าหลัก) ถ่ายใหม่ด้วยสคริปต์เดิมใน `docs/images/shots/` เพราะกลุ่มฟอนต์มีปุ่มเพิ่ม
  - ตรวจด้วยภาพหน้าจอ (`ui_shot`, ภาษาไทย): ปุ่ม 1→๑ และ ๑→1 อยู่ถัดจากปุ่มเปลี่ยนตัวพิมพ์เล็ก-ใหญ่ และย่อหน้าที่เลือกแปลงเป็น ๒๕๖๗ ๑๒ ๑,๒๕๐ โดยการเลือกยังครอบทั้งย่อหน้า
  - รอบตรวจหลัง push: `cargo test -p doc4me-engine` ผ่าน 377 + 13 + 8 (หลังแก้ UTC), ui-egui ทั้งชุดผ่าน 323 รายการ (`--include-ignored`), clippy `-D warnings` `--all-targets` ของ engine, ui-egui และ xtask ผ่าน
  - ตรวจหลัง push (คอมมิต `21bb34a`): ตำแหน่งของอักขระที่ใช้เทียบกับ content control ที่ล็อกถูกสร้างทุกตัวอักษร (คัดลอก path ทุกครั้ง) เปลี่ยนให้สร้างเฉพาะตัวเลขในเอกสารที่มี control ที่ต้องข้าม ผลลัพธ์เหมือนเดิม
  - ตัวเลขที่โปรแกรมสร้างเองเปลี่ยนตามด้วย (ผู้ใช้สั่งเพิ่ม, `convert_generated_numbers`): เลขลำดับรายการของย่อหน้าในส่วนที่เลือกหรือที่เคอร์เซอร์อยู่ (ทุกระดับของรายการนั้นที่เป็นเลขอารบิกหรือเลขไทย และรายการที่ใช้นิยามเดียวกัน แบบที่ Define New Multilevel List เปลี่ยน) และเลขหน้า เลขเชิงอรรถ และเลขอ้างอิงท้ายเรื่องของ section ที่ส่วนที่เลือกอยู่ (ถ้าอยู่ในหัวกระดาษหรือท้ายกระดาษ คือ section ที่ใช้หัวหรือท้ายกระดาษนั้น); รูปแบบอื่น เช่น เลขโรมันหรือตัวอักษร ไม่เปลี่ยน; ทำงานแม้เคอร์เซอร์ไม่ได้อยู่ในคำ; ตรวจด้วยเทสต์ `digit_conversion_converts_list_page_and_note_numbers` และ `digit_conversion_in_a_footer_converts_the_page_numbers`
  - ตรวจหลัง push (`c4777f3`): ย่อหน้าที่ได้เลขลำดับจากสไตล์ (เช่นหัวข้อที่มีเลข) ไม่ถูกจัดวางใหม่ จึงอาจยังแสดงเลขเดิมจนกว่าจะแก้ย่อหน้านั้น แก้ให้จัดวางย่อหน้าของสไตล์ที่ผูกกับรายการนั้นด้วย (`touch_styles`)
  - เทสต์เดิมของปุ่มนี้และของ JPEG ใช้ `document.setText` กับ `\r` ซึ่งไม่ได้แยกย่อหน้า (คำสั่งนี้แยกด้วย `\n`) แก้ให้เป็นหลายย่อหน้าจริงแล้ว
  - ไม่ได้เพิ่มลงแค็ตตาล็อก parity (`crates/engine/src/catalog.rs`) เพราะ Word ไม่มีคำสั่งนี้ ตัวเลข parity จึงไม่เปลี่ยน
  - ผู้ใช้และความเข้ากันได้: ไฟล์ .docx เดิมเปิดและบันทึกเหมือนเดิม; ตัวเลขที่แปลงแล้วเป็นอักขระ U+0E50–U+0E59 จริงในไฟล์ ต่างจาก File › Options › Numerals ซึ่งเปลี่ยนแค่การแสดงผล; ติดตั้งทับรุ่นเดิมได้ตามปกติ
  - ตรวจสอบ: เทสต์ใหม่ `digits_convert_between_arabic_and_thai`, `digit_conversion_keeps_each_runs_formatting`, `digit_conversion_under_track_changes_is_a_revision` (`crates/engine/src/tests.rs`) และ `digit_conversion_skips_controls_that_cant_be_edited` (`cmd/controls.rs`) ผ่าน; `cargo test -p doc4me-engine` ผ่าน 376 + 13 + 8; `cargo test -p doc4me-ui-egui --lib -- --include-ignored i18n ribbon icons widgets` ผ่าน 82 รายการ (รวม `every_tab_and_command_is_translated`, การตรวจว่าแค็ตตาล็อกทุกภาษามีคีย์ชุดเดียวกัน และ audit `untranslated_interface_text`); `cargo fmt --all --check` ผ่าน

- **บันทึกเป็น (Save As) และส่งออก (Export) หน้าเอกสารเป็นไฟล์ภาพ JPG/JPEG**
  - เหตุผล: ผู้ใช้สั่งเพิ่ม เดิมส่งออกเป็นภาพได้แค่ PNG
  - คำสั่งใหม่ `file.exportJpeg` "Export Page as JPEG" (`crates/engine/src/cmd/file.rs`, รวมกับ PNG ใน `export_page`): พารามิเตอร์ `path`, `page`, `scale`, `quality` (1–100 ค่าเริ่มต้น 90 ค่านอกช่วงถูกบีบ) และ `markup`; path ที่ไม่ลงท้าย `.jpg` หรือ `.jpeg` (ตัวพิมพ์ใดก็ได้) จะต่อ `.jpg` ให้
  - บันทึกเป็น: `.jpg` และ `.jpeg` อยู่ใน `SAVE_EXTS` (`crates/engine/src/io.rs`) จึงอยู่ในกล่อง Save As ของระบบ และ `io_ext::save` เขียนหน้า 1 ที่ 2 px/pt คุณภาพ 90 (`render_jpeg`, `JPEG_QUALITY`) เป็นสำเนาเหมือน PNG: เอกสารยังมี path เดิมและยังนับว่าไม่ได้บันทึก
  - File › Export และหน้าพิมพ์มีปุ่ม "Page image (*.jpg, *.jpeg)" (`crates/ui-egui/src/print_page.rs`) เสนอชื่อ `.jpg` แล้วเรียก `file.exportJpeg` (`file_dialogs.rs`); บนเว็บ การส่งออกและบันทึกเป็น JPEG กลายเป็นการดาวน์โหลด (`lib.rs`, MIME `image/jpeg` ใน `apps/doc4me-web` มีอยู่แล้ว)
  - JPEG ไม่มีความโปร่งใส หน้าจึงวาดบนพื้นขาว (`Rendered::to_jpeg` ใน `crates/render/src/lib.rs`); ถ้าเข้ารหัสไม่ได้ (เช่นกว้างหรือสูงเกิน 65,535 px) จะแจ้งข้อผิดพลาดแทนการเขียนไฟล์เสีย ซึ่งใช้กับ PNG ด้วย (เดิม `to_jpeg` ไม่ถูกเรียกใช้ และไม่สนใจข้อผิดพลาด)
  - `doc4me-cli`: `convert`, `render` และ `run --save` เลือก JPEG ตามนามสกุล `.jpg`/`.jpeg` (`image_export`) และข้อความ USAGE บอกแล้ว; MCP `save_document` ส่ง `.jpg`/`.jpeg` ไป `file.exportJpeg` และ `file.exportJpeg` เขียนได้เฉพาะใน `--automation-write-root` เหมือนคำสั่งเขียนไฟล์อื่น (`crates/engine/src/io.rs` `write_file`, `docs/mcp.md`)
  - คำแปล "Export Page as JPEG" และ "Page image (*.jpg, *.jpeg)" ครบ 18 ภาษา (ไทย: "ส่งออกหน้าเป็น JPEG", "ภาพหน้า (*.jpg, *.jpeg)") ใช้คำเดียวกับของ PNG
  - ไม่ได้เพิ่มลงแค็ตตาล็อก parity เพราะ Word ไม่มีคำสั่งนี้
  - ผู้ใช้และความเข้ากันได้: ไฟล์ .docx เดิมไม่เปลี่ยน; ภาพ JPEG เป็นสำเนาของหน้าเดียว เปิดกลับมาแก้เป็นเอกสารไม่ได้ (เหมือน PNG); ติดตั้งทับได้ตามปกติ; ไม่ได้ออกเวอร์ชันใหม่ตามที่ผู้ใช้สั่ง
  - `README.md`, `README.th.md` และบันทึกความคืบหน้าใน `ROADMAP.md` บอกว่าส่งออกภาพหน้าเป็น PNG และ JPG/JPEG ได้
  - ตรวจสอบ: เทสต์ใหม่ `pages_export_and_save_as_jpeg` (`crates/engine/src/tests.rs`: ไฟล์ถอดรหัสเป็น JPEG ได้ ขนาดตรงกับที่รายงาน มุมหน้าเป็นสีขาว ต่อ `.jpg` ให้ path ที่ไม่มี คง `.JPEG` ไว้ quality ถูกบีบและเปลี่ยนขนาดไฟล์ หน้าที่ไม่มีเป็นข้อผิดพลาด Save As `.jpg`/`.jpeg` เป็นสำเนา), `export_writes_a_jpeg_page_image` (`crates/ui-egui/src/file_dialogs.rs`), `page_images_export_by_extension` (`apps/doc4me-cli`) และ `file.exportJpeg` ในเทสต์ write root ของ MCP (`crates/mcp/tests/automation_roots.rs`); `cargo test -p doc4me-engine` ผ่าน 378 + 13 + 8; `cargo test -p doc4me-cli -p doc4me-mcp -p doc4me-render` ผ่านทั้งหมด; `cargo fmt --all --check` ผ่าน; `cargo test -p doc4me-ui-egui --lib -- --include-ignored` ผ่าน 324 รายการ; `cargo clippy -p doc4me-engine -p doc4me-ui-egui -p doc4me-cli -p doc4me-mcp -p doc4me-render -p xtask --all-targets -- -D warnings` ผ่าน

### เปลี่ยนแปลง

- **แผนความพร้อมใช้งานจริง 80% แบบวัดได้ และ roadmap เป็นปัจจุบัน** (`docs/roadmap.md` หัวข้อใหม่ "Path to 80% measured readiness", `ROADMAP.md` Upcoming ข้อ 0 และ 1, `docs/target-app-parity.md`, `handoff.md`)
  - เหตุผล: ผู้ใช้สั่ง (2026-10-25) ให้สรุปงานที่ต้องทำให้พร้อมใช้งานจริง 80% แบบไม่ใช่ค่าประมาณ ตัวเลข ~60% เดิมเป็นผลรวมถ่วงน้ำหนักของค่าที่ประมาณ จึงวางแผนเป็นสามขั้น: สร้างเครื่องวัด (`cargo xtask readiness` บนชุดตรวจผ่าน/ไม่ผ่านที่ตรึงไว้: คลัง DOCX จริงที่มี PDF อ้างอิงจาก Word, การแบ่งหน้า, ความเสถียร, รายการฟีเจอร์และ UI ของ Word for Windows, ประสิทธิภาพ ภาษา แพลตฟอร์ม), วัดครั้งแรก, แล้วแก้ตามผลจนได้ ≥ 80%; handoff มีรายการงานภาษาไทยฉบับเดียวกัน
  - ROADMAP Upcoming ข้อ 1 กลับไปยึดการติดตาม WordCraft ต้นทาง (ผู้ใช้สั่งให้ติดตามต่อ) และระบุ PR ของ Dependabot 4 รายการของ repo นี้; ตัดข้อความ "lines wrap differently without an Aptos-metric font" ในส่วนลดของ readiness เพราะ Intos แก้แล้ว
  - ผู้ใช้: ไม่มีผลต่อโปรแกรม ไฟล์ .docx หรือ installer (ยังไม่ได้สร้างเครื่องวัด ตัวเลขใน ROADMAP ยังเป็นค่าประมาณ)
  - ตรวจสอบ: ลิงก์ `docs/roadmap.md#path-to-80-measured-readiness` ตรงกับหัวข้อ; น้ำหนักรายมิติตรงกับสูตรใน `docs/target-app-parity.md` (รวม 1.00)
- **ระบบเสริมที่เป็นของ Microsoft และผู้ให้บริการ AI ในโปรแกรมออกจากเป้าหมาย parity; ระบุตัวทดแทน open-source** (`docs/gaps.md` #22 และ #25, `ROADMAP.md`, `docs/target-app-parity.md`, `docs/roadmap.md`)
  - เหตุผล: ผู้ใช้สั่ง (2026-10-25) ให้ตัดระบบเสริมที่เป็นลิขสิทธิ์ของ Microsoft Office ทั้งหมดแล้วใช้ open-source แทน และตัดผู้ให้บริการ AI ในโปรแกรม
  - ตัดออกจากเป้าหมาย: การรัน VBA, Add-in ของ Office (Office.js) และ Office Store, OneDrive/SharePoint, การแก้ไขร่วมกันแบบเรียลไทม์ของ Microsoft และผู้ช่วย AI ในโปรแกรม (แบบ Copilot)
  - ตัวทดแทน open-source ที่มีอยู่แล้ว: แมโครของ Doc4Me (`tools.macros`, `tools.recordMacro` บันทึกและเล่นคำสั่ง), แค็ตตาล็อก Add-in ของ Doc4Me เอง (`addins.get`), การเก็บไฟล์บนคลาวด์ผ่านโฟลเดอร์ที่ Nextcloud Desktop หรือ Syncthing ซิงก์ให้ (Doc4Me เปิดและบันทึกไฟล์ในโฟลเดอร์นั้นได้ตามปกติ), ส่วนเอเจนต์ AI ภายนอกใช้ MCP และ CLI ได้เหมือนเดิม การแก้ไขร่วมกันแบบเรียลไทม์ไม่มีตัวทดแทนที่ใส่ในโปรแกรมได้ จึงตัดออก
  - ไม่มีโค้ดที่ผูกกับบริการของ Microsoft ให้ลบ (`file.share` เปิด Save As อยู่แล้ว); VBA ในไฟล์ `.docm` ยังเก็บไว้ในไฟล์ตามเดิม (ไม่รัน) เพื่อไม่ให้ข้อมูลในไฟล์ของผู้ใช้หาย
  - แก้เพิ่ม: `docs/hardware-parity.md` แถวปากกาและ Dictate ยังเป็น ❌ ทั้งที่แท็บวาด, Ink to Math และ Dictate (whisper.cpp) มีแล้ว จึงแก้ให้ตรงกับโค้ด (ยังไม่ได้วัดค่า parity ใหม่)
  - ผู้ใช้: ไม่มีผลต่อโปรแกรม ไฟล์ .docx หรือ installer
  - ตรวจสอบ: `grep` หาโค้ดที่อ้าง OneDrive, SharePoint, Office.js, Copilot หรือผู้ให้บริการ AI ใน `crates` และ `apps` ไม่พบ (มีแค่ชื่อกลุ่ม "SharePoint" ของ Word ใน catalog ที่เป็นคุณสมบัติเอกสาร); คำสั่งแมโครและ `addins.get` มีใน `crates/engine/src/catalog.rs`
- **TH SarabunIT๙ ไม่ใช้ค่าที่อนุมานอีกต่อไป: เมื่อเครื่องไม่มีฟอนต์นี้ แสดงด้วย Sarabun ขนาดของ Sarabun เอง และแจ้งผู้ใช้ตอนเปิดไฟล์** (`crates/fonts/src/word.rs`, `crates/layout/src/para.rs`, `crates/layout/src/display.rs`, `crates/engine/src/cmd/file.rs`, `crates/ui-egui/src/font_notice.rs` ไฟล์ใหม่, `crates/ui-egui/src/lib.rs`, `crates/ui-egui/src/i18n/*.tsv` 18 ไฟล์, เทสต์ใน `crates/layout/src/tests_complex.rs`, `crates/engine/src/tests.rs`)
  - เหตุผล: ผู้ใช้สั่ง (2026-10-25) ให้ตัดการวัดค่า TH SarabunIT๙ จากไฟล์ฟอนต์จริงออก ไม่อ้างอิงฟอนต์นี้ และเมื่อไฟล์ที่เปิดใช้ฟอนต์นี้แต่เครื่องไม่มี ให้แทนด้วย Sarabun และแจ้งเตือนผู้ใช้ เดิม (1.1.0) Doc4Me วาด IT๙ ด้วย Sarabun บนความกว้างและระยะบรรทัดของ TH Sarabun New ซึ่ง "อนุมาน" ไม่ได้วัดจากไฟล์ IT๙ และแสดงเลขอารบิกเป็นเลขไทย
  - เปลี่ยน: `metric_fit` รับเฉพาะ TH Sarabun New และ TH SarabunPSK (ค่าที่วัดจากไฟล์จริง); ลบ `MetricFit::shown` และการแสดงเลขอารบิกเป็นเลขไทย; IT๙ จึงเป็นฟอนต์ไทยที่ Sarabun แทนที่ขนาดของ Sarabun เอง เหมือน Angsana New หรือ Cordia New
  - เพิ่ม: `file.open` ตรวจฟอนต์ในสไตล์และข้อความทุก story (เนื้อหา หัว/ท้ายกระดาษ เชิงอรรถ กล่องข้อความ) ด้วย `cmd::file::unmatched_fonts` ถ้าใช้ TH SarabunIT๙ (เขียน ๙ หรือ 9, ช่องว่างหรือตัวพิมพ์ใดก็ได้) และเครื่องไม่มี จะคืน `substitutedFonts` ในผลลัพธ์ (CLI/MCP เห็น) และส่งคำขอ `fontsSubstituted` ให้หน้าจอ ซึ่งแสดงข้อความในแถบสถานะและหน้าต่าง "ไม่มีฟอนต์นี้ในเครื่อง" ที่อยู่จนผู้ใช้กดตกลงหรือปิด ข้อความแปลครบ 18 ภาษา
  - ผู้ใช้: เอกสาร IT๙ บนเครื่องที่ไม่มีฟอนต์ ตัวอักษรใหญ่ขึ้นราว 1.47 เท่า (ขนาดของ Sarabun) จึงขึ้นบรรทัดและแบ่งหน้าต่างจาก Word และเลขอารบิกแสดงเป็นเลขอารบิกตามข้อความจริง; เครื่องที่ติดตั้ง IT๙ ไว้ใช้ฟอนต์จริงเหมือนเดิมและไม่มีการแจ้งเตือน; ไฟล์ .docx ไม่เปลี่ยน ชื่อฟอนต์ในไฟล์ยังเป็น TH SarabunIT๙ เมื่อบันทึก; TH Sarabun New และ PSK ไม่เปลี่ยน
  - ตรวจสอบ: `cargo test -p doc4me-fonts` (52 ผ่าน), `-p doc4me-layout` (224 ผ่าน), `-p doc4me-engine` (400 + 13 + 8 ผ่าน รวม `opening_a_document_in_an_unmatched_font_tells_the_user` ที่บันทึก .docx ที่ใช้ IT๙ แล้วเปิดใหม่), `-p doc4me-ui-egui` (345 ผ่าน รวม `substituted_fonts_are_reported_until_closed` และเทสต์คำแปลครบทุกภาษา); `cargo clippy -p doc4me-fonts -p doc4me-layout -p doc4me-engine -p doc4me-ui-egui --all-targets -- -D warnings` ผ่าน; `cargo fmt --check` ผ่าน
- **`.gitignore` ไม่ละเว้น `/.claude/` แล้ว** (`.gitignore`)
  - เหตุผล: ผู้ใช้สั่ง (2026-10-25) ให้เอาออก บรรทัดนี้ทำให้ไฟล์ใหม่ใต้ `.claude/` (เช่นการตั้งค่า Claude Code ของโปรเจกต์) ไม่ถูก git เห็น ส่วนบรรทัดอื่น (`/target`, `/craft-fonts/`, `plan/`, `/corpus` ฯลฯ) คงไว้เพราะกันผลการ build และไฟล์ภายนอกไม่ให้ถูก commit
  - ผลต่อผู้พัฒนา: ไฟล์ส่วนตัวใต้ `.claude/` เช่น `settings.local.json` จะขึ้นเป็นไฟล์ untracked ต้องระวังไม่ให้ commit
  - ผู้ใช้: ไม่มีผลต่อโปรแกรม ไฟล์ .docx หรือ installer
  - ตรวจสอบ: `git status --short` หลังแก้ไม่มีไฟล์ใหม่โผล่ (ไม่มีโฟลเดอร์ `.claude/` ใน repo); `cargo xtask assets` ผ่าน
- เอกสารที่ไม่ตรงกับโค้ด (งานค้างใน handoff ข้อ "เอกสารล้าสมัย") ตรวจกับโค้ดแล้วแก้: `docs/file-format-parity.md` แถวการเข้ารหัสเคยบอก ❌ ทั้ง .docx/.doc แต่ `crates/docx/src/crypt.rs` เปิดและบันทึก .docx ที่เข้ารหัส (agile, [MS-OFFCRYPTO]) ได้ จึงแยกเป็นสองแถว (.docx ✅ R/W, .doc ❌ ตรวจพบแล้วปฏิเสธ); `docs/roadmap.md` แถว M1 (chart/SmartArt/OLE อ่านและเขียนกลับแล้ว ไม่ได้ทิ้ง), M2 (Paste Special, Clipboard pane, Style Inspector, Manage Styles, Asian Typography มีคำสั่งแล้ว), M3 และ M8 (ทำแล้วเว้นรายการที่ต้องใช้บริการออนไลน์), M11 (`arrange.group` มีแล้ว), M12 และจำนวนคำสั่งใน alpha gate 609 → 618 (นับด้วย `doc4me-cli commands --json`); `docs/mcp.md` บอก `list_commands` มี "over 270" → over 600 (618); `packaging/windows/package.ps1` อ้าง `.github/workflows/windows-arm64.yml` ที่ไม่มี; `packaging/env.sh` และ `ATTRIBUTION.md` บอกว่า release workflow ตั้ง `CRAFT_FONTS_DIR` / "ทุก release ทางการ" ฝัง craft-fonts แต่ workflow ทั้งปัจจุบันและที่ tag `v1.1.1` ไม่ได้ตั้ง (ตรวจไฟล์ที่ tag ผ่าน GitHub) จึงแก้ให้ตรงว่ารุ่น Windows 1.1.0/1.1.1 ไม่มีฟอนต์ CJK ของ craft-fonts — เปลี่ยนเฉพาะเอกสารและคอมเมนต์ ไม่กระทบโปรแกรม
- **เอกสาร parity เทียบกับ Word for Windows แทนเมนูและอุปกรณ์ของ Mac** (ผู้ใช้สั่งแก้ข้อจำกัดที่เหลือ)
  - `docs/target-app-parity.md`: หัวข้อ What we measured against บอกว่าเป้าหมายตั้งแต่ 2026-10-24 คือ Word for Windows และอะไรนับตามแบบ Windows (ribbon และ Backstage แทนแถบเมนูของ Mac, Ctrl แทน ⌘, File › Print ใน Backstage แทน `WordPDE.plugin`, PDF Reflow มีเฉพาะ Windows, Continuity Camera มีเฉพาะ Mac) ส่วนค่าที่วัดกับ Word for Mac ยังเป็นค่าตั้งต้นจนกว่าจะวัดใหม่บน Windows; แถว Platforms บอกตามจริงว่า Doc4Me ออก Release เฉพาะ Windows x64 ส่วนสคริปต์ของแพลตฟอร์มอื่นมาจาก WordCraft
  - `docs/hardware-parity.md`: คอลัมน์ Word ของเครื่องพิมพ์อธิบาย File › Print ของ Word for Windows; แถวสแกนเนอร์และกล้องเป็นของ Word for Mac เท่านั้น จึงไม่นับเป็นช่องว่างเทียบกับ Word for Windows (ชั่วโมง 0 เหลือ 2–4 ชั่วโมงเฉพาะ build ของ macOS); ตัด `InkRender.bundle` ของ Mac ออกจากแถวปากกา
  - `docs/ui-parity.md`: Windows ขึ้นก่อนในแถว Platform conventions และ Narrator กับ NVDA ในแถว Screen readers
  - ยังไม่ได้ทำ (ต้องใช้เครื่อง Windows ที่มี Word): วัดตัวเลข parity และ alpha gate ใหม่บน Windows
  - ผู้ใช้: ไม่มีผลต่อโปรแกรม ไฟล์ .docx หรือการติดตั้ง
  - ตรวจสอบ: ข้อความเรื่อง Word for Windows ตรวจกับความรู้เรื่อง Backstage และหน้าพิมพ์ของ Word 365; `git diff --stat` แก้แค่สามไฟล์นี้
- **เอกสารและคำแนะนำในโปรแกรมใช้ทางลัดแบบ Windows แทน ⌘ ของ Mac** (ผู้ใช้สั่ง)
  - `docs/typing-parity.md` (⌘Z, ⌘B, ⌘Y เป็น Ctrl+Z, Ctrl+B, Ctrl+Y และตัด ⌃⇧ ลูกศรของ macOS), `docs/ui-parity.md` และ `ROADMAP.md` (Ctrl/⌘+click เป็น Ctrl+click), `docs/localization-th.md` (คู่มือรูปแบบเขียนชื่อแป้นแบบ Windows เช่น Ctrl+Alt+G และใช้ ⊞ เมื่อหมายถึงแป้นโลโก้ Windows)
  - ใช้ Ctrl แทน ⌘ ไม่ใช่ ⊞ เพราะทางลัดของ Word for Windows ใช้ Ctrl (เช่น Ctrl+Z) ส่วน ⊞ เป็นแป้นของระบบ (⊞+Z เป็นทางลัดของ Windows เอง ซึ่ง Windows 11 ใช้เปิด Snap layouts ไม่ใช่เลิกทำ)
  - คำแนะนำในโปรแกรมที่ฝัง ⌘ ไว้ตายตัวและแสดงบน Windows ด้วย: ปุ่มวาง ("Paste (⌘V)" ซึ่งบน Windows ขึ้น "Paste (⌘V) (Ctrl+V)") และเลขหน้าในแถบสถานะ ("Go To (⌘⌥G)") เปลี่ยนให้แสดงทางลัดตามแพลตฟอร์ม (`ribbon.rs`, `chrome.rs`: Ctrl+V และ Ctrl+Alt+G บน Windows, ⌘V และ ⌘⌥G บน macOS) และลบคีย์คำแปลเดิมสองคีย์ออกจากแค็ตตาล็อกทั้ง 18 ภาษา (จำนวนรายการเท่าเดิมเพราะเพิ่มคีย์ของ JPEG สองคีย์)
  - ผู้ใช้: ไม่มีผลต่อไฟล์ .docx หรือการติดตั้ง
  - ตรวจสอบ: `grep` ไม่พบ ⌘, ⌥, ⌃ ใน `ROADMAP.md` และ `docs/` นอกจากประโยคที่อธิบายว่าไม่ใช้; ui-egui 324 รายการผ่าน รวม audit คำแปลที่ตรวจว่าข้อความใน `tl!` ทุกอันมีในแค็ตตาล็อก; clippy ผ่าน
- **เอกสารที่ล้าสมัยหลังออก 1.1.1 และเป้าหมาย parity เป็น Word for Windows** (handoff ข้อ 2 และ 5.6)
  - หัวเอกสาร **Target** ของ `ROADMAP.md` และ `docs/` 10 ไฟล์ (architecture, file-format-parity, gaps, hardware-parity, layout-parity, localization-parity, roadmap, target-app-parity, typing-parity, ui-parity) เปลี่ยนจาก Word for Mac 16.113.4 เป็น "Microsoft Word (Microsoft 365) for Windows" โดยไม่ใส่เลขรุ่นเพราะยังไม่รู้ build ที่ใช้เทียบ และบอกว่าค่าที่วัดถึง 2026-10-21 วัดกับ Word for Mac และยังไม่ได้วัดใหม่บน Windows; alpha gate ใน `docs/roadmap.md` เปลี่ยนแพลตฟอร์มหลักเป็น Windows พร้อมบอกว่ายังไม่ได้รันใหม่
  - `docs/gaps.md`: #2 เหลือแค่ Word ink (`w14:contentPart`) เพราะ drawing canvas (`wpc`) อ่านเขียนได้แล้ว; #8 เพิ่มตัวตรวจตัวสะกดไทย; #9 เหลือแค่ `.doc` แบบ RC4/XOR เพราะ .docx ที่เข้ารหัสเปิดและบันทึกได้ (`crates/docx/src/crypt.rs`); #10 group มีแล้ว (`arrange.group`); #11 ฝรั่งเศสมีแล้ว (`fr.tsv`); #15 แหล่งอ้างอิงอ่านเขียน `b:Sources` ได้แล้ว; #17 `para.asianTypography` อยู่ใน catalog แล้ว; ตัดข้อความว่ามี PR เปิดอยู่ของ Clipboard pane, Manage Styles และ Group (มีคำสั่งครบแล้ว); เพิ่มแถว #30 ภาษาไทย (ยังไม่ตรวจไวยากรณ์, การตัดบรรทัดอาจต่างจาก Word, ไม่จำลอง ligature ทางเลือกของ TH Sarabun, ค่าของ TH SarabunIT๙ ใช้ค่าของ TH Sarabun New)
  - `docs/localization-parity.md`: 19 ภาษารวมไทย, จำนวนรายการ 2,040–2,458, ภาษาไทยมี 2,458 รายการ, ตัวตรวจภาษาอังกฤษ โปแลนด์ และไทย, ค่าของ IT๙ เป็นค่าอนุมาน และปุ่มแปลงเลข
  - `docs/target-app-parity.md`: หัวข้อ headline บอกว่าวัดใน WordCraft (คอมมิต `a99dddc` ไม่มีใน repo นี้), กล่อง Font ขาดแค่ stylistic set, number form และ number spacing, ตัวตรวจภาษาไทย, การแลกไฟล์กับ Word (chart/SmartArt/OLE เขียนกลับได้แล้ว column balancing และ footnote ต่อหน้ามีแล้ว), 19 ภาษา UI
  - `docs/architecture.md`: ตัวตรวจภาษาไทย และส่วน CI ตรงกับ workflow เดียวที่มีจริง (ไม่มี CI บน PR หรือ branch `release`)
  - `docs/roadmap.md`: 609 คำสั่ง, M14 เป็น Release ของ Doc4Me (ไม่ใช่ v0.1.0–v0.4.0 ของ WordCraft), M15 19 ภาษาและตัวตรวจภาษาไทย, ตัด column balancing กับ footnote continuation ออกจากงานที่เหลือ
  - `ROADMAP.md`: แถว Releases เป็น v1.1.1, ตัวเลข 609 คำสั่ง 1,758 เทสต์ ~181,000 บรรทัด (นับจากซอร์ส), Why alpha / Why not beta ไม่อ้างว่ามี build ที่ลงนามทุกแพลตฟอร์มและไม่นับฟีเจอร์ที่มีแล้วว่ายังขาด, จำนวนรายการของ tr (2,414) และ hu (2,446), Upcoming ข้อ 3, 5, 6 และบันทึกความคืบหน้า 2026-10-22 กับ 2026-10-24
  - `docs/localization-hu.md`, `docs/localization-tr.md`: Target "Doc4Me 0.5.0" ซึ่งเป็นรุ่นที่ไม่เคยมี เปลี่ยนเป็น Doc4Me 1.1
  - `README.th.md`: หัวข้อ "ภาษาไทยใน Doc4Me" เพิ่มปุ่มจัดกระจายแบบไทยของ 1.1.1 และปุ่มแปลงเลข
  - ตัวเลข parity ของ catalog: `cargo xtask parity` (2026-10-24) ได้ 485 จาก 500 (97.0%) และแท็บ File 22/22 เพราะ catalog มี `File|Options|Numerals|view.numeral` แล้ว แก้ `docs/parity-checklist.md` (สร้างใหม่โดยคำสั่งนั้น) และแถวแรกของ `ROADMAP.md` (เดิม 484/499 แก้ด้วยมือตอน 1.1.1)
  - ยังไม่ได้ทำ: ข้อความที่อ้าง PR และ issue ของ WordCraft รอเจ้าของ repo ตัดสินใจ (handoff ข้อ 5.7); แถวในเอกสาร parity ที่เขียนทางลัดแบบ ⌘ ของ Mac ยังไม่ได้เขียนใหม่สำหรับ Windows
  - ผู้ใช้: ไม่มีผลต่อโปรแกรม ไฟล์ .docx หรือ installer
  - ตรวจสอบ: ข้อเท็จจริงแต่ละข้อตรวจกับโค้ดด้วย grep (`crypt.rs`, `b:Sources` ใน `crates/docx/src/read/mod.rs` และ `write/mod.rs`, `arrange.group`, `edit.clipboardPane`, `styles.manage`, `wpc` ใน `crates/docx/src/xml.rs`, ไม่มี `contentPart`, การ balance คอลัมน์และการแบ่ง footnote ใน `crates/layout/src/lib.rs`); จำนวนรายการนับจากไฟล์ `.tsv`; จำนวนคำสั่งนับ id ของ `CommandSpec::new` ที่ไม่ซ้ำ; `git diff --stat` ตรวจว่าแก้แค่บรรทัดที่ตั้งใจ

- **CI ลงนามโค้ดได้เมื่อมี secret และ build โดยไม่มี secret ใน environment** (`.github/workflows/windows-x64-installer.yml`, `packaging/windows/package.ps1`, `docs/releasing.md` ข้อ 2)
  - เหตุผล: workflow ไม่เคยส่ง `secrets.*` ให้ `packaging/windows/sign.ps1` จึงไม่ลงนามแม้เจ้าของ repo จะใส่ secret แล้ว (handoff ข้อ 5.2 ผู้ใช้ตัดสินใจให้ลงนาม)
  - แยก step เดิมเป็นสองขั้น: "Build doc4me + doc4me-cli" (`package.ps1 -BuildOnly` สวิตช์ใหม่ build แล้วหยุด ไม่มี secret) และ "Sign and package MSI + portable zip" (`package.ps1 -SkipBuild`) ซึ่งได้ `WINDOWS_CERTIFICATE`, `WINDOWS_CERTIFICATE_PASSWORD`, `AZURE_TENANT_ID`, `AZURE_CLIENT_ID`, `AZURE_CLIENT_SECRET`, `AZURE_SIGNING_ENDPOINT`, `AZURE_SIGNING_ACCOUNT` และ `AZURE_CERT_PROFILE` จาก `secrets.*` ชื่อตาม `sign.ps1:7-10`
  - การแยกทำให้ build script และ proc-macro ของ dependency อ่าน secret ไม่ได้ ตัวเลือกและ environment ของ `cargo build` เหมือนเดิม (`--release --locked`, `+crt-static`, `DOC4ME_REQUIRE_WINRES`)
  - secret ที่ยังไม่ได้ใส่อ่านได้ค่าว่าง `sign.ps1` ตรวจด้วย `[bool]$env:WINDOWS_CERTIFICATE` และต้องมี `AZURE_*` ครบหกค่า จึงยังข้ามการลงนามพร้อมคำเตือน "leaving unsigned" เหมือนเดิม
  - `docs/releasing.md`: บอกชื่อ secret ที่ตั้งใน Settings › Secrets and variables › Actions, ข้อจำกัดของใบรับรองที่ออกตั้งแต่ 2023-06-01 (กุญแจต้องอยู่ในฮาร์ดแวร์ ส่วนใหญ่ส่งออกเป็น .pfx ไม่ได้) และวิธีตรวจหลังใส่ secret; ตัวอย่างเวอร์ชันเปลี่ยนเป็น x.y.z, ใช้ `git tag -a`, เพิ่มขั้นแก้ลิงก์ `[Unreleased]` เป็น `compare/vx.y.z...HEAD` และขั้นรัน `cargo xtask ci`, audit ที่ติด `#[ignore]` และ `cargo audit` ก่อนออกรุ่น
  - ผู้ใช้: ตอนนี้ยังไม่มี secret ไฟล์ MSI และ zip จึงยังไม่ลงนามเหมือนเดิม เมื่อเจ้าของ repo ใส่ secret แล้ว `doc4me.exe`, `doc4me-cli.exe` และ MSI จะลงนาม การติดตั้งทับรุ่นเดิมไม่เปลี่ยน (`UpgradeCode` เดิม)
  - ตรวจสอบ: อ่าน `sign.ps1:48-56` และ `package.ps1` แล้วเทียบเส้นทาง `-SkipBuild`; ผลบน Windows ดูได้จาก run ของคอมมิตนี้บน `main` (container ไม่มี pwsh และ Windows)
- **คู่มือออกรุ่น: ห้าม push ไป `main` จนกว่า job `release` ที่สั่งจากแท็บ Actions จะจบ โดยเฉพาะการแก้ไฟล์ workflow** (`docs/releasing.md` ข้อ 4)
  - เหตุผล: รอบแรกของ Release v1.1.1 (run 39783304886 ที่ `858b9bd`) ล้มด้วย `HTTP 403: Resource not accessible by integration` ตอน `gh release create` เรียก `POST /repos/chatraweechai/doc4me/releases` (13:14:20 UTC) ก่อนหน้านั้น `main` ถูก push `e06edac` ซึ่งแก้ `.github/workflows/windows-x64-installer.yml` (13:11:46 UTC) คอมมิตที่จะติด tag จึงมีไฟล์ workflow ต่างจาก default branch และไม่มี branch หรือ tag ชี้อยู่
  - กลไกนี้ GitHub เขียนไว้ใน [เอกสาร REST หัวข้อ Create a release](https://docs.github.com/en/rest/releases/releases#create-a-release): ถ้าคอมมิตเป้าหมายเพิ่มหรือแก้ไฟล์ใต้ `.github/workflows/` เมื่อเทียบกับ default branch token ต้องมีสิทธิ์แก้ workflow ไม่เช่นนั้นตอบ 404 หรือ `403 Resource not accessible by integration` และ `GITHUB_TOKEN` ได้สิทธิ์นี้ไม่ได้ ส่วน [changelog ของ GitHub 2023-11-02](https://github.blog/changelog/2023-11-02-github-actions-enforcing-workflow-scope-when-creating-a-release/) เขียนเงื่อนไขว่าเป็นคอมมิตที่แก้ไฟล์ workflow และไม่มี branch หรือ tag ชี้อยู่ รอบแรกเข้าทั้งสองแบบ
  - รอบที่สอง (run 39788628618 ที่ `be5893e`) ผ่าน แม้ระหว่างรัน `main` ขยับไป `10a276a` (push 13:42:41 UTC แก้แค่ `handoff.md`) เพราะ `be5893e` ไม่ได้แก้ไฟล์ workflow เอง และไฟล์ workflow ก็ตรงกับ `main` จึงผ่านได้ทั้งสองแบบ การเทียบสองรอบนี้แยกไม่ได้ว่า GitHub ใช้เงื่อนไขแบบไหน คู่มือจึงให้งด push ไป `main` ทั้งหมดจนกว่า job `release` จะจบ
  - คู่มือบอกวิธีแก้ด้วย: สั่ง run ใหม่บน `main` ล่าสุด เพราะรอบที่ล้มไม่ได้สร้าง tag หรือ Release
  - ผู้ใช้: ไม่มีผลต่อโปรแกรม ไฟล์ .docx หรือ installer
  - ตรวจสอบ: log ของ job `release` (119845910393) แสดง `Contents: write` และ `HTTP 403: Resource not accessible by integration (https://api.github.com/repos/chatraweechai/doc4me/releases)`; เวลา push ดูจาก `gh api repos/chatraweechai/doc4me/activity` (`858b9bd->e06edac` 13:11:46, `be5893e->10a276a` 13:42:41); `git diff --stat 858b9bd e06edac` แก้ไฟล์ workflow และ `git diff --stat be5893e 10a276a` แก้แค่ `handoff.md`; เปิดเอกสาร REST และ changelog ของ GitHub ตามลิงก์แล้วตรงกับที่สรุปไว้
  - ตรวจหลัง push (คอมมิต `670f22b`): ฉบับแรกสรุปเองจากการเทียบสองรอบว่าตัวการคือไฟล์ workflow ไม่ใช่การที่ `main` ขยับ และบอกว่า push ไฟล์อื่นได้ ซึ่งการเทียบนั้นยืนยันไม่ได้ จึงแก้ให้อ้างเอกสารของ GitHub และให้งด push ทั้งหมด; เวลา 13:11:49 และ 13:42:43 ที่ใช้ก่อนหน้าเป็นเวลาสร้าง run ไม่ใช่เวลา push

- **xtask: USAGE มีคำสั่ง `version` และ `corpus --download` ตรวจ SHA-256 ของ PngSuite ก่อนแตกไฟล์** (`xtask/src/main.rs`, `xtask/Cargo.toml`)
  - เหตุผล: ข้อความ USAGE ไม่มี `cargo xtask version [set X.Y.Z[-pre]]` ที่ `docs/releasing.md` ใช้ และ PngSuite โหลดผ่าน `http://` โดยไม่ตรวจอะไร (handoff ข้อ 3) ส่วน HTTPS ของเว็บนั้น TLS handshake ล้ม (`tlsv1 alert internal error` เมื่อ 2026-10-24) จึงเปลี่ยนเป็น HTTPS ไม่ได้
  - ไฟล์ที่ hash ไม่ตรงกับ `PNGSUITE_SHA256` (`0294b244…bfdc8` ของไฟล์ 68,521 ไบต์ 178 รายการ โหลดเมื่อ 2026-10-24) ถูกลบและไม่ถูกแตก; ค่า hash ได้จากการโหลดครั้งเดียวและยังไม่ได้เทียบกับแหล่งที่สอง
  - ใช้ `sha2` ที่ workspace มีอยู่แล้ว (`Cargo.lock` ไม่มี crate ใหม่)
  - ผู้ใช้: ไม่มีผลต่อโปรแกรม ไฟล์ .docx หรือ installer (เฉพาะเครื่องมือของนักพัฒนา)
  - ตรวจสอบ: `curl -fsSL http://…/PngSuite-2017jul19.tgz` แล้ว `sha256sum` และ `tar -tzf` ได้ 178 รายการ; clippy ของ `xtask` ดูหมวดการตรวจรอบสองข้างล่าง
- **Dependabot สำหรับ GitHub Actions** (`.github/dependabot.yml` ไฟล์ใหม่)
  - เหตุผล: action ที่ pin SHA ไว้ (`actions/checkout` v4.4.0, `actions/upload-artifact` v4.6.2, `actions/download-artifact` v4.3.0) ยังรันบน Node 20 และ GitHub เตือนใน log (handoff ข้อ 3) Dependabot จะเปิด PR ที่แก้ SHA พร้อมคอมเมนต์เวอร์ชันทุกสัปดาห์ session นี้ไม่ได้อัปเองเพราะสิทธิ์ GitHub ของ session จำกัดอยู่ที่ repo นี้ จึงตรวจ SHA กับ tag ของ repo `actions/*` ไม่ได้
  - workflow ไม่มี trigger `pull_request` PR ของ Dependabot จึงไม่รัน build; ก่อน merge ให้ดู release notes ของ action นั้น และห้าม merge ระหว่างที่ job `release` รันอยู่ (เป็นการแก้ไฟล์ workflow)
  - ผู้ใช้: ไม่มีผลต่อโปรแกรม ไฟล์ .docx หรือ installer
  - ตรวจสอบ: รูปแบบตามเอกสาร Dependabot (`version: 2`, `package-ecosystem: "github-actions"`, `directory: "/"`, `schedule.interval: "weekly"`)

- **เอกสารตรงกับโค้ดหลังรอบ parity 499/499** (`ROADMAP.md`, `docs/gaps.md`, `docs/mcp.md`, `docs/roadmap.md`, `README.md`, `README.th.md`)
  - เหตุผล: ไล่รายการเอกสารล้าสมัยใน handoff ข้อ 2 เทียบกับโค้ดทีละข้อ ส่วนใหญ่แก้ไปแล้วในรอบก่อน ที่ยังผิดอยู่คือ
    - `ROADMAP.md`: ตัวเลขคำสั่ง/เทสต์/บรรทัดยังเป็นของ 2026-10-24 (609 · 1,758 · ~181,000) แก้เป็น 625 คำสั่ง · 1,816 เทสต์ · ~188,000 บรรทัด (2026-10-25); "Last updated" เป็น 2026-10-25; Upcoming ข้อ 1 "~45 open PRs" เป็นของ WordCraft (repo นี้มี PR เปิดอยู่ 4 รายการ เป็น Dependabot ทั้งหมด); เพิ่มแถว revision history 2026-10-24 และ 2026-10-25 ที่ขาดไป
    - `docs/gaps.md` #29 ยังบอกว่าไม่มีสมการด้วยลายมือ แก้เป็นช่องว่างที่เหลือจริง (ตัวจดจำรู้จักสัญลักษณ์ชุดจำกัดตาม `crates/engine/src/ink_math.rs`)
    - `docs/mcp.md` และ alpha gate ใน `docs/roadmap.md`: 618 → 625 คำสั่ง
    - `README.md` และ `README.th.md`: เพิ่มบรรทัด "รุ่นถัดไป" ของพิมพ์ด้วยเสียง (รองรับภาษาไทย ทำงานบนเครื่อง) แปลเอกสาร รูปภาพออนไลน์ สมการด้วยลายมือ และ Get Add-ins
  - ผู้ใช้: ไม่มีผลต่อโปรแกรม ไฟล์ .docx หรือ installer
  - ตรวจสอบ: นับคำสั่งด้วย `grep -rho 'CommandSpec::new("[^"]*"' crates apps | sort -u | wc -l` (625), เทสต์ด้วย `grep -rn '#\[test\]' crates apps xtask` (1,816), บรรทัดของไฟล์ `.rs` ใน `crates`, `apps`, `xtask` (188,030); PR ที่เปิดอยู่จาก GitHub (#1–#4 ของ Dependabot); สัญลักษณ์ของตัวจดจำจากคอมเมนต์ต้นไฟล์และ `templates()` ใน `ink_math.rs`
- **ภาพ `docs/images/review.png` ถ่ายใหม่ให้เห็นปุ่มแปล (Translate) ในแท็บรีวิว** (`docs/images/review.png`)
  - เหตุผล: handoff ข้อ 4 ให้ตรวจความใหม่ของภาพใน `docs/images`; ถ่ายใหม่ทั้ง 11 ภาพจาก `docs/images/shots/*.jsonl` ด้วย `ui_shot` (Vulkan แบบซอฟต์แวร์ `mesa-vulkan-drivers`) มีแค่ `review.png` ที่ต่างจริง (กลุ่มภาษามีปุ่ม Translate); `hero.png` และ `dark.png` ต่างแค่เคอร์เซอร์กะพริบ (412 พิกเซล), `backstage.png` ต่างแค่วันที่ในภาพย่อของแม่แบบจดหมาย (28 พิกเซล) และ `layout.png` 2 พิกเซล จึงคืนค่าเดิมไม่ให้ไฟล์เปลี่ยนโดยไม่มีเหตุ อีก 6 ภาพได้ไฟล์เหมือนเดิมทุกไบต์
  - ผู้ใช้: เฉพาะเอกสารใน repo ไม่มีผลต่อโปรแกรม ไฟล์ .docx หรือ installer
  - ตรวจสอบ: สคริปต์ทั้ง 11 ไฟล์จบด้วยรหัส 0; เทียบพิกเซลกับภาพเดิมด้วย Pillow (`ImageChops.difference`) แล้วเปิดดู `review.png` ว่าแถบคำสั่งและข้อคิดเห็นแสดงถูกต้อง
### แก้ไข

- **`cargo xtask ci` ผ่านทั้ง workspace อีกครั้ง: สคริปต์ถ่ายภาพหน้าจอมีแถวใน `ATTRIBUTION.md`** (`ATTRIBUTION.md`)
  - เหตุผล: `cargo xtask ci` ทั้ง workspace รันครั้งแรกตั้งแต่ 1.0.0 (handoff ข้อ 3) แล้วล้มที่ขั้น `assets` เพราะสคริปต์ `ui_shot` 11 ไฟล์ใน `docs/images/shots/*.jsonl` (เพิ่มเมื่อ 2026-10-21 ใน `d895deb` และ `8804b1f`) อยู่ใต้ `docs/images/` ซึ่งตัวตรวจถือว่าทุกไฟล์เป็น asset แต่ไม่มีแถวใน `ATTRIBUTION.md`; CI บน GitHub ไม่รันขั้นนี้จึงไม่มีใครเห็น
  - แก้: เพิ่มแถวละไฟล์ (ผู้เขียน Doc4Me contributors, MIT OR Apache-2.0, ข้อความล้วนที่เขียนใน repo นี้) แบบเดียวกับภาพหน้าจอ ไม่ผ่อนตัวตรวจ
  - ผู้ใช้: ไม่มีผลต่อโปรแกรม ไฟล์ .docx หรือ installer
  - ตรวจสอบ: `CARGO_INCREMENTAL=0 cargo xtask ci` (หลัง `cargo clean`): fmt ผ่าน, clippy `-D warnings` ผ่าน, test ผ่าน 1,809 ล้ม 0 ignored 0 (55 ชุด), `assets` ล้มด้วยรายชื่อ 11 ไฟล์ข้างต้น; หลังแก้ `cargo xtask assets` → "all 67 asset files attributed", `cargo xtask layers` → "20 crates, no layering violations", `cargo xtask wasm` ผ่านทั้ง 16 crate; `cargo audit` ไม่มีช่องโหว่ เหลือคำเตือน unmaintained 5 รายการ (`rustybuzz`, `ttf-parser` ผ่าน krilla; `zbus_names`, `zvariant`, `zvariant_derive` ผ่าน zbus/accesskit บน Linux) รวม whisper-rs และ cpal ที่เพิ่มในรอบนี้
- เทสต์ engine `failed_command_restores_history_changed_by_nested_commands` ล้มเป็นครั้งคราว: เทียบประวัติ undo ของสองเซสชันที่สร้างห่างกันเล็กน้อย โดยล้าง `date` ของข้อคิดเห็นแต่ไม่ล้าง `date_utc` (เพิ่มในรอบที่ 4) ถ้าสองเซสชันคร่อมวินาทีจะต่างกัน — ล้าง `date_utc` ด้วย (`crates/engine/src/tests.rs`); เฉพาะเทสต์ ไม่กระทบโปรแกรม
- **ข้อความผิดพลาดของ อ่านออกเสียง (Read Aloud) แสดงเป็นภาษาอังกฤษเสมอ** และพบสาเหตุที่เทสต์ audit ภาษา (`untranslated_interface_text`) ล้มเป็นครั้งคราว (งานค้างใน handoff): เมื่อไม่มีโปรแกรมเสียงพูด (เช่น espeak) worker ของ Read Aloud ส่งข้อความ "Read Aloud couldn't start the speech program: …" มาแบบ asynchronous แล้วแถบสถานะแสดงตรงๆ โดยไม่แปล ถ้าข้อความมาถึงก่อนเทสต์ถ่ายเฟรมเสร็จ (เครื่องโหลดหนัก) audit จะล้ม — ทำซ้ำได้ 4 ใน 24 ครั้งเมื่อรัน 8 ชุดพร้อมกัน
  - แก้: `crates/ui-egui/src/read_aloud.rs` แปลข้อความทั้ง 4 แบบของตัวเล่น (`message()`) เหตุผลจากระบบ (เช่น os error) คงไว้ตามที่ระบบให้; ข้อความใหม่ 5 ข้อความแปลครบ 18 ภาษา
  - เทสต์ของ ui-egui ไม่เรียกโปรแกรมเสียงพูดของระบบอีก (`WordApp::new` ใต้ `cfg(test)` ตั้ง backend เป็น `Hold`; เดิม `cfg!(test)` ใน engine ไม่มีผลเมื่อ engine ถูก build เป็น dependency จึงอาจมีเสียงพูดบนเครื่องนักพัฒนาระหว่างรันเทสต์)
  - audit ยกเว้นคำในพจนานุกรมผู้ใช้ (หน้า Options › Proofing แสดงคำที่เทสต์อื่นเพิ่ม เช่น `zqxoptionsword` เพราะพจนานุกรมใช้ร่วมทั้ง process) แล้ว **เอา `#[ignore]` ออก** audit จึงรันทุกครั้งกับ `cargo test -p doc4me-ui-egui` อีกครั้ง
  - ตรวจแล้ว: เทสต์ใหม่ `read_aloud_errors_are_translated`; `cargo test -p doc4me-ui-egui` 332 ผ่าน (รวม audit); รันชุดเทสต์ ui-egui เต็ม 24 ครั้ง (8 ชุดพร้อมกันบนเครื่อง 4 คอร์) ผ่าน 24/24 (ก่อนแก้ audit ล้ม 4/24)
- build เว็บ (wasm32) ไม่ผ่าน `cargo clippy -D warnings`: `crates/fonts/src/fontdb.rs` (`let mut base` ที่บนเว็บไม่ถูกแก้), `crates/engine/src/speech.rs` (ฟิลด์ `running` ที่เว็บไม่มี worker thread อ่าน) และ `apps/doc4me-web/src/web/font_store.rs` (`map_err(JsValue::from)` ที่แปลงเป็นชนิดเดิม) — แก้ด้วย `cfg_attr(target_arch = "wasm32", allow(...))` สองจุดและลบการแปลงที่ไม่จำเป็น ไม่เปลี่ยนพฤติกรรม; ตรวจด้วย `rustup target add wasm32-unknown-unknown` แล้ว `cargo clippy -p doc4me-web --target wasm32-unknown-unknown -- -D warnings` ผ่าน (งานค้างใน handoff: wasm check)
- **เวลา UTC ของข้อคิดเห็นและ tracked change ถูกเขียนกลับลงไฟล์ และอ่าน `w16du:dateUtc` ของ revision** (ผู้ใช้สั่งแก้ข้อจำกัดที่เหลือ)
  - เดิม: ตอนบันทึก .docx ไม่เขียน `commentsIds.xml` และ `commentsExtensible.xml` และไม่อ่าน `w16du:dateUtc` ของ revision Word จึงแสดง `w:date` ซึ่งเป็นเวลาท้องถิ่นของผู้เขียน ผิดเมื่อผู้อ่านอยู่คนละ time zone
  - แก้ (`crates/docx/src/write/mod.rs`): เมื่อมีข้อคิดเห็นที่รู้เวลา UTC จะเขียน `word/commentsIds.xml` (`w16cid:commentId` คู่ `w14:paraId` ของย่อหน้าสุดท้ายกับ durable id `10000000` ขึ้นไป ไม่ซ้ำกันและต่ำกว่า `7FFFFFFF`) และ `word/commentsExtensible.xml` (`w16cex:commentExtensible` กับ `w16cex:dateUtc`) พร้อม relationship และ content type
  - revision: `Revision` มีช่องใหม่ `date_utc` (`crates/doc/src/lib.rs`) ตัวอ่านอ่าน `w16du:dateUtc` (`crates/docx/src/read/mod.rs`) และตัวเขียนเขียนลงทุก tracked change ที่เขียน `w:date` (`w:ins`, `w:del`, `w:moveFrom`, `w:moveTo`, เครื่องหมายย่อหน้า และการเปลี่ยนรูปแบบ, `crates/docx/src/write/story.rs` `change_attrs`); namespace `w16du` อยู่ในตาราง namespace ของ body และใน `mc:Ignorable` (`crates/docx/src/xml.rs`) Word รุ่นที่ไม่รู้จักจึงข้ามไป; revision ใหม่ใน Doc4Me เก็บ `date_utc` เป็น UTC (`cmd/mod.rs`, `moves.rs`, `fmt_revisions.rs`, `tools.rs`)
  - ค่าที่อ่านมาต้องเป็นรูปแบบ xsd:dateTime (`looks_iso`: 16–40 ตัว เป็นตัวเลขกับ `-:TZ.+`) ไม่อย่างนั้นทิ้ง เพราะจะถูกเขียนกลับตามที่อ่าน
  - ข้อเท็จจริงที่ใช้ ตรวจกับ [MS-DOCX](https://learn.microsoft.com/en-us/openspecs/office_standards/ms-docx/d760867e-6bc3-43c7-ac53-b32247e1b7e6): namespace `http://schemas.microsoft.com/office/word/2016/wordml/cid`, `…/2018/wordml/cex` และ `…/2023/wordml/word16du`, และ `dateUtc` เป็นเวลา UTC ต่างจาก `w:date`; relationship `…/2016/09/relationships/commentsIds`, `…/2018/08/relationships/commentsExtensible` และ content type `application/vnd.openxmlformats-officedocument.wordprocessingml.commentsIds+xml` / `…commentsExtensible+xml` จาก [เอกสาร API ของ Paper Instruments](https://docs.paperinstruments.com/docs/docx/api/commentops) ตรงกับ issue ของ [JSv4/Docxodus#1228](https://github.com/JSv4/Docxodus/issues/1228) (ยังไม่ได้เทียบกับไฟล์ที่ Word บันทึกเอง)
  - ผู้ใช้และความเข้ากันได้: ไฟล์ .docx เดิมเปิดได้เหมือนเดิม; ไฟล์ที่บันทึกใหม่มีสอง part นี้เมื่อมีข้อคิดเห็นที่รู้เวลา UTC; Word รุ่นเก่าไม่อ่าน part ที่ไม่รู้จัก
  - ตรวจสอบ: `comments_round_trip` และ `tracked_changes_round_trip` (`crates/docx/tests/roundtrip.rs`) ตั้ง `date_utc` แล้วอ่านกลับได้ค่าเดิม (ข้อคิดเห็นที่ไม่มี UTC ยังว่าง) ผ่าน; `cargo test -p doc4me-docx -p doc4me-layout` ผ่านทั้งหมด
- **ฟิลด์เลขหน้าที่มีสวิตช์รูปแบบของตัวเอง เปลี่ยนตามปุ่มแปลงเลขด้วย และรองรับ `\* ThaiArabic`** (ผู้ใช้สั่งแก้ข้อจำกัดที่เหลือ)
  - เดิม: ฟิลด์ PAGE ที่มี `\* Arabic` ยังแสดงเลขอารบิกแม้เปลี่ยนรูปแบบเลขหน้าของ section แล้ว และ NUMPAGES, SECTIONPAGES, SECTION, SEQ แสดงเลขอารบิกเสมอ; ตัวจัดวางไม่รู้จัก `\* ThaiArabic`
  - แก้: `format_switch` (`crates/layout/src/fields.rs`) รู้จัก `ThaiArabic` (๑๒๓), `ThaiCardText` (คำอ่าน) และ `ThaiLetter` (ก ข ค) ตามรายการ general formatting ของ ECMA-376 §17.16.4.3; ปุ่มแปลงเลข (`swap_field_digits`, `convert_fields` ใน `cmd/format.rs`) สลับ `\* Arabic` ↔ `\* ThaiArabic` ในฟิลด์ PAGE, NUMPAGES, SECTIONPAGES, SECTION และ SEQ ที่ไม่ล็อกของย่อหน้าที่เลือก และในฟิลด์เลขหน้าของหัวและท้ายกระดาษของ section ที่เปลี่ยน; ฟิลด์ที่ไม่มีรูปแบบตัวเลข (ยกเว้น PAGE ซึ่งตามรูปแบบของ section) ได้ `\* ThaiArabic`; ผลที่เก็บไว้ของฟิลด์แบบ simple แปลงเลขด้วย; ฟิลด์ที่มีรูปแบบอื่น (เช่นเลขโรมัน) ไม่เปลี่ยน; กดซ้ำไม่เปลี่ยนเพิ่ม
  - ผู้ใช้และความเข้ากันได้: รหัสฟิลด์ที่บันทึกเป็นแบบที่ Word เขียนเอง เปิดใน Word ได้ตามปกติ
  - ตรวจสอบ: เทสต์ใหม่ `digit_conversion_switches_number_fields` (`crates/engine/src/tests.rs`) และกรณี `ThaiArabic` ใน `crates/layout/src/fields.rs` ผ่าน; ชุดเต็ม engine 383 + 13 + 8, ui-egui 324, docx, layout, pdf, mcp, cli ผ่าน; clippy `-D warnings` ผ่าน; CI [run 40108607965](https://github.com/chatraweechai/doc4me/actions/runs/40108607965) ของ `8757e7d` ผ่าน
- **บานหน้าต่างข้อคิดเห็นแสดงวันที่และเวลาตามเครื่อง และ `w:date` เขียนแบบเดียวกับ Word** (ผู้ใช้สั่ง)
  - อาการเดิม: บานหน้าต่างข้อคิดเห็น (`crates/ui-egui/src/panes.rs`) แสดง 10 ตัวแรกของวันที่ที่เก็บเป็น UTC ข้อคิดเห็นที่เขียนช่วง 00:00–06:59 น. ในไทยจึงขึ้นวันของเมื่อวาน และ `w:date` ที่ Doc4Me เขียนเป็น UTC จึงขึ้นเร็วไป 7 ชั่วโมงเมื่อเปิดใน Word ในไทย
  - ข้อเท็จจริงที่ใช้: Word เขียน `w:date` ของข้อคิดเห็นและ revision เป็นเวลาท้องถิ่นของผู้เขียนแม้ลงท้าย `Z` และ Word รุ่นใหม่เก็บเวลา UTC จริงแยกไว้ใน `w16cex:dateUtc` ของ `commentsExtensible.xml` (โยงผ่าน durable id ใน `commentsIds.xml`) และ `w16du:dateUtc` ของ revision ข้อมูลนี้มาจากรายงานของผู้พัฒนาหลายโครงการ เช่น [CollaboraOnline/online#16271](https://github.com/CollaboraOnline/online/issues/16271) และ [jandira-tech/jubarte-redlines#1118](https://github.com/jandira-tech/jubarte-redlines/issues/1118) ไม่ใช่เอกสารทางการของ Microsoft
  - แก้: `Comment` มีช่องใหม่ `date_utc` (`crates/doc/src/lib.rs`, ว่างเมื่อไม่รู้) ตัวอ่าน .docx อ่าน `w16cex:dateUtc` ผ่าน `commentsIds.xml` (`crates/docx/src/read/mod.rs`, ค่ายาวเกิน 40 ตัวไม่เก็บ) ข้อคิดเห็นใหม่เก็บ `date` เป็นเวลาท้องถิ่นแบบ Word (`now_word_date`) และ `date_utc` เป็น UTC; `review.comments` ส่ง `dateUtc` และ `localDate` เพิ่ม (`comment_local_date`: UTC แปลงเป็นเวลาเครื่องถ้ารู้ ไม่รู้ก็ใช้ `w:date` ตามที่เขียน ซึ่งเป็นสิ่งที่ Word แสดงเช่นกัน) และบานหน้าต่างแสดง `localDate` เป็น `YYYY-MM-DD HH:MM`
  - revision ใหม่ (การแทรก การลบ การย้าย การจัดรูปแบบ และ Compare) ใช้ `w:date` แบบเดียวกัน; ตัวแยกเวลา ISO-8601 ใหม่ `parse_iso` และ `days_from_civil` (`crates/engine/src/cmd/mod.rs`)
  - ผู้ใช้และความเข้ากันได้: ไฟล์ .docx เดิมเปิดได้เหมือนเดิม; ข้อคิดเห็นจาก Word 2019 ขึ้นไปแสดงตามเวลาเครื่องของผู้อ่าน; ข้อคิดเห็นที่ Doc4Me รุ่นก่อนเขียนไว้ (UTC ใน `w:date`) แสดงตามค่าที่เขียนไว้; ไฟล์ .json ของ Doc4Me เก็บ `dateUtc` ด้วย
  - ตรวจสอบ: เทสต์ใหม่ `iso_times_parse_back_to_unix_seconds`, `comments_show_their_utc_time_in_the_local_time_zone` (`cmd/mod.rs`), `comment_utc_times_come_from_comments_extensible`, `without_the_extension_parts_only_the_word_date_is_known` (`crates/docx/tests/comment_dates.rs`) และ `timestamps_come_from_the_clock` ที่แก้ให้ตรวจ `w:date` ท้องถิ่น `dateUtc` และ `localDate` (`crates/engine/src/tests.rs`) ผ่าน; ชุดเต็ม: `cargo test -p doc4me-doc -p doc4me-docx -p doc4me-engine -p doc4me-cli -p doc4me-mcp -p doc4me-layout -p doc4me-pdf` ผ่านทั้งหมด (engine 382 + 13 + 8, layout 224), ui-egui 324 (`--include-ignored`), clippy `-D warnings` `--all-targets` ของ doc, docx, engine, ui-egui ผ่าน, CI [run 40106198767](https://github.com/chatraweechai/doc4me/actions/runs/40106198767) ของ `637bd89` ผ่าน
- **วันที่และเวลาที่แสดงผู้ใช้เป็นเวลาของเครื่อง ไม่ใช่ UTC** (`crates/engine/src/cmd/mod.rs` `local_offset_secs`, `shift_secs`, `now_local`)
  - อาการเดิม: ผู้ใช้ในไทย (UTC+7) เห็นคำทักทายบนหน้า Home ของ backstage เป็น "สวัสดีตอนเช้า" ไปถึง 18:59 น., Insert › Date and Time และฟิลด์ DATE/TIME ได้วันของเมื่อวานช่วง 00:00–06:59 น. และเวลาช้าไป 7 ชั่วโมง, และ "วันนี้" ในตัวเลือกวันที่ของ content control ก็เป็นวันตาม UTC
  - สาเหตุ: `now_iso` และ `iso_from_unix_secs` ให้เวลา UTC และไม่มีโค้ดส่วนไหนอ่าน time zone ของเครื่อง
  - แก้: `format_date` (`cmd/insert.rs`, ใช้กับ Insert › Date and Time, การอัปเดตฟิลด์ DATE/TIME, หน้าปก และเอกสารตัวอย่าง), `today` (`crates/ui-egui/src/content_controls.rs`) และคำทักทาย (`crates/ui-egui/src/backstage.rs`) ใช้ `now_local` ซึ่งเลื่อนเวลา Unix ตาม offset ของเครื่อง: desktop อ่าน time zone ของระบบผ่าน crate `chrono` (Windows ผ่าน API ของระบบ, Linux/macOS ผ่าน `TZ` และ `/etc/localtime`), เว็บอ่าน `Date.getTimezoneOffset` ของเบราว์เซอร์ อ่านไม่ได้ให้ offset 0
  - วันที่ที่เก็บในไฟล์: `core.modified` และรุ่นของเอกสารยังเป็น UTC; `w:date` ของข้อคิดเห็นและ revision เปลี่ยนเป็นเวลาท้องถิ่นแบบที่ Word เขียน ดูรายการ "บานหน้าต่างข้อคิดเห็นแสดงวันที่และเวลาตามเครื่อง" ในหมวดนี้ (ฉบับแรกของรายการนี้บอกว่า `w:date` ยังเป็น UTC เพราะตอนนั้นยังไม่ได้ตรวจ)
  - dependency ใหม่: `chrono` 0.4 (ได้ 0.4.45 พร้อม `iana-time-zone` 0.1.65 และ `iana-time-zone-haiku` 0.1.2 ใน `Cargo.lock`; `default-features = false`, `features = ["clock"]`) เฉพาะ target ที่ไม่ใช่ wasm32 ประกาศใน `[workspace.dependencies]` ของ `Cargo.toml` และ `crates/engine/Cargo.toml` เลือกเพราะ workspace ห้าม `unsafe` (`unsafe_code = "forbid"`) จึงเรียก API ของระบบเองไม่ได้ สัญญาอนุญาต MIT/Apache-2.0
  - ผู้ใช้: ไฟล์ .docx เดิมไม่เปลี่ยน; ฟิลด์ DATE ที่อัปเดตใหม่ได้วันที่ตามเวลาเครื่อง; ติดตั้งทับได้ตามปกติ
  - ตรวจสอบ: เทสต์ใหม่ `local_time_is_the_unix_time_moved_by_the_offset` (`cmd/mod.rs`: 00:30 UTC เป็น 07:30 ที่ UTC+7 และ 20:30 ของวันก่อนที่ UTC−4, การตัดที่ขอบ และ offset ของเครื่องอยู่ระหว่าง UTC−12 ถึง UTC+14) ผ่าน; container ตั้งเป็น UTC จึงยังไม่ได้ลองบนเครื่องที่ตั้ง UTC+7 จริง

- **ข้อมูลใน `[1.1.1]` ที่ล้าสมัย: Release v1.1.1 ออกจากแท็บ Actions ได้แล้ว** (บันทึกเป็นรายการใหม่ตามกติกา ไม่แก้หัวข้อ `[1.1.1]`)
  - หัวข้อ `[1.1.1]` › เปลี่ยนแปลง › ข้อจำกัดที่ทราบ บอกว่าออก Release ผ่าน workflow ไม่ได้และเจ้าของ repo ต้อง push tag เอง ตอนนี้ไม่จริงแล้ว: [Doc4Me v1.1.1](https://github.com/chatraweechai/doc4me/releases/tag/v1.1.1) ออกเมื่อ 2026-10-22 13:51 UTC โดย [run 39788628618](https://github.com/chatraweechai/doc4me/actions/runs/39788628618) (workflow_dispatch `release: true`)
  - tag `v1.1.1` ชี้ `be5893e` มีไฟล์ `doc4me-1.1.1-windows-x64.msi` (32,993,280 ไบต์) และ `doc4me-1.1.1-windows-x64-portable.zip` (43,584,012 ไบต์) บันทึกประจำรุ่นคือหัวข้อ `[1.1.1]` ของ `be5893e`
  - ผู้ใช้: ไม่มีผลต่อโปรแกรม ไฟล์ .docx หรือ installer; บันทึกบนหน้า Release v1.1.1 ยังมีข้อจำกัดที่ทราบเดิม ("จึงไม่มี tag และ Release v1.1.1" และให้เจ้าของ repo push tag เอง) เพราะมาจากหัวข้อ `[1.1.1]` ของ `be5893e` ซึ่งไม่แก้ตามกติกา ให้ถือรายการนี้แทน ดาวน์โหลด MSI และ zip จากหน้านั้นได้ตามปกติ
  - หน้า Release v1.1.1 มี "หมายเหตุหลังออกรุ่น (2026-10-22)" ต่อท้ายบันทึกแล้ว เจ้าของ repo เพิ่มบนหน้า Release เมื่อ 14:32 UTC (ข้อความเดียวกันอยู่ในคอมมิต `8f72a1b`) หมายเหตุบอกว่าข้อจำกัดที่ทราบข้างต้นเป็นผลของรันแรก ([run 39783304886](https://github.com/chatraweechai/doc4me/actions/runs/39783304886)) ซึ่งล้มเพราะระหว่างรันมีการ push การแก้ไฟล์ workflow ขึ้น `main` และ Release ออกสำเร็จจากแท็บ Actions ในรอบที่สอง ([run 39788628618](https://github.com/chatraweechai/doc4me/actions/runs/39788628618)) โดยไม่ต้อง push tag เอง
    - ตรวจสอบ: `get_release_by_tag v1.1.1` (14:39 UTC) ได้ `updated_at` 2026-10-22T14:32:00Z; ดึง body จาก API แล้วตัด CR เทียบกับหัวข้อ `[1.1.1]` ของ `be5893e` ด้วย `diff` ต่างกันแค่บรรทัดว่างแรกกับหมายเหตุที่ต่อท้าย เนื้อหาเดิมไม่เปลี่ยน
  - ตรวจสอบ:
    - job `build` และ `release` ของ run 39788628618 ผ่าน; `get_release_by_tag v1.1.1` ได้ `target_commitish` = `be5893e`, `draft: false`, ไฟล์สองไฟล์สถานะ `uploaded`; `git fetch --tags` แล้ว `git rev-parse v1.1.1^{commit}` ได้ `be5893e`
    - ดาวน์โหลดทั้งสองไฟล์แล้ว ขนาดและ sha256 ตรงกับ digest ของ GitHub (MSI `e4dc153c…`, zip `310c7308…`)
    - zip มี `doc4me.exe`, `doc4me-cli.exe`, ไฟล์สัญญาอนุญาต และ README รวม 12 ไฟล์; `doc4me.exe` มี FileVersion และ ProductVersion 1.1.1
    - MSI มี ProductVersion 1.1.1 และ `UpgradeCode` เดียวกับ 1.1.0 จึงติดตั้งทับได้
    - เนื้อหาบันทึกบนหน้า Release ตรงกับหัวข้อ `[1.1.1]` ของ `be5893e` ทุกไบต์

### ลบ

- **`package-lock.json`** (ไฟล์ที่รากของ repo)
  - คอมมิต `fc7bab5` (2026-10-21, "Initial commit") เพิ่มไฟล์นี้ 6 บรรทัด เป็น lockfile ของ npm ที่ไม่มี package ใดเลย (`"packages": {}`) และไม่มี `package.json` คู่กัน คอมมิตนั้นไม่มีรายการใน CHANGELOG จึงบันทึกไว้ที่นี่
  - เหตุผลที่ลบ: Doc4Me ไม่ใช้ npm (ส่วนเว็บ build ด้วย Cargo) ไฟล์จึงไม่มีผลกับอะไร ผู้ใช้สั่งให้ลบ (2026-10-25)
  - ผู้ใช้: ไม่มีผลต่อโปรแกรม ไฟล์ .docx หรือ installer
  - ตรวจสอบ: `grep` ทั้ง repo ไม่พบไฟล์ใดอ้างถึง `package-lock`; `git show --stat fc7bab5` เพิ่มแค่ไฟล์นี้
- **ปุ่มและรายการ "ปิดกั้นผู้เขียนอื่น" (Review › Protect › Block Authors)** ออกจากริบบอน KeyTip และ catalog (`crates/engine/src/catalog.rs`, `ribbon.rs`, `keytips.rs`): Word เก็บการล็อกนี้บนช่องข้อมูลของเซิร์ฟเวอร์ co-authoring ของ SharePoint ([MS-WORDLFF] ผ่าน MS-FSSHTTP ไม่ได้อยู่ในไฟล์ .docx) ซึ่งต้องใช้ Microsoft 365 แบบเสียเงิน ผู้ใช้สั่งให้ตัด parity ที่ต้องใช้บริการเสียเงิน; ปุ่มเดิมไม่มีคำสั่ง (กดแล้วไม่มีอะไรเกิด) จึงไม่กระทบผู้ใช้; catalog เหลือ 499 รายการ parity 499/499 (100%)

### ความปลอดภัย

- แบบจำลอง 3 มิติ (ตรวจหลัง push `4cb88d6`): ไฟล์ glTF ที่ node อ้างตัวเองเป็นลูกเคยถูกวาดซ้ำได้ถึง 65 ชั้น และกราฟ node ที่วาด mesh ใหญ่ซ้ำหลายครั้ง (หรือ mesh แบบเส้น/จุดที่ไม่เพิ่มสามเหลี่ยม) อาจทำให้อ่านนานมาก (ค้าง) — ตอนนี้ข้าม node ที่เป็นบรรพบุรุษของตัวเอง และจำกัดจำนวน vertex+index ที่อ่านรวม 24 ล้าน (`MAX_WORK` ใน `crates/engine/src/model3d.rs`) เกินแล้วแจ้งว่าโมเดลใหญ่เกินไป; ตรวจด้วยเทสต์ `model3d::tests` (node วนวาดครั้งเดียว, node ที่มีลูกเป็นตัวเอง 4,000 ครั้งจบเร็ว)
- **อัปเดต dependency ที่ `cargo audit` พบช่องโหว่** (handoff ข้อ 3: audit ไม่ได้รันตั้งแต่ 1.0.0)
  - `zip` 3.0.0 → 9.1.0 (`Cargo.toml` workspace): RUSTSEC-2026-0367 (SUID/SGID ตอนแตกไฟล์, high), RUSTSEC-2026-0368 (symlink ออกนอกโฟลเดอร์ปลายทาง) และ RUSTSEC-2026-0369 (สิทธิ์ไฟล์ผ่าน symlink, critical) ทั้งหมดอยู่ใน `ZipArchive::extract` / `ZipStreamReader::extract` ซึ่งโค้ดของ Doc4Me ไม่เรียก (อ่านแต่ละ entry ในหน่วยความจำ) จึงไม่โดนโดยตรง แต่อัปเดตเพื่อปิดช่อง; API ที่เปลี่ยน: `ZipFile::name()` คืน `Result` แล้ว entry ที่ชื่ออ่านไม่ได้ถูกข้ามพร้อมบันทึก log (`crates/docx/src/package.rs`) และเทสต์ที่อ่านชื่อ entry แก้ตาม
  - `ctutils` 0.4.3 → 0.4.5 (RUSTSEC-2026-0385, การเปรียบเทียบที่ไม่เป็น constant-time; มาจาก `sha2`/`hmac` ที่ใช้ในการเข้ารหัส .docx), `tokio` 1.53.2 → 1.54.1 (RUSTSEC-2026-0386 unsound), `clipboard-win` 5.4.1 → 5.4.3 (รุ่นเดิมถูก yank)
  - เหลือคำเตือน unmaintained 5 รายการจาก dependency ทางอ้อม: `rustybuzz` และ `ttf-parser` (ผ่าน `krilla` ของการส่งออก PDF), `zbus_names`, `zvariant`, `zvariant_derive` (ผ่าน `zbus`) ยังไม่มีรุ่นทดแทนที่เปลี่ยนได้โดยไม่เปลี่ยน dependency หลัก
  - ผู้ใช้: ไม่มีผลต่อไฟล์ .docx; การเปิดและบันทึก .docx, .odt, .xlsx (ข้อมูลจดหมายเวียน) ใช้ zip รุ่นใหม่
  - ตรวจสอบ: `cargo audit` หลังอัปเดต: ไม่มี vulnerability เหลือคำเตือน 5 รายการข้างต้น; `cargo test -p doc4me-docx -p doc4me-formats -p doc4me-engine` ผ่านทั้งหมด

- **บันทึกย้อนหลัง: คอมมิต `9954ae0` (2026-10-22, "Initial commit") เพิ่ม `.claude/settings.json`** (`.claude/settings.json`)
  - ไฟล์ 8 บรรทัดนี้อนุญาตให้ Claude Code รัน `git tag -a v*` และ `git push origin v*` โดยไม่ถาม คอมมิตนั้นไม่มีรายการใน CHANGELOG จึงบันทึกไว้ที่นี่ตามที่ผู้ใช้สั่ง (2026-10-25) ไฟล์ถูกลบแล้วใน `e5214c4` (รายการถัดไป)
  - ผู้ใช้: ไม่มีผลต่อโปรแกรม ไฟล์ .docx หรือ installer
  - ตรวจสอบ: `git show 9954ae0` เพิ่มแค่ไฟล์นี้ มีเฉพาะ `permissions.allow` สองรายการข้างต้น
- **ถอนสิทธิ์ push tag แบบไม่ถามของ Claude Code** (`.claude/settings.json`)
  - เจ้าของ repo ลบไฟล์นี้ทั้งไฟล์ในคอมมิต `e5214c4` (2026-10-23) ไฟล์มีแค่ `permissions.allow` สองรายการ คือ `Bash(git tag -a v*)` และ `Bash(git push origin v*)` บันทึกในคอมมิตนี้เพราะคอมมิตนั้นไม่มีรายการใน CHANGELOG
  - เหตุผล: การอนุญาตแบบไม่ถามเปิดทางให้ push tag สาธารณะ (ซึ่งสร้าง Release) โดยไม่มีการยืนยัน ทั้งที่ตอนนี้ออกรุ่นผ่าน workflow_dispatch และ session บนคลาวด์ push tag ไม่ได้อยู่แล้ว (handoff ข้อ 5.4)
  - ผู้ใช้: ไม่มีผลต่อโปรแกรม ไฟล์ .docx หรือ installer
  - ตรวจสอบ: `git show --stat e5214c4` ลบ `.claude/settings.json` 8 บรรทัด และไม่มีไฟล์ใต้ `.claude/` เหลือใน repo
- **secret ของการลงนามโค้ดไม่อยู่ใน environment ระหว่าง `cargo build` ใน CI** ดูรายการ "CI ลงนามโค้ดได้เมื่อมี secret" ในหมวดเปลี่ยนแปลง

### ข้อจำกัดที่ทราบ

- พูดเป็นข้อความ: ข้อความออกมาเป็นช่วง ๆ หลังหยุดพูด (ไม่ใช่ทีละคำแบบ Word); ไม่มีคำสั่งเสียงเช่น "ขึ้นบรรทัดใหม่" หรือเครื่องหมายวรรคตอนอัตโนมัติแบบ Word; ความเร็วขึ้นกับ CPU (build ปิด AVX เพื่อความเข้ากันได้ จึงช้ากว่าที่ CPU ใหม่ทำได้); ต้องใช้เครือข่ายครั้งแรกเพื่อดาวน์โหลดโมเดล; Flatpak ต้อง `flatpak override --user --share=network` ครั้งแรก; ยังไม่ได้ทดสอบกับไมโครโฟนจริงบน Windows/macOS/Linux
- สมการแบบหมึก: ตัวจดจำเป็นแบบแม่แบบ ความแม่นยำต่ำกว่าบริการของ Microsoft มาก รู้จักสัญลักษณ์ชุดจำกัด (ยังไม่มีอักษรกรีกอื่น ตัวอักษรอื่น ลิมิต/ผลรวม เมทริกซ์) และ `x` กับ `×` เป็นสัญลักษณ์เดียวกัน; จึงแสดงข้อความที่จดจำได้ให้ผู้ใช้แก้ก่อนแทรกเสมอ; ทดสอบด้วยเส้นสังเคราะห์ ยังไม่ได้ทดสอบกับลายมือจริงหลายคน
- Add-in: ไม่ใช่ Add-in ของ Office (Office.js) และไม่เชื่อมต่อ Office Store; ทำได้เท่าคำสั่งของ Doc4Me ที่แก้ไขเอกสาร; แค็ตตาล็อกอยู่บนสาขา `main` ของ repo นี้ (ถ้า repo เปลี่ยนเป็น private ร้านค้าจะเปิดไม่ได้)
- แปล: คุณภาพขึ้นกับบริการ (MyMemory ใช้หน่วยความจำการแปลและการแปลด้วยเครื่อง); แปลทั้งเอกสารยาวจะชนโควตาฟรีของ MyMemory เร็ว (ใช้ LibreTranslate ของตัวเองแทนได้); การแปลทั้งเอกสารแปลเฉพาะเนื้อหาหลัก (ไม่รวมหัว/ท้ายกระดาษ เชิงอรรถ กล่องข้อความ) และข้อความในย่อหน้าหนึ่งใช้รูปแบบตัวอักษรตัวแรกทั้งหมด รูปภาพในย่อหน้าย้ายไปต้นย่อหน้า; API key ของ LibreTranslate เก็บแบบข้อความธรรมดาใน preferences
- ฟีเจอร์ออนไลน์ (รูปภาพออนไลน์ และที่จะตามมา) ต้องมี `curl` ของระบบบน desktop (Windows 10 รุ่นก่อน 1803 ไม่มี); รุ่น Flatpak ไม่ได้รับสิทธิ์เครือข่ายโดยตั้งใจ (ดูความเห็นใน manifest) จึงใช้ไม่ได้จนกว่าผู้ใช้สั่ง `flatpak override --user --share=network io.github.chatraweechai.doc4me`; Openverse จำกัดจำนวนครั้งต่อวันสำหรับผู้ไม่ระบุตัวตน
- แบบจำลอง 3 มิติ: แทรกเป็นภาพนิ่งของโมเดล (หมุนหรือเปลี่ยนมุมมองหลังแทรกยังไม่ได้ ต้องแทรกใหม่ด้วย `yaw`/`pitch`) ไม่เขียน `am3d:model3d` ลง .docx จึงเป็นรูปภาพธรรมดาใน Word; ไม่อ่าน texture, `.fbx`, `.3mf`, `.ply` หรือ glTF แบบข้อความ (`.gltf`); โมเดล 3 มิติจากไฟล์ Word เปิดเป็นรูป fallback และเมื่อบันทึกจะเหลือเป็นรูปภาพ
- วิดีโอออนไลน์: Doc4Me เล่นวิดีโอในเบราว์เซอร์ ไม่เล่นในเอกสารแบบ Word และไม่ดึงภาพตัวอย่าง/ชื่อวิดีโอจากเว็บไซต์ (ไม่มีการเชื่อมต่อเครือข่าย) โปสเตอร์จึงเป็นการ์ดปุ่มเล่นที่วาดเอง; ยังไม่มีช่องค้นหาวิดีโอ (Bing/YouTube) ของ Word; การเปิดไฟล์ที่ Doc4Me เขียนใน Word for Windows ยังไม่ได้ทดสอบ (ไม่มี Word ใน container)
- ภาพหน้าจอ: มีเฉพาะการตัดหน้าจอ ยังไม่มีแกลเลอรี "หน้าต่างที่พร้อมใช้งาน" (Available Windows) ของ Word ซึ่งต้องจับภาพหน้าต่างของโปรแกรมอื่น; บน Windows ถ้ากด Esc ใน Snipping Tool โปรแกรมไม่รู้ว่ายกเลิก หน้าต่างจะย่ออยู่จนกว่าผู้ใช้เปิดกลับมา; macOS ต้องอนุญาต Screen Recording ครั้งแรก; Linux แบบ portal บน GNOME จะบันทึกภาพลงโฟลเดอร์ Pictures ของผู้ใช้ด้วย; การทำงานจริงบน Windows/macOS/เดสก์ท็อป Linux ยังไม่ได้ทดสอบด้วยมือ (container ไม่มีหน้าจอ ตรวจด้วย cross-check และเทสต์)
- ข้อคิดเห็นและ revision จาก Word รุ่นก่อน 2019 หรือจาก Doc4Me รุ่นก่อนหน้า ไม่มีเวลา UTC จึงแสดง `w:date` ตามที่เขียนไว้ (เวลาท้องถิ่นของผู้เขียน เหมือนที่ Word แสดง)
- ปุ่มแปลงเลขแปลงเฉพาะเลขอารบิก 0–9 กับเลขไทย ๐–๙ ไม่แปลงเลขของอักษรอื่น (เช่น ٠–٩) เลขที่โปรแกรมสร้างเองเปลี่ยนได้เฉพาะรูปแบบเลขอารบิก (decimal) กับเลขไทย (thaiNumbers); ผลของฟิลด์ที่แปลงเป็นข้อความแล้วจะกลับเป็นค่าเดิมเมื่ออัปเดตฟิลด์

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
