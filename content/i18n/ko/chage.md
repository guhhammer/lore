# chage

사용자 계정 및 비밀번호 만료 정보를 변경합니다.

## Usage
- `chage -l 사용자명` : 사용자의 비밀번호 정보를 나열
- `sudo chage -M 10 사용자명` : 비밀번호 만료를 10일 후로 설정
- `sudo chage -M -1 사용자명` : 비밀번호 만료 비활성화
- `sudo chage -E YYYY-MM-DD 사용자명` : 계정 만료 날짜 설정
- `sudo chage -d 0 사용자명` : 다음 로그인 시 비밀번호 변경을 강제

## Links
- More information: https://manned.org/chage
