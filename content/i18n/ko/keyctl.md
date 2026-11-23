# keyctl

Linux 커널 키링을 조작합니다.

## Usage
- `keyctl list 대상_키링` : 특정 키링에 있는 키 나열
- `keyctl list @us` : 사용자 기본 세션의 현재 키 나열
- `keyctl add 타입_키링 키_이름 키_값 대상_키링` : 특정 키링에 키 저장
- `echo -n 키_값 | keyctl padd 타입_키링 키_이름 대상_키링` : `stdin`에서 값을 받아 키 저장
- `keyctl timeout 키_이름 초단위_시간_제한` : 키에 시간 제한 설정
- `keyctl read 키_이름` : 키를 읽어 16진 덤프로 형식화 (출력 불가 시)
- `keyctl pipe 키_이름` : 키를 읽어 그대로 형식화
- `keyctl revoke 키_이름` : 키를 취소하고 추가 작업 방지

## Links
- More information: https://manned.org/keyctl
