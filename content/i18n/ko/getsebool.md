# getsebool

SELinux 부울 값 가져오기.

## Usage
- `getsebool httpd_can_connect_ftp` : 특정 부울의 현재 설정 보기
- `getsebool -a` : 모든 부울의 현재 설정 보기
- `sudo semanage boolean -l` : 설명과 함께 모든 부울의 현재 설정 보기

## Links
- More information: https://manned.org/getsebool
