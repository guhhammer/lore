# smbget

SMB 서버에서 파일을 다운로드하기 위한 `wget` 유사 도구.

## Usage
- `smbget smb://server/share/file` : 서버에서 파일 다운로드
- `smbget --recursive smb://server/share` : 공유 또는 폴더를 재귀적으로 다운로드
- `smbget smb://server/share/file -U 사용자명%비밀번호` : 사용자명과 비밀번호로 연결
- `smbget smb://server/share/file -e` : 암호화된 전송 요구

## Links
- More information: https://www.samba.org/samba/docs/current/man-html/smbget.1.html
