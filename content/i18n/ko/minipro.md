# minipro

Xgecu 칩 프로그래머 (TL866A/CS, TL866II+, T48, T56) 제어 도구. AVR, PIC, 마이크로컨트롤러 및 메모리 등 다양한 칩 지원.

## Usage
- `minipro -l` : 지원되는 모든 장치 목록 표시
- `minipro -L 장치_이름` : 특정 장치 검색
- `minipro -p 칩_이름 -D` : 칩 ID 읽기
- `minipro -p 칩_이름 -r 경로/대상/출력_파일.bin` : 칩 내용을 파일로 읽기
- `minipro -p 칩_이름 -w 경로/대상/입력_파일.bin` : 파일 내용을 칩에 기록
- `minipro -p 칩_이름 -m 경로/대상/파일.bin` : 칩 내용과 파일 비교
- `minipro -p 칩_이름 -E` : 칩 지우기
- `minipro -h` : 도움말 표시

## Links
- More information: https://manned.org/minipro
