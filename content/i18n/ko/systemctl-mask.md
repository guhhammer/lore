# systemctl mask

유닛을 `/dev/null`에 연결하여 시작할 수 없도록 완전히 차단.

## Usage
- `systemctl mask 서비스_이름` : 서비스 마스킹
- `systemctl mask 서비스_이름 --now` : 서비스 중지 후 마스킹
- `systemctl mask 서비스_이름 --user` : 사용자 서비스 마스킹
