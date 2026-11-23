# smbmap

SMB 열거 도구.

## Usage
- `smbmap --host-file 경로/대상/파일` : NULL 세션이 활성화되고 공유가 열린 호스트 열거
- `smbmap -u 사용자명 --prompt -H ip_address` : 사용자의 비밀번호 또는 NTLM 해시를 입력하여 호스트의 SMB 공유 및 권한 표시
- `smbmap -u 사용자명 --prompt -H ip_address -x 명령` : 원격 시스템에서 셸 명령 실행
- `smbmap --host-file 경로/대상/파일 -u 사용자_이름 -p 비밀번호 -q` : 호스트를 열거하고 SMB 파일 권한 확인
- `smbmap -u 사용자_이름 -p 비밀번호 -d 도메인 -H ip_또는_호스트_이름` : 사용자 이름과 비밀번호를 사용하여 IP 또는 호스트 이름에 SMB로 연결
- `smbmap --host-file 경로/대상/파일 -u 사용자_이름 -p 비밀번호 -q -R --depth 숫자 --exclude 공유이름 -A 파일패턴` : 파일 이름 패턴(정규 표현식)으로 검색하고 특정 공유를 제외하면서 N 단계 깊이까지 재귀적으로 파일을 찾아 다운로드
- `smbmap -u 사용자_이름 -p 비밀번호 -d 도메인 -H ip_또는_호스트_이름 --upload 경로/대상/파일 '/공유_이름/원격_파일명'` : 사용자 이름과 비밀번호를 사용하여 SMB를 통해 파일 업로드
- `smbmap -u 사용자명 --prompt -H ip_address -R -F 패턴` : SMB 공유를 표시하고 디렉토리 및 파일을 재귀적으로 나열하며, 정규 표현식과 일치하는 파일 내용을 검색

## Links
- More information: https://github.com/ShawnDEvans/smbmap#help
