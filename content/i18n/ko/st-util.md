# st-util

STM32 ARM Cortex 마이크로컨트롤러와 상호작용하기 위해 GDB (GNU Debugger) 서버 실행.

## Usage
- `st-util -p 4500` : 포트 4500에서 GDB 서버 실행
- `target extended-remote localhost:4500` : [대화형] GDB 서버에 연결
- `load firmware.elf` : [대화형] 장치에 펌웨어 쓰기

## Links
- More information: https://github.com/stlink-org/stlink/blob/testing/doc/man/st-util.md
