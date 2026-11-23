# setfacl

파일 접근 제어 목록(ACL) 설정.

## Usage
- `setfacl -m u:사용자명:rw 경로/대상/파일_또는_폴더` : u사용자에게 읽기 및 쓰기 권한으로 파일의 ACL m수정
- `setfacl -d -m u::rw 경로/대상/파일_또는_폴더` : 모든 사용자에 대한 파일의 기본 ACL m수정
- `setfacl -x u:사용자명 경로/대상/파일_또는_폴더` : 파일의 사용자에 대한 ACL 제거
- `setfacl -b 경로/대상/파일_또는_폴더` : 파일의 모든 ACL 항목 제거

## Links
- More information: https://manned.org/setfacl
