# aa-status

현재 로드된 AppArmor 모듈 나열.

## Usage
- `sudo aa-status` : 상태 확인
- `sudo aa-status --json` : JSON 형태로 상태 표시
- `sudo aa-status --pretty-json` : 정리된 JSON 형태로 상태 표시
- `sudo aa-status --profiled` : 로드된 정책의 수 표시
- `sudo aa-status --enforced` : 로드된 시행 정책의 수 표시
- `sudo aa-status --complaining` : 로드된 비시행 정책의 수 표시
- `sudo aa-status --kill` : 작업을 종료하는 로드된 시행 정책의 수 표시

## Links
- More information: https://gitlab.com/apparmor/apparmor/-/wikis/manpage_aa-status.8
