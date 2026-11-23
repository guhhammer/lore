# kcadm.sh

관리 작업 수행.

## Usage
- `kcadm.sh config credentials --server 호스트 --realm 영역_이름 --user 사용자명 --password 비밀번호` : 인증된 세션 시작
- `kcadm.sh create users -s username=사용자명 -r 영역_이름` : 사용자 생성
- `kcadm.sh get realms` : 모든 영역 나열
- `kcadm.sh update realms/영역_이름 -f 경로/대상/파일.json` : JSON 구성으로 영역 업데이트

## Links
- More information: https://www.keycloak.org/docs/latest/server_admin/#admin-cli
