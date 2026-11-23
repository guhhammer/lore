# xvfb-run

가상 X 서버 환경에서 명령 실행.

## Usage
- `xvfb-run 명령` : 가상 X 서버에서 지정된 명령 실행
- `xvfb-run -a 명령` : 기본값(99)이 사용 불가능한 경우, 사용 가능한 서버 번호를 자동으로 선택
- `xvfb-run -s "-screen 0 1024x768x24" 명령` : xvfb 서버에 인수 전달

## Links
- More information: https://manned.org/xvfb-run
