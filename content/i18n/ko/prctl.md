# prctl

실행 중인 프로세스, 작업 및 프로젝트의 리소스 제어를 가져오거나 설정합니다.

## Usage
- `prctl pid` : 프로세스 제한 및 권한 검사
- `prctl -P pid` : 기계적 분석이 가능한 형식으로 프로세스 제한 및 권한 검사
- `prctl -n process.max-file-descriptor pid` : 실행 중인 프로세스의 특정 제한 가져오기

## Links
- More information: https://www.unix.com/man-page/sunos/1/prctl
