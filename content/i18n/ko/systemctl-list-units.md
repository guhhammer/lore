# systemctl list-units

systemd가 현재 메모리에 로드하고 있는 유닛을 나열.

## Usage
- `systemctl list-units` : 활성 상태이거나, 작업이 대기 중이거나, 실패한 유닛 목록 출력
- `systemctl list-units -a` : 비활성 유닛을 포함한 모든 유닛 출력
- `systemctl list-units -t service|socket|timer|...` : 유닛 타입 기준으로 필터링
- `systemctl list-units --state running|listening|dead|...` : 상태를 기준으로 필터링
- `systemctl list-units 'systemd*'` : 이름 기준으로 필터링
- `systemctl list-units --no-pager` : 페이저 없이 `stdout`으로 출력
- `systemctl list-units --no-legend` : 헤더 및 푸터 없이 출력 (스크립트용)
