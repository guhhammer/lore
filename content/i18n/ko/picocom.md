# picocom

직렬 콘솔을 에뮬레이션 하는 최소한의 프로그램.

## Usage
- `sudo picocom /dev/ttyXYZ` : 기본 baud rate 9600으로 직렬 콘솔 연결
- `sudo picocom /dev/ttyXYZ -b baud_rate` : 지정한 baud rate로 직렬 콘솔 연결
- `sudo picocom /dev/ttyXYZ --imap lfcrlf` : 특수문자 매핑 설정 (예: `LF`를 `CRLF`으로 변환)
- `<Ctrl a><Ctrl x>` : picocom 종료
- `picocom -h` : 도움말 표시

## Links
- More information: https://manned.org/picocom
