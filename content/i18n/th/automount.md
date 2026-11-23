# automount

อ่านไฟล์ `/etc/auto_master` และทำการ mount `autofs` ที่จุดเชื่อมต่อที่กำหนดไว้เพื่อเรียกใช้งานการ mount แบบอัตโนมัติตามต้องการ โดยทั่วไปใช้สำหรับเริ่มกระบวนการ automount ของระบบแบบแมนนวล หมายเหตุ: อาจจะต้องใช้ `sudo` ในการรันหากคุณไม่มีสิทธิ์เพียงพอในการรัน

## Usage
- `automount -cv` : รัน automount โดยล้างแคชล่วงหน้า (`-c`) และแสดงข้อมูลอย่างละเอียด (`-v`) (ใช้บ่อยที่สุด)
- `automount -t 300` : ยกเลิกการเชื่อมต่อโดยอัตโนมัติหลังไม่มีการใช้งาน 5 นาที (300 วินาที)
- `automount -u` : ยกเลิกการเชื่อมต่อทั้งหมดที่เคยถูก mount โดย automount (และใน / หรือที่กำหนดใน) `/etc/auto_master`

## Links
- More information: https://keith.github.io/xcode-man-pages/automount.8.html
