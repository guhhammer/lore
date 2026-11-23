# pdbedit

Samba 사용자 데이터베이스 편집. 간단한 사용자 추가/제거/비밀번호 변경은 `smbpasswd`를 사용할 수도 있습니다.

## Usage
- `sudo pdbedit --list --verbose` : 모든 Samba 사용자 나열 (설정을 보려면 자세히 플래그 사용)
- `sudo pdbedit --user 사용자명 --create` : 기존 Unix 사용자를 Samba에 추가 (비밀번호 입력 요청)
- `sudo pdbedit --user 사용자명 --delete` : samba 사용자 제거
- `sudo pdbedit --user 사용자명 --bad-password-count-reset` : samba 사용자의 실패한 비밀번호 시도 횟수 초기화

## Links
- More information: https://manned.org/pdbedit
