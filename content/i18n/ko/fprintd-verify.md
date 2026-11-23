# fprintd-verify

데이터베이스에 저장된 지문을 검증합니다.

## Usage
- `fprintd-verify` : 현재 사용자의 모든 저장된 지문 검증
- `fprintd-verify -f left-thumb|left-index-finger|left-middle-finger|left-ring-finger|left-little-finger|right-thumb|right-index-finger|right-middle-finger|right-ring-finger|right-little-finger` : 현재 사용자의 특정 지문 검증
- `fprintd-verify 사용자명` : 특정 사용자의 지문 검증
- `fprintd-verify -f 손가락_이름 사용자명` : 특정 사용자의 특정 지문 검증
- `fprintd-verify --g-fatal-warnings` : 현재 사용자의 데이터베이스에 저장된 지문과 일치하지 않으면 프로세스 실패
- `fprintd-verify -h` : 도움말 표시

## Links
- More information: https://manned.org/fprintd-verify
