# auditctl

Linux 감사 시스템의 동작을 제어하고 상태를 확인하며 규칙을 관리하는 유틸리티.

## Usage
- `sudo auditctl -s` : 감사 시스템의 s상태 표시
- `sudo auditctl -l` : 현재 로드된 모든 감사 규칙 l목록
- `sudo auditctl -D` : 모든 감사 규칙 D삭제
- `sudo auditctl -e 1|0` : 감사 시스템 e활성화/비활성화
- `sudo auditctl -a always,exit -F arch=b64 -F path=/경로/대상/파일 -F perm=wa` : 파일 변경 감시
- `sudo auditctl -a always,exit -F arch=b64 -F dir=/경로/대상/폴더/ -F perm=wa` : 디렉토리를 재귀적으로 변경 감시
- `auditctl -h` : h도움말 표시

## Links
- More information: https://manned.org/auditctl
