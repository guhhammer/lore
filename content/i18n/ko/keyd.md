# keyd

키 매핑을 변경.

## Usage
- `systemctl enable keyd --now` : `keyd` 서비스를 활성화하고 시작
- `sudo keyd -m` : 키 입력 정보 표시
- `sudo keyd reload` : 바인딩을 초기화하고 `/etc/keyd`의 설정 파일 다시 불러오기
- `keyd list-keys` : 사용 가능한 모든 키 이름 목록 표시
- `keyd check` : 감지된 설정 파일의 오류 검사
- `sudo keyd bind "누른_키 = 출력_키"` : 임시 키 바인딩 생성

## Links
- More information: https://manned.org/keyd
