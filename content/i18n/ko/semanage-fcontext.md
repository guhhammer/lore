# semanage fcontext

파일/폴더에 대한 지속적인 SELinux 보안 컨텍스트 규칙 관리.

## Usage
- `sudo semanage fcontext -l` : 모든 파일 레이블링 규칙 나열
- `sudo semanage fcontext -lCn` : 사용자 정의 파일 레이블링 규칙을 헤더 없이 나열
- `sudo semanage fcontext -a -t samba_share_t '/mnt/share(/.*)?'` : PCRE 정규표현식과 일치하는 경로에 레이블을 지정하는 사용자 정의 규칙 추가
- `sudo semanage fcontext -d '/mnt/share(/.*)?'` : PCRE 정규표현식을 사용하여 사용자 정의 규칙 삭제
- `restorecon -Rv 경로/대상/폴더` : 새로운 규칙을 적용하여 폴더를 재귀적으로 다시 레이블링

## Links
- More information: https://manned.org/semanage-fcontext
