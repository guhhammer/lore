# audit2why

audit 로그에서 발생한 SELinux 거부의 원인을 설명하는 도구 . `policycoreutils-python-utils` 패키지의 일부.

## Usage
- `sudo audit2why` : 가장 최근 SELinux 거부 원인 설명
- `sudo audit2why -i 경로/대상/audit.log` : 특정 audit 로그 파일에서 SELinux 거부 원인 분석
- `sudo ausearch -m avc | audit2why` : audit 로그의 모든 SELinux 거부 분석
- `sudo ausearch -m avc -c 서비스_이름 | audit2why` : 특정 서비스에 대한 거부 원인 분석

## Links
- More information: https://manned.org/audit2why
