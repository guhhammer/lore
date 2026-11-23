# mpremote

MicroPython 장치를 원격으로 제어하는 도구.

## Usage
- `mpremote connect list` : 연결된 모든 MicroPython 장치 목록 표시
- `mpremote connect 장치` : 연결된 장치에 REPL (대화형 Python 쉘) 접속
- `mpremote run 경로/대상/스크립트.py` : 로컬 Python 스크립트를 장치에서 실행
- `mpremote mount 경로/대상/디렉터리` : 로컬 디렉터리를 장치에 마운트
- `mpremote mip install 패키지` : 장치에 mip 패키지 설치

## Links
- More information: https://docs.micropython.org/en/latest/reference/mpremote.html
