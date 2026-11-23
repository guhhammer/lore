# audit2allow

audit 로그를 바탕으로 SELinux 정책의 allow 규칙 생성하는 도구. `policycoreutils-python-utils` 패키지의 일부.

## Usage
- `sudo audit2allow -a` : 최근 audit 거부 로그를 기반으로 allow 규칙 생성 및 출력
- `sudo audit2allow -i 경로/대상/audit.log` : 특정 audit 로그 파일에서 allow 규칙 생성
- `sudo audit2allow -a -M 모듈_이름` : 최근 audit 거부 로그를 기반으로 정책 모듈 생성
- `sudo audit2allow -a --why` : SELinux 거부 원인 설명 (`audit2why`와 동일)
- `sudo audit2allow -a -e` : 생성된 메시지 주변의 상세 정보 출력
- `sudo audit2allow -a -R` : 설치된 매크로를 사용해 reference policy 생성
- `sudo ausearch -m avc -c 서비스_이름 | audit2allow -M 정책_이름` : 특정 서비스에 대한 allow 규칙 생성
- `sudo audit2allow -a -v` : 상세 출력 모드 활성화

## Links
- More information: https://manned.org/audit2allow
