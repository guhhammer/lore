# rmmod

Linux 커널에서 모듈을 제거합니다.

## Usage
- `sudo rmmod 모듈_이름` : 커널에서 모듈 제거
- `sudo rmmod --verbose 모듈_이름` : 커널에서 모듈을 제거하고 자세한 정보 표시
- `sudo rmmod --syslog 모듈_이름` : 커널에서 모듈을 제거하고 오류를 `stderr` 대신 syslog로 전송
- `rmmod --help` : 도움말 표시
- `rmmod --version` : 버전 표시

## Links
- More information: https://manned.org/rmmod
