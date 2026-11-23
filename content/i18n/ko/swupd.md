# swupd

Clear Linux의 패키지 관리 도구.

## Usage
- `sudo swupd update` : 최신 버전으로 업데이트
- `swupd check-update` : 현재 버전을 표시하고, 새 버전이 있는지 확인
- `swupd bundle-list` : 설치된 번들 나열
- `swupd search -b 패키지` : 원하는 패키지가 존재하는 번들 찾기
- `sudo swupd bundle-add 번들` : 새 번들 설치
- `sudo swupd bundle-remove 번들` : 번들 제거
- `sudo swupd verify` : 손상되었거나 누락된 파일 수정
