# systemctl show

유닛 또는 systemd 자체의 속성을 출력.

## Usage
- `systemctl show` : 시스템 서비스 매니저의 속성 출력
- `systemctl show --user` : 사용자 서비스 매니저의 속성 출력
- `systemctl show unit` : 특정 유닛의 속성 출력
- `systemctl show unit --user` : 특정 사용자 유닛의 속성 출력
- `systemctl show -a` : 빈 속성까지 포함하여 출력
- `systemctl show unit -p Wants,Conflicts,...` : 지정한 속성만 출력
