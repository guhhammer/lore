# cupsctl

서버의 `cupsd.conf` 업데이트 또는 질의.

## Usage
- `cupsctl` : 현재 구성 설정 값을 표시
- `cupsctl -h 서버:포트` : 특정 서버의 구성 값을 표시
- `cupsctl -E` : 스케줄러 연결 시 암호화를 활성화
- `cupsctl --debug-logging|--no-debug-logging` : `error_log` 파일에 대한 디버그 로깅을 활성화 또는 비활성화
- `cupsctl --remote-admin|--no-remote-admin` : 원격 관리 활성화 또는 비활성화
- `cupsctl | grep '^_debug_logging' | awk -F= '{print $2}'` : 현재 디버그 로깅 상태를 구문 분석

## Links
- More information: https://openprinting.github.io/cups/doc/man-cupsctl.html
