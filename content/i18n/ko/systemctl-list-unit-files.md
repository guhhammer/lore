# systemctl list-unit-files

설치된 유닛 파일과 활성화 상태를 나열.

## Usage
- `systemctl list-unit-files` : 설치된 유닛 파일과 상태 출력
- `systemctl list-unit-files --state enabled|disabled|static|...` : 상태를 기준으로 필터링
- `systemctl list-unit-files -t service|socket|timer|...` : 유닛 타입 기준으로 필터링
- `systemctl list-unit-files 'sshd*'` : 이름 기준으로 필터링
- `systemctl list-unit-files --no-pager` : 출력을 페이저 없이 `stdout`로 출력
- `systemctl list-unit-files --no-legend` : 헤더 및 푸터 제외 출력
