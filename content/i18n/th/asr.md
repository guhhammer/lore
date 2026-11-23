# asr

กู้คืน (คัดลอก) ดิสก์อิมเมจไปยังวอลุ่ม ชื่อคำสั่งย่อมาจาก Apple Software Restore

## Usage
- `sudo asr restore --source อิมเมจไฟล์.dmg --target ทาง/ไป/วอลุมไฟล์` : กู้คืนอิมเมจดิสก์ไปยังวอลุ่มเป้าหมาย
- `sudo asr restore --source อิมเมจไฟล์.dmg --target ทาง/ไป/วอลุมไฟล์ --erase` : ลบข้อมูลในวอลุ่มเป้าหมายก่อนการกู้คืน
- `sudo asr restore --source อิมเมจไฟล์.dmg --target ทาง/ไป/วอลุมไฟล์ --noverify` : ข้ามขั้นตอนตรวจสอบหลังการกู้คืน
- `sudo asr restore --source ทาง/ไป/วอลุมไฟล์ --target ทาง/ไป/วอลุมไฟล์` : โคลนวอลุ่มโดยไม่ใช้อิมเมจดิสก์ตัวกลาง

## Links
- More information: https://keith.github.io/xcode-man-pages/asr.8.html
