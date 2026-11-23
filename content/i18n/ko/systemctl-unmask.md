# systemctl unmask

마스킹된 유닛을 해제하여 다시 시작할 수 있도록 함. `systemctl mask` 효과를 되돌림.

## Usage
- `systemctl unmask 서비스_이름` : 서비스 마스킹 해제
- `systemctl unmask 서비스_이름 --now` : 마스킹 해제 후 즉시 서비스 시작
- `systemctl unmask 서비스_이름 --user` : 사용자 서비스 마스킹 해제
