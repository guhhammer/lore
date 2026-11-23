# rpm

RPM 패키지 관리 도구.

## Usage
- `rpm -q httpd` : httpd 패키지의 버전 표시
- `rpm -qa 'mariadb*'` : 모든 일치하는 패키지의 버전 나열
- `rpm -U 경로/대상/패키지.rpm --force` : 현재 설치된 버전에 상관없이 강제로 패키지 설치
- `rpm -qf /etc/postfix/main.cf` : 파일의 소유자를 식별하고 패키지 버전 표시
- `rpm -ql kernel` : 패키지가 소유한 파일 나열
- `rpm -qp --scripts 패키지.rpm` : RPM 파일의 스크립트릿 표시
- `rpm -Va 'php-*'` : 일치하는 패키지의 변경되었거나 누락되었거나 잘못 설치된 파일 표시
- `rpm -q --changelog 패키지` : 특정 패키지의 변경 로그 표시

## Links
- More information: https://wiki.archlinux.org/title/Pacman/Rosetta
