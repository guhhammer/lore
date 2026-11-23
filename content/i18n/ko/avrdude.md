# avrdude

Atmel AVR 마이크로 컨트롤러 프로그래밍을 위한 드라이버 프로그램.

## Usage
- `avrdude -p part_id -c programmer -U flash:r:file.hex:i` : AVR 마이크로 컨트롤러 읽기
- `avrdude -p part_id -c programmer -U flash:w:file.hex` : AVR 마이크로 컨트롤러 쓰기
- `avrdude -p \?` : 사용 가능한 AVR 장치 목록
- `avrdude -c \?` : 사용 가능한 AVR 프로그래머 목록
