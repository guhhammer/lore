# salt-key

Salt 마스터에서 Salt 미니언 키를 관리. 루트 사용자로 또는 `sudo`와 함께 Salt 마스터에서 실행해야 함.

## Usage
- `salt-key -L` : 수락된, 수락되지 않은 및 거부된 모든 미니언 키 나열
- `salt-key -a 미니언_id` : 이름으로 미니언 키 수락
- `salt-key -r 미니언_id` : 이름으로 미니언 키 거부
- `salt-key -F` : 모든 공개 키의 지문 출력

## Links
- More information: https://docs.saltproject.io/en/latest/ref/cli/salt-key.html
