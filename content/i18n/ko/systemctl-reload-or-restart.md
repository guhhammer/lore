# systemctl reload-or-restart

`systemd` 유닛들을 재로드 할 수 있다면 다시 불러오고, 그렇지 않으면 다시 시작.

## Usage
- `systemctl reload-or-restart 유닛` : 유닛 재로드 또는 재시작
- `systemctl reload-or-restart 패턴` : 패턴과 일치하는 여러 유닛 재로드 또는 재시작
- `systemctl reload-or-restart 유닛 --no-block` : 작업 완료를 기다리지 않고 실행
- `systemctl reload-or-restart 유닛 --user` : 사용자 유닛에만 적용
