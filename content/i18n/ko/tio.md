# tio

직렬 포트를 모니터링하고 상호작용.

## Usage
- `tio /dev/ttyUSB0` : 기본 설정으로 직렬 포트 열기
- `tio -b 9600 /dev/ttyUSB0` : 지정한 baud rate로 직렬 포트 열기
- `tio -L --log-file 로그_파일 /dev/ttyUSB0` : 직렬 포트를 열고 출력을 파일에 기록
- `tio --output-mode hex /dev/ttyUSB0` : 직렬 포트를 열고 16진수 출력 활성화
- `tio -l` : 사용가능한 직렬 포트 목록 표시
- `<Ctrl t><q>` : `tio` 세션 종료

## Links
- More information: https://github.com/tio/tio#3-usage
